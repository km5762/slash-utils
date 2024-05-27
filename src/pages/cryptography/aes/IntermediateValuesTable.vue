<script setup lang="ts">
import TableRow from "../../../components/TableRow.vue";
import TableData from "../../../components/TableData.vue";
import TableHeading from "../../../components/TableHeading.vue";
import { useIntermediateValuesStore } from "./IntermediateValuesStore";
import { useEnabledTransformsStore } from "./EnabledTransformsStore";
import { onBeforeMount } from "vue";

const intermediateValuesStore = useIntermediateValuesStore();
</script>

<template>
  <div class="flex-col items-center flex">
    <h3 class="font-bold underline text-3xl pb-2 text-center">
      Intermediate Values
    </h3>
    <table
      class="border-2 border-slate-900 rounded-xl border-separate table-fixed w-full"
    >
      <TableRow class="sm:text-2xl text-xl bg-slate-900">
        <TableHeading class="rounded-tl-xl">Step</TableHeading>
        <TableHeading class="">Value</TableHeading>
        <TableHeading class="rounded-tr-xl">Enabled</TableHeading>
      </TableRow>
      <TableRow v-if="intermediateValuesStore.intermediateValues.length == 0">
        <TableData class="rounded-bl-xl"></TableData>
        <TableData></TableData>
        <TableData class="rounded-br-xl"></TableData>
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
            class="bg-slate-900"
            >Round {{ (step - 1) / 4 + 1 }}</TableHeading
          ></TableRow
        >
        <TableRow>
          <TableData
            :class="{
              'rounded-bl-xl':
                step === intermediateValuesStore.intermediateValues.length - 1,
            }"
            >{{ intermediateValue.transformation }}</TableData
          >
          <TableData class="break-words">{{
            intermediateValue.value
          }}</TableData>
          <TableData
            :class="{
              'rounded-br-xl':
                step === intermediateValuesStore.intermediateValues.length - 1,
            }"
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
