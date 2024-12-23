<script setup lang="ts">
import { computed, inject, ref, watch, type Ref, type VNodeRef } from "vue";
import BaseDivider from "./BaseDivider.vue";
import ChevronDown from "./icons/ChevronDown.vue";
import { type LinkedNode, type Node } from "./TreeView.vue";

const props = defineProps<{ node: LinkedNode }>();

const clicked = ref(false);
const li = ref<VNodeRef | null>(null);
const selected = inject<Ref<string | number | null>>("selected");
const { focused, setFocused } = inject<{
  focused: Ref<LinkedNode | null>;
  setFocused: (node: LinkedNode | null) => void;
}>("focused")!;
const { expanded, setExpanded } = inject<{
  expanded: Set<string | number>;
  setExpanded: (id: string | number, value: boolean) => void;
}>("expanded")!;

watch(focused, () => {
  if (focused.value?.id === props.node.id) {
    li.value.focus();
  }
});

const isSelected = computed(() => {
  if (selected !== undefined && selected.value !== null) {
    return props.node.id === selected.value;
  }
  return false;
});

const isFocused = computed(() => {
  if (focused.value?.id === props.node.id) {
    return true;
  }
  return false;
});

const isExpanded = computed(() => expanded.has(props.node.id));

function handleClick() {
  if (props.node.children?.length) {
    setExpanded(props.node.id, !isExpanded.value);
  }

  if (selected !== undefined && !props.node.children?.length) {
    selected.value = props.node.id;
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
    class="focus-visible:outline-none focus-visible:ring-transparent"
    tabindex="-1"
    @focus="handleFocus"
    @blur="handleBlur"
    @mousedown="clicked = true"
    @keydown.enter="handleClick"
  >
    <span
      :class="[
        'hover-overlay cursor-pointer p-1 px-2 rounded-md w-full h-full inline-block',
        isSelected ? 'bg-teal-600' : '',
        !clicked && isFocused ? 'ring-2 ring-teal-500' : '',
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
    <ul v-if="isExpanded" class="ml-8 space-y-1 mt-1" role="group">
      <TreeNode v-for="child in node.children" :node="child" />
    </ul>
  </li>
</template>
