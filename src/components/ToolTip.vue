<script setup lang="ts">
defineOptions({
  inheritAttrs: false,
});

defineProps<{
  visible: boolean;
}>();
</script>

<template>
  <div class="relative inline-block">
    <slot name="source"></slot>
    <div
      id="tool-tip"
      v-bind="$attrs"
      v-if="visible"
      :class="`absolute min-w-56 bg-inherit rounded-xl p-2 left-[calc(100%+17px)] -top-2`"
    >
      <slot name="text"></slot>
      <div id="arrow" class="bg-inherit"></div>
    </div>
  </div>
</template>

<style>
#arrow {
  position: absolute;
  width: 16px;
  height: 16px;
  clip-path: polygon(0% 50%, 100% 0%, 100% 100%);
  right: 100%;
  top: 13px;
}
</style>
