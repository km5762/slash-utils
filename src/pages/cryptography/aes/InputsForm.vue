<script setup>
import init, {
  encrypt,
  IntermediateValues,
} from "../../../../utils/pkg/slash_utils.js";
import { toUint32Array, toUint8Array } from "../../../utils/conversions";
import { store } from "./store";
import { ref } from "vue";

const block = ref("");
const key = ref("");

async function computeIntermediateValues() {
  await init();
  const intermediate_values = encrypt(
    toUint8Array(block.value),
    toUint32Array(key.value)
  );

  store.free = intermediate_values.free;
  store.final_add_round_key = intermediate_values.final_add_round_key;
  store.initial_add_round_key = intermediate_values.initial_add_round_key;
  store.rounds = intermediate_values.rounds;
  store.shift_rows = intermediate_values.shift_rows;
  store.sub_bytes = intermediate_values.sub_bytes;
}
</script>

<template>
  <div class="flex-col items-center flex">
    <h3 class="font-bold underline text-3xl pb-2">Inputs</h3>
    <label
      >Block:
      <input
        type="text"
        class="block rounded bg-slate-200 text-slate-950 border-slate-700 border"
        v-model="block"
    /></label>
    <label
      >Key:
      <input
        type="text"
        class="block rounded bg-slate-200 text-slate-950 border-slate-700 border"
        v-model="key"
    /></label>
    <button
      class="bg-teal-600 rounded px-4 py-2 font-bold mt-4"
      @click="computeIntermediateValues"
    >
      COMPUTE
    </button>
  </div>
</template>
