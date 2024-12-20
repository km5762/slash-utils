<script setup lang="ts">
const model = defineModel();

defineProps<{
  options: string[];
}>();
</script>

<template>
  <div class="bg-slate-700 p-1 rounded-xl border-slate-600 border">
    <template v-for="(option, index) in options">
      <button
        :class="[
          option === model && 'bg-teal-600',
          index === 0 && 'rounded-l-xl',
          index === options.length - 1 && 'rounded-r-xl',
          'font-bold text-white px-4 py-2 grow hover-overlay',
        ]"
        style="--overlay-opacity: 0.1"
        @click="() => (model = option)"
      >
        {{ option }}
      </button>
    </template>
  </div>
</template>
