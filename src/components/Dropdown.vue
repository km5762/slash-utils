<template>
  <select
    class="bg-slate-700 rounded font-semibold px-2 border-slate-600 border block"
    :value="modelValue"
    @change="$emit('update:modelValue', $event?.target?.value)"
  >
    <slot />
  </select>
</template>

<script setup lang="ts">
defineProps({
  modelValue: {
    type: String,
    required: true,
  },
});

defineEmits(["update:modelValue"]);
</script>
