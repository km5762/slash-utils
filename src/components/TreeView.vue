<script setup lang="ts">
import { computed, onMounted, provide, ref, type Ref } from "vue";
import TreeNode from "./TreeNode.vue";

const props = defineProps<{
  nodes: Node[];
}>();

export type Node = {
  id: number | string;
  title: string;
  children?: Node[];
  [key: string]: unknown;
};

export type LinkedNode = Node & {
  previous?: LinkedNode | null;
  next?: LinkedNode | null;
  parent?: LinkedNode | null;
  children?: LinkedNode[];
};

const selected = defineModel();
provide("selected", selected);

const focused = ref<LinkedNode | null>(null);
function setFocused(node: LinkedNode | null) {
  focused.value = node;
}
provide("focused", { focused, setFocused });

const linkedNodes = computed(() => linkNodes(props.nodes));

function linkNodes(nodes: Node[], currentParent?: LinkedNode) {
  const linkedNodes: LinkedNode[] = [];

  for (let i = 0; i < nodes.length; i++) {
    const linkedNode = { ...nodes[i] };
    linkedNode.previous = i > 0 ? linkedNodes[i - 1] : null;
    linkedNode.parent = currentParent ?? null;
    linkedNode.children = linkNodes(nodes[i].children ?? [], linkedNode);

    if (i > 0) {
      linkedNodes[i - 1].next = linkedNode;
    }
    linkedNodes.push(linkedNode);
  }

  return linkedNodes;
}

onMounted(() => {
  document.addEventListener("keydown", handleKeydown);
});

function handleKeydown(event: KeyboardEvent) {
  if (focused.value === null) {
    return;
  }
  switch (event.key) {
    case "ArrowDown": {
      focused.value =
        focused.value.next ?? focused.value?.parent?.next ?? focused.value;
      break;
    }
    case "ArrowUp": {
      focused.value =
        focused.value.previous ??
        focused.value?.parent?.previous ??
        focused.value;
      break;
    }
  }
}
</script>

<template>
  <ul role="tree" class="text-lg font-bold space-y-1">
    <TreeNode
      v-for="(node, i) in linkedNodes"
      :node="node"
      :tabindex="i === 0 ? 0 : -1"
    ></TreeNode>
  </ul>
</template>
