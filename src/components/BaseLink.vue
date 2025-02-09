<script setup lang="ts">
import type { IntrinsicElementAttributes } from "vue";
import ArrowRight from "./icons/ArrowRight.vue";
defineProps<{
  button?: boolean;
  arrow?: boolean;
  nav?: boolean;
}>();
</script>

<template>
  <a
    :class="[
      'hover:underline cursor-pointer',
      button && 'hover-overlay rounded px-4 py-2 font-bold bg-teal-600',
      !nav && !button && 'text-teal-500',
      $attrs.class,
    ]"
    ><slot></slot
    ><ArrowRight v-if="arrow" class="inline w-4 h-4 ml-2"></ArrowRight
  ></a>
</template>
