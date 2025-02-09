<script setup lang="ts">
import { computed, inject, ref, watch, type Ref, type VNodeRef } from "vue";
import BaseDivider from "./BaseDivider.vue";
import ChevronDown from "./icons/ChevronDown.vue";
import { type LinkedNode, type Node } from "./TreeView.vue";

const props = defineProps<{ node: LinkedNode; disabled?: boolean }>();

const clicked = ref(false);
const li = ref<VNodeRef | null>(null);
const selected = inject<Ref<Node | null>>("selected");
const { focused, setFocused } = inject<{
  focused: Ref<LinkedNode | null>;
  setFocused: (node: LinkedNode | null) => void;
}>("focused")!;
const { expanded, setExpanded } = inject<{
  expanded: Set<string | number>;
  setExpanded: (index: string | number, value: boolean) => void;
}>("expanded")!;

watch(focused, () => {
  if (focused.value?.index === props.node.index) {
    li.value.focus();
  }
});

const isSelected = computed(() => {
  if (selected !== undefined && selected.value !== null) {
    return props.node.index === selected.value.index;
  }
  return false;
});

const isFocused = computed(() => {
  if (focused.value?.index === props.node.index) {
    return true;
  }
  return false;
});

const isExpanded = computed(() => expanded.has(props.node.index));

function handleClick(event: Event) {
  if (props.disabled === true) return;
  event.stopPropagation();
  if (props.node.children?.length) {
    setExpanded(props.node.index, !isExpanded.value);
  }

  if (selected !== undefined && !props.node.children?.length) {
    selected.value = props.node;
  }
}

function handleFocus() {
  setFocused(props.node);
}

function handleBlur() {
  clicked.value = false;
  setFocused(null);
}
</script>

<template>
  <li
    ref="li"
    role="treeitem"
    :aria-expanded="isSelected ? 'true' : 'false'"
    :aria-selected="isSelected ? 'true' : 'false'"
    :aria-disabled="disabled === true ? 'true' : 'false'"
    class="focus-visible:outline-none focus-visible:ring-transparent"
    tabindex="-1"
    @focus="handleFocus"
    @blur="handleBlur"
    @mousedown="clicked = true"
    @keydown.enter="handleClick"
  >
    <span
      :class="[
        'p-1 px-2 w-full h-full inline-block',
        disabled === true
          ? 'text-white/60 pointer-events-none'
          : 'hover-overlay cursor-pointer',
        isSelected ? 'bg-teal-600' : '',
        !clicked && isFocused && !disabled ? 'ring-2 ring-teal-500' : '',
      ]"
      @click="handleClick"
      @mousedown="clicked = true"
    >
      <ChevronDown
        v-if="node.children?.length"
        :class="{ 'rotate-180': isExpanded }"
        class="inline w-4 h-4 transition-transform duration-300"
      />
      {{ node.title }}
    </span>
    <ul v-if="isExpanded" class="ml-8" role="group">
      <TreeNode v-for="child in node.children" :node="child" />
    </ul>
  </li>
</template>
