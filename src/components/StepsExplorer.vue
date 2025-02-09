<script setup lang="ts">
import TreeView from "@/components/TreeView.vue";
import { computed, ref, useTemplateRef, watch, type Ref } from "vue";
import Container from "@/components/Container.vue";
import type { LinkedNode, Node } from "@/components/TreeView.vue";
import Typography from "@/components/Typography.vue";
import { getWASMObjectValues } from "@/utils/wasm";
import BaseButton from "./BaseButton.vue";
import ChevronLeft from "./icons/ChevronLeft.vue";
import ChevronRight from "./icons/ChevronRight.vue";
import { useMediaQuery } from "@vueuse/core";
import ChevronDown from "./icons/ChevronDown.vue";

const props = defineProps<{
  steps: Node[];
  disabled?: boolean;
}>();

const selected = defineModel<LinkedNode | null>();
const expanded = ref(false);
const treeView = useTemplateRef("tree-view");

function getUniqueTypes(steps: Node[], types?: Set<number>) {
  types = types ?? new Set();
  for (const step of steps) {
    types?.add(step.kind);
    if (step.children) {
      getUniqueTypes(getWASMObjectValues(step.children), types);
    }
  }
}

const uniqueTypes = computed(() => {
  const types = new Set<number>();
  getUniqueTypes(props.steps, types);
  return types;
});

const isLargeScreen = useMediaQuery("(min-width: 1024px)");

function selectNext() {
  treeView.value?.selectNext();
}

function selectPrevious() {
  treeView.value?.selectPrevious();
}

const isLastStep = computed(
  () => selected.value?.next === null && selected.value?.parent === null
);

const isFirstStep = computed(
  () => selected.value?.previous === null && selected.value?.parent === null
);

watch(selected, () => {
  if (window.innerHeight < 1024) {
    expanded.value = false;
  }
});

defineExpose({ selectNext, selectPrevious });
</script>

<template>
  <div class="flex">
    <TreeView
      ref="tree-view"
      v-if="isLargeScreen"
      :nodes="steps"
      v-model="selected"
      :disabled="disabled"
      class="rounded-r-none text-base flex-shrink-0"
    ></TreeView>
    <Container
      :class="[isLargeScreen && 'border-l-0 rounded-l-none', 'flex-grow']"
      class="steps-explorer-container"
    >
      <div class="flex items-center justify-between mb-4 gap-4">
        <div class="flex w-8 h-8">
          <BaseButton
            v-if="!disabled"
            @click="selectPrevious"
            style="padding: 0"
            :class="[
              isFirstStep && 'hidden',
              'bg-teal-600 flex-grow border-none',
            ]"
            ><ChevronLeft
              class="inline w-5 h-5"
              stroke-width="3"
              stroke-weight="bold"
          /></BaseButton>
        </div>
        <Typography variant="h3" class="text-center" v-if="isLargeScreen">{{
          selected?.title
        }}</Typography>
        <div v-else class="relative flex-grow">
          <div
            :class="[
              disabled && 'opacity-50 pointer-events-none',
              'border border-slate-600 bg-slate-700 rounded-md flex items-center justify-between gap-3 h-10 px-2 w-full cursor-pointer select-none',
            ]"
            @click="disabled ? null : (expanded = !expanded)"
          >
            {{ selected?.title }}
            <button class="px-0 py-0">
              <ChevronDown
                @click="disabled ? null : (expanded = !expanded)"
                class="rotate-180 w-3 h-4"
                stroke-width="3"
                stroke-weight="bold"
              />
            </button>
          </div>
          <TreeView
            v-if="expanded"
            :nodes="steps"
            v-model="selected"
            :disabled="disabled"
            style="font-size: inherit"
            class="absolute font-normal text-base min-w-full"
          />
        </div>
        <div class="flex w-8 h-8">
          <BaseButton
            v-if="!disabled"
            @click="selectNext"
            style="padding: 0"
            :class="[
              isLastStep && 'hidden',
              'bg-teal-600 flex-grow border-none',
            ]"
            ><ChevronRight
              class="inline w-5 h-5"
              stroke-width="3"
              stroke-weight="bold"
          /></BaseButton>
        </div>
      </div>
      <div v-for="type in uniqueTypes">
        <div v-show="type === selected?.kind">
          <slot :name="type.toString()" :step="selected"></slot>
        </div>
      </div>
      <div v-if="disabled === true">
        <div class="flex justify-between items-center w-full">
          <Typography variant="h3" class="text-center w-full">
            No input specified
          </Typography>
        </div>
        <p class="text-center">Enter some values to begin</p>
      </div>
    </Container>
  </div>
</template>

<style scoped>
@media (max-width: 1024px) {
  .steps-explorer-container {
    border: none;
    padding: 0;
    box-shadow: none;
  }
}
</style>
