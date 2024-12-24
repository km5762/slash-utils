<script setup lang="ts">
import { computed, onMounted, provide, reactive, ref, type Ref } from "vue";
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
  previous: LinkedNode | null;
  next: LinkedNode | null;
  parent: LinkedNode | null;
  children: LinkedNode[];
};

const selected = defineModel();
provide("selected", selected);

const focused = ref<LinkedNode | null>(null);
function setFocused(node: LinkedNode | null) {
  focused.value = node;
}
provide("focused", { focused, setFocused });

const expanded = reactive<Set<number | string>>(new Set());
function setExpanded(id: number | string, value: boolean) {
  if (value) {
    expanded.add(id);
  } else {
    expanded.delete(id);
  }
}
provide("expanded", { expanded, setExpanded });

const linkedNodes = computed(() => linkNodes(props.nodes));

function linkNodes(nodes: Node[], currentParent?: LinkedNode): LinkedNode[] {
  const linkedNodes: LinkedNode[] = [];

  for (let i = 0; i < nodes.length; i++) {
    const linkedNode: LinkedNode = {
      ...nodes[i],
      previous: i > 0 ? linkedNodes[i - 1] : null,
      next: null,
      parent: currentParent ?? null,
      children: [],
    };

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
    case "ArrowRight": {
      if (focused.value.children?.length) {
        if (expanded.has(focused.value.id)) {
          focused.value = focused.value.children[0];
        } else {
          setExpanded(focused.value.id, true);
        }
      }
      break;
    }
    case "ArrowLeft": {
      if (expanded.has(focused.value.id)) {
        setExpanded(focused.value.id, false);
      } else if (focused.value.parent) {
        focused.value = focused.value.parent;
      }
      break;
    }
    case "ArrowDown": {
      focusNext();
      break;
    }
    case "ArrowUp": {
      focusPrevious();
      break;
    }
    case "Home": {
      focused.value = linkedNodes.value[0];
      break;
    }
    case "End": {
      let end = linkedNodes.value[linkedNodes.value.length - 1];

      while (end.children.length && expanded.has(end.id)) {
        end = end.children[end.children.length - 1];
      }

      focused.value = end;
      break;
    }
  }
}

function focusNext() {
  if (focused.value === null) {
    return;
  }

  let next = null;
  if (
    Array.isArray(focused.value.children) &&
    focused.value.children.length > 0 &&
    expanded.has(focused.value.id)
  ) {
    // If the focused node has children and is expanded
    next = focused.value.children[0];
  } else if (focused.value.next === null && focused.value.parent === null) {
    // If the focused node is the last node in the tree
    next = focused.value;
  } else if (focused.value.next === null) {
    // If there's no next sibling, go up to find the next parent's sibling
    next = focused.value;
    while (next?.parent && next.parent.next === null) {
      next = next.parent;
    }
    next = next?.parent?.next || null;
  } else {
    // Otherwise, move to the next sibling
    next = focused.value.next;
  }

  focused.value = next;
}

function focusPrevious() {
  if (focused.value === null) {
    return;
  }

  let previous = null;

  console.log(focused.value);

  if (focused.value.previous === null && focused.value.parent === null) {
    // If the focused node is the first node in the tree
    previous = focused.value;
  } else if (focused.value.previous === null && focused.value.parent !== null) {
    previous = focused.value.parent;
  } else {
    previous = focused.value.previous;

    while (
      previous?.children &&
      expanded.has(previous.id) &&
      previous.children.length
    ) {
      previous = previous.children[previous.children.length - 1];
    }
  }

  focused.value = previous;
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
