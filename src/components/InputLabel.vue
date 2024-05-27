<script setup lang="ts">
import { onMounted, ref, type Ref } from "vue";
import ToolTip from "./ToolTip.vue";
import Info from "./icons/Info.vue";

defineOptions({
  inheritAttrs: false,
});

defineProps({
  text: String,
});

const visible = ref(false);
const toolTip = ref<InstanceType<typeof ToolTip>>();
const button = ref<HTMLButtonElement>();

onMounted(() => {
  document.addEventListener("click", (event) => {
    if (
      visible.value &&
      !toolTip.value?.toolTip?.contains(event.target as Node) &&
      !button.value?.contains(event.target as Node)
    ) {
      visible.value = false;
    }
  });
});
</script>

<template>
  <label v-bind="$attrs">
    <slot name="text"></slot>
    <ToolTip class="bg-slate-700/85" :visible="visible" ref="toolTip">
      <template #source>
        <button
          type="button"
          ref="button"
          @click="
            () => {
              visible = !visible;
            }
          "
        >
          <Info class="size-5" />
        </button>
      </template>
      <template #text>
        <slot name="tool-tip"></slot>
      </template>
    </ToolTip>
  </label>
</template>
