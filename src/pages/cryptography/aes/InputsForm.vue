<script setup lang="ts">
import init, {
  encrypt,
  IntermediateValue,
} from "../../../../utils/pkg/slash_utils.js";
import { pinia } from "../../../lib/pinia.ts";
import { toUint32Array, toUint8Array } from "../../../utils/conversions.ts";
import { useEnabledTransformsStore } from "./EnabledTransformsStore.ts";
import { useIntermediateValuesStore } from "./IntermediateValuesStore.ts";
import { onBeforeMount } from "vue";

const intermediateValuesStore = useIntermediateValuesStore(pinia);
</script>

<template>
  <div class="flex-col items-center flex">
    <h3 class="font-bold underline text-3xl pb-2">Inputs</h3>
    <label
      >Block:
      <input
        type="text"
        class="block rounded bg-slate-200 text-slate-950 border-slate-700 border"
        v-model="intermediateValuesStore.block"
    /></label>
    <label
      >Key:
      <input
        type="text"
        class="block rounded bg-slate-200 text-slate-950 border-slate-700 border"
        v-model="intermediateValuesStore.key"
    /></label>
    <button
      class="bg-teal-600 rounded px-4 py-2 font-bold mt-4"
      @click="intermediateValuesStore.computeIntermediateValues"
    >
      COMPUTE
    </button>
  </div>
</template>
