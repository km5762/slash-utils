<script setup lang="ts">
import { computed, ref } from "vue";
import ToolTip from "./ToolTip.vue";

const [model, modifiers] = defineModel({
  set(value) {
    if()
  }
});

defineProps<{
  filter?: (input: string) => string;
}>();
</script>

<template>
  <input
    :class="[
      'block rounded bg-slate-200 text-slate-950 border-slate-700 border px-2',
    ]"
    v-model="model"
  />
</template>
