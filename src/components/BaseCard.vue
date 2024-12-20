<script setup lang="ts">
import Container from "./Container.vue";

defineProps<{ title: string; subtitle: string }>();
</script>

<template>
  <Container
    ><div class="space-y-1.5 p-6 flex flex-row items-center gap-4">
      <slot name="icon"></slot>
      <h3 class="text-2xl font-semibold leading-none tracking-tight">
        {{ title }}
      </h3>
    </div>
    <div class="p-6 pt-0">
      <p class="text-white/65 mb-4">
        {{ subtitle }}
      </p>
      <slot></slot></div
  ></Container>
</template>
