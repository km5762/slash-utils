<script setup lang="ts">
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
  <textarea
    class="block w-full border-slate-600 bg-slate-700 border px-3 py-2 rounded-md"
    @input="filterInput"
    v-model="model"
  />
</template>
