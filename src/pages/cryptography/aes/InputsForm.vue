<script setup lang="ts">
import init, {
  encrypt,
  IntermediateValue,
} from "../../../../utils/pkg/slash_utils.js";
import { pinia } from "../../../lib/pinia.ts";
import { toUint32Array, toUint8Array } from "../../../utils/conversions.ts";
import { strip } from "../../../utils/string-formatting.ts";
import { useEnabledTransformsStore } from "./EnabledTransformsStore.ts";
import { useIntermediateValuesStore } from "./IntermediateValuesStore.ts";
import { onBeforeMount, ref } from "vue";
import Container from "../../../components/Container.vue";

const intermediateValuesStore = useIntermediateValuesStore(pinia);
</script>

<template>
  <div class="flex-col items-center flex grow">
    <h3 class="font-bold underline text-3xl pb-2">Inputs</h3>
    <div class="flex flex-wrap content-between justify-center w-full gap-8">
      <Container class="w-full max-w-96">
        <label class="w-full"
          >Block:
          <textarea
            class="block rounded bg-slate-200 text-slate-950 border-slate-700 border w-full px-2"
            v-model="intermediateValuesStore.block"
          />
        </label>
        <label class=""
          >Key:
          <textarea
            type="text"
            class="block rounded bg-slate-200 text-slate-950 border-slate-700 border w-full px-2"
            v-model="intermediateValuesStore.key"
          />
        </label>
        <div class="flex justify-center">
          <button
            class="bg-teal-600 rounded px-4 py-2 font-bold mt-4 text-white"
            @click="intermediateValuesStore.computeIntermediateValues"
          >
            ENCRYPT
          </button>
        </div>
      </Container>
      <Container class="w-full max-w-96">
        <label class=""
          >Encrypted Block:
          <textarea
            class="block rounded bg-slate-200 text-slate-950 border-slate-700 border w-full px-2"
            v-model="intermediateValuesStore.block"
          />
        </label>
        <label class=""
          >Key:
          <textarea
            type="text"
            class="block rounded bg-slate-200 text-slate-950 border-slate-700 border w-full px-2"
            v-model="intermediateValuesStore.key"
          />
        </label>
        <div class="flex justify-center">
          <button
            class="bg-teal-600 rounded px-4 py-2 font-bold mt-4 text-white"
            @click="intermediateValuesStore.computeIntermediateValues"
          >
            DECRYPT
          </button>
        </div>
      </Container>
    </div>
  </div>
</template>
