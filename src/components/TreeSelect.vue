<script setup lang="ts">
defineProps<{ disabled: boolean }>();
</script>

<template>
  <div class="relative flex-grow">
    <div
      :class="[
        disabled && 'opacity-50 pointer-events-none',
        'border border-slate-600 bg-slate-700 rounded-md flex items-center justify-between gap-3 h-10 px-2 w-full',
      ]"
    >
      {{ selected?.title }}
      <button class="px-0 py-0">
        <ChevronDown
          class="rotate-180 w-3 h-4"
          stroke-width="3"
          stroke-weight="bold"
        />
      </button>
    </div>
    <TreeView
      :nodes="steps"
      v-model="selected"
      :disabled="disabled"
      class="absolute font-normal text-base min-w-full"
    />
  </div>
</template>
