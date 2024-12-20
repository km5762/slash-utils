<script setup lang="ts">
import ArrowRight from "./icons/ArrowRight.vue";
</script>

<template>
  <a
    class="hover-overlay border border-slate-600 rounded px-4 py-2 font-bold text-white cursor-pointer hover:underline bg-teal-600"
    ><slot></slot><ArrowRight class="inline ml-2 w-4 h-4"></ArrowRight
  ></a>
</template>
