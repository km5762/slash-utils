<script setup lang="ts">
import { onMounted, ref } from "vue";
import ToolTip from "./ToolTip.vue";
import Info from "./icons/Info.vue";
import { v4 as uuidv4 } from "uuid";

defineOptions({
  inheritAttrs: false,
});

defineProps({
  text: String,
});

const visible = ref(false);
</script>

<template>
  <label v-bind="$attrs">
    <slot name="text"></slot>
    <ToolTip class="bg-slate-700/85" :visible="visible">
      <template #source>
        <button
          type="button"
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
