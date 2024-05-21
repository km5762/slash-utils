<script setup>
const model = defineModel();
</script>

<template>
  <textarea
    class="block rounded bg-slate-200 text-slate-950 border-slate-700 border w-full px-2"
    v-model="model"
  />
</template>
