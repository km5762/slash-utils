<script setup lang="ts">
const model = defineModel();

defineProps<{
  options: string[];
}>();
</script>

<template>
  <div>
    <template v-for="(option, index) in options">
      <button
        :class="[
          option === model && 'bg-teal-600',
          index === 0 && 'rounded-l-xl',
          index === options.length - 1 && 'rounded-r-xl',
          'border-teal-600 border-2',
          'font-bold text-white px-4 py-2 grow',
        ]"
        @click="() => (model = option)"
      >
        {{ option }}
      </button>
    </template>
  </div>
</template>
