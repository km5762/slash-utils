<script setup lang="ts">
import { computed, onMounted, provide, reactive, ref, type Ref } from "vue";
import TreeNode from "./TreeNode.vue";

const props = defineProps<{
  nodes: Node[];
  disabled?: boolean;
}>();

defineExpose({ selectNext, selectPrevious });

export type Node = {
  title: string;
  children?: Node[];
  kind: number;
  value: any;
  [key: string]: unknown;
};

export type LinkedNode = Node & {
  index: number;
  previous: LinkedNode | null;
  next: LinkedNode | null;
  parent: LinkedNode | null;
  children: LinkedNode[];
};

const selected = defineModel<LinkedNode | null>();
provide("selected", selected);

const focused = ref<LinkedNode | null>(null);
function setFocused(node: LinkedNode | null) {
  focused.value = node;
}
provide("focused", { focused, setFocused });

const expanded = reactive<Set<number | string>>(new Set());
function setExpanded(index: number | string, value: boolean) {
  if (value) {
    expanded.add(index);
  } else {
    expanded.delete(index);
  }
}
provide("expanded", { expanded, setExpanded });

const linkedNodes = computed(() => linkNodes(props.nodes));

let idCounter = 0; // Global counter for unique IDs

function linkNodes(nodes: Node[], currentParent?: LinkedNode): LinkedNode[] {
  const linkedNodes: LinkedNode[] = [];

  for (let i = 0; i < nodes.length; i++) {
    const linkedNode: LinkedNode = {
      ...nodes[i],
      value: nodes[i].value,
      title: nodes[i].title,
      kind: nodes[i].kind,
      index: idCounter++, // Assign a unique ID and increment the counter
      previous: i > 0 ? linkedNodes[i - 1] : null,
      next: null,
      parent: currentParent ?? null,
      children: [],
    };

    // Recursively link child nodes and assign their `parent`
    linkedNode.children = linkNodes(nodes[i].children ?? [], linkedNode);

    if (i > 0) {
      linkedNodes[i - 1].next = linkedNode; // Set the previous node's `next`
    }

    linkedNodes.push(linkedNode); // Add the current node to the list
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
        if (expanded.has(focused.value.index)) {
          focused.value = focused.value.children[0];
        } else {
          setExpanded(focused.value.index, true);
        }
      }
      break;
    }
    case "ArrowLeft": {
      if (expanded.has(focused.value.index)) {
        setExpanded(focused.value.index, false);
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

      while (end.children.length && expanded.has(end.index)) {
        end = end.children[end.children.length - 1];
      }

      focused.value = end;
      break;
    }
  }
}

function focusNext() {
  focused.value = getNext(focused.value);
}

function focusPrevious() {
  focused.value = getPrevious(focused.value);
}

function selectNext() {
  if (!selected.value) {
    selected.value = linkedNodes.value[0] ?? null;
  } else {
    selected.value = getNext(selected.value ?? null);
  }
}

function selectPrevious() {
  if (!selected.value) {
    selected.value = linkedNodes.value[linkedNodes.value.length - 1] ?? null;
  } else {
    selected.value = getPrevious(selected.value ?? null);
  }
}

function getNext(node: LinkedNode | null): LinkedNode | null {
  if (node === null) {
    return null;
  }

  let next = null;
  if (
    Array.isArray(node.children) &&
    node.children.length > 0 &&
    expanded.has(node.index)
  ) {
    // If the focused node has children and is expanded
    next = node.children[0];
  } else if (node.next === null && node.parent === null) {
    // If the focused node is the last node in the tree
    next = node;
  } else if (node.next === null) {
    // If there's no next sibling, go up to find the next parent's sibling
    next = node;
    while (next?.parent && next.parent.next === null) {
      next = next.parent;
    }
    next = next?.parent?.next || null;
  } else {
    // Otherwise, move to the next sibling
    next = node.next;
  }

  return next;
}

function getPrevious(node: LinkedNode | null): LinkedNode | null {
  if (node === null) {
    return null;
  }

  let previous = null;

  if (node.previous === null && node.parent === null) {
    // If the focused node is the first node in the tree
    previous = node;
  } else if (node.previous === null && node.parent !== null) {
    previous = node.parent;
  } else {
    previous = node.previous;

    while (
      previous?.children &&
      expanded.has(previous.index) &&
      previous.children.length
    ) {
      previous = previous.children[previous.children.length - 1];
    }
  }

  return previous;
}
</script>

<template>
  <ul
    role="tree"
    class="text-lg font-bold bg-slate-700 border border-slate-600 rounded-md"
    :aria-disabled="props.disabled === true ? 'true' : 'false'"
  >
    <TreeNode
      v-for="(node, i) in linkedNodes"
      :disabled="disabled"
      :node="node"
      :tabindex="i === 0 && props.disabled !== true ? 0 : -1"
    ></TreeNode>
  </ul>
</template>
