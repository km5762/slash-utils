<script setup lang="ts">
type Variant = "h1" | "h2" | "h3" | "h4" | "h5" | "h6";

defineProps<{ variant: Variant }>();

const variantClasses: Record<Variant, string> = {
  h1: "font-bold text-4xl md:text-5xl lg:text-6xl/none mb-6",
  h2: "font-bold text-3xl md:text-4xl lg:text-5xl mb-5",
  h3: "font-semibold text-2xl md:text-3xl lg:text-4xl mb-4",
  h4: "font-semibold text-xl md:text-2xl lg:text-3xl mb-3",
  h5: "text-lg text-lg md:text-xl lg:text-2xl font-medium mb-2",
  h6: "text-base md:text-lg lg:text-xl font-medium mb-1",
};
</script>

<template>
  <component :is="variant" :class="variantClasses[variant]">
    <slot></slot>
  </component>
</template>
