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
    class="block rounded bg-slate-200 text-slate-950 border-slate-700 border px-2"
    @input="filterInput"
    v-model="model"
  />
</template>
