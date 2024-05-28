<script setup lang="ts">
import { getCurrentInstance, onMounted, ref } from "vue";

const toolTip = ref<HTMLDivElement>();

defineOptions({
  inheritAttrs: false,
});

defineProps<{
  visible: boolean;
}>();

defineExpose({
  toolTip,
});
</script>

<template>
  <div class="relative inline-block">
    <slot name="source"></slot>
    <div
      ref="toolTip"
      id="tool-tip"
      v-bind="$attrs"
      v-show="visible"
      :class="`absolute min-w-56 bg-inherit rounded-xl p-2 left-[calc(100%+17px)] -top-2 z-50`"
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
