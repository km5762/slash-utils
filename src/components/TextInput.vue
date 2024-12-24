<script setup lang="ts">
import { ref } from "vue";
import ToolTip from "./ToolTip.vue";

const model = defineModel();

const props = defineProps<{
  filter?: (input: string) => string;
}>();

function filterInput(event: Event) {
  const input = event.target as HTMLInputElement;
  if (input.value && props.filter) {
    model.value = input.value = props.filter(input.value);
  }
}
</script>

<template>
  <input
    class="block w-full border-slate-600 bg-slate-700 border px-3 py-2 h-10 rounded-md"
    v-model="model"
    @input="filterInput"
  />
</template>
