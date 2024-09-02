<script setup lang="ts">
import { strip } from "../../../utils/string-formatting.ts";
import { useIntermediateValuesStore, Mode } from "./IntermediateValuesStore.ts";
import { onBeforeMount, ref } from "vue";
import Container from "../../../components/Container.vue";
import TextArea from "@/components/TextArea.vue";
import { pinia } from "@/pinia.ts";
import SubmitButton from "@/components/SubmitButton.vue";
import Label from "@/components/Label.vue";

const intermediateValuesStore = useIntermediateValuesStore(pinia);
</script>

<template>
  <div class="flex-col items-center flex grow">
    <h3 class="font-bold underline text-3xl pb-2">Inputs</h3>
    <div class="flex flex-wrap content-between justify-center w-full gap-8">
      <Container class="w-full max-w-96">
        <Label
          >Block:
          <TextArea
            v-model="intermediateValuesStore.decryptedBlock"
            class="w-full"
          />
        </Label>
        <Label
          >Key:
          <TextArea
            v-model="intermediateValuesStore.encryptionKey"
            class="w-full"
          />
        </Label>
        <div class="flex justify-center">
          <SubmitButton
            class="bg-teal-600 rounded px-4 py-2 font-bold mt-4 text-white"
            @click="
              () => {
                intermediateValuesStore.mode = Mode.ENCRYPTION;
                intermediateValuesStore.computeIntermediateValues();
              }
            "
          >
            ENCRYPT
          </SubmitButton>
        </div>
      </Container>
      <Container class="w-full max-w-96">
        <Label
          >Encrypted Block:
          <textarea
            class="block rounded bg-slate-200 text-slate-950 border-slate-700 border w-full px-2"
            v-model="intermediateValuesStore.encryptedBlock"
          />
        </Label>
        <Label
          >Key:
          <textarea
            type="text"
            class="block rounded bg-slate-200 text-slate-950 border-slate-700 border w-full px-2"
            v-model="intermediateValuesStore.decryptionKey"
          />
        </Label>
        <div class="flex justify-center">
          <SubmitButton
            class="bg-teal-600 rounded px-4 py-2 font-bold mt-4 text-white"
            @click="
              () => {
                intermediateValuesStore.mode = Mode.DECRYPTION;
                intermediateValuesStore.computeIntermediateValues();
              }
            "
          >
            DECRYPT
          </SubmitButton>
        </div>
      </Container>
    </div>
  </div>
</template>
