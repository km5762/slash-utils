<script setup lang="ts">
import Typography from "./Typography.vue";
import Dropdown from "./Dropdown.vue";
import Label from "./Label.vue";
import BaseButton from "./BaseButton.vue";
import Container from "./Container.vue";
import TextInput from "./TextInput.vue";
import StepsContainer from "@/pages/cryptography/ecdh/StepsContainer.vue";
import StepsExplorer from "./StepsExplorer.vue";
import { nextTick, onMounted, ref, useTemplateRef, watch } from "vue";
import type { LinkedNode, Node } from "./TreeView.vue";
import LoaderSpinner from "./icons/LoaderSpinner.vue";

const props = defineProps<{
  title: string;
  getPlaceholderSteps?: () => Promise<Node[]>;
  computeSteps: () => Node[];
}>();

const selected = ref<LinkedNode | null>(null);
const steps = ref<Node[]>([]);
const disabled = ref(true);
const loading = ref(true);
const stepsExplorer = useTemplateRef("steps-explorer");

function handleSubmit(event: Event) {
  event.preventDefault();
  steps.value = props.computeSteps();
  disabled.value = false;

  nextTick(() => {
    stepsExplorer.value?.selectNext(); // Ensure it runs after reactivity updates
  });
}

onMounted(async () => {
  if (props.getPlaceholderSteps) {
    steps.value = await props.getPlaceholderSteps();
    loading.value = false;
  }
});

watch(selected, () => {
  if (selected.value) {
    console.log(selected.value);
  }
});
</script>

<template>
  <Typography variant="h1" class="text-center">/{{ title }}</Typography>
  <div class="flex flex-col lg:flex-row mx-auto gap-4 w-full max-w-7xl">
    <Container class="flex-none lg:basis-1/3 lg:self-start">
      <Typography variant="h2">Input</Typography>
      <form @submit="handleSubmit">
        <slot name="input"></slot>
      </form>
    </Container>
    <Container class="flex-grow h-5xl flex flex-col">
      <Typography variant="h2">Steps Explorer</Typography>
      <div class="flex-grow flex items-center justify-center" v-if="loading">
        <LoaderSpinner />
      </div>
      <StepsExplorer
        v-else
        class="flex-grow"
        :steps="steps"
        ref="steps-explorer"
        v-model="selected"
        :disabled="disabled"
      >
        <template v-for="(_, slot) in $slots" #[slot]="selected">
          <slot :name="slot" :step="selected.step"></slot>
        </template>
      </StepsExplorer>
    </Container>
  </div>
</template>
