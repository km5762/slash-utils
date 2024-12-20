<script setup lang="ts">
import ArrowRight from "./icons/ArrowRight.vue";
</script>

<template>
  <a class="hover:underline text-teal-600 text-lg visited:text-blue-900"
    ><slot></slot><ArrowRight class="inline w-4 h-4 ml-2"></ArrowRight
  ></a>
</template>
