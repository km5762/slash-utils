<script setup lang="ts">
import TableRow from "../../../components/TableRow.vue";
import TableData from "../../../components/TableData.vue";
import TableHeading from "../../../components/TableHeading.vue";
import { useIntermediateValuesStore } from "./IntermediateValuesStore";
import { useEnabledTransformsStore } from "./EnabledTransformsStore";
import { pinia } from "../../../lib/pinia";
import { onBeforeMount } from "vue";

const intermediateValuesStore = useIntermediateValuesStore(pinia);
</script>

<template>
  <div class="flex-col items-center flex flex-1">
    <h3 class="font-bold underline text-3xl pb-2">Intermediate Values</h3>
    <table class="border-2 border-slate-900 rounded-xl border-separate">
      <TableRow class-name="text-2xl bg-slate-900">
        <TableHeading class-name="w-32 rounded-tl-xl">Step</TableHeading>
        <TableHeading class-name="w-3/5">Value</TableHeading>
        <TableHeading class-name="w-32 rounded-tr-xl">Enabled</TableHeading>
      </TableRow>
      <TableRow v-if="intermediateValuesStore.intermediateValues.length == 0">
        <TableData class-name="rounded-bl-xl"></TableData>
        <TableData></TableData>
        <TableData class-name="rounded-br-xl"></TableData>
      </TableRow>
      <template
        v-for="(
          intermediateValue, step
        ) in intermediateValuesStore.intermediateValues"
      >
        <TableRow>
          <TableHeading
            v-if="step > 0 && (step - 1) % 4 == 0"
            colspan="3"
            class-name="bg-slate-900"
            >Round {{ (step - 1) / 4 + 1 }}</TableHeading
          ></TableRow
        >
        <TableRow>
          <TableData>{{ intermediateValue.transformation }}</TableData>
          <TableData>{{ intermediateValue.value }}</TableData>
          <TableData
            ><input
              type="checkbox"
              class="scale-150"
              checked
              @click="
                (event) => {
                  intermediateValuesStore.setTransform(
                    step,
                    (event.target as HTMLInputElement).checked
                  );
                  intermediateValuesStore.computeIntermediateValues();
                }
              "
          /></TableData>
        </TableRow>
      </template>
    </table>
  </div>
</template>
