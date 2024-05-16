<script setup>
import TableRow from "../../../components/TableRow.vue";
import TableData from "../../../components/TableData.vue";
import TableHeading from "../../../components/TableHeading.vue";
import { store } from "./store";
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
      <TableRow v-if="!store?.initial_add_round_key">
        <TableData class-name="rounded-bl-xl"></TableData>
        <TableData>{{ store.final_add_round_key }}</TableData>
        <TableData class-name="rounded-br-xl"></TableData>
      </TableRow>
      <TableRow v-if="store?.initial_add_round_key">
        <TableData>Add Round Key</TableData>
        <TableData>{{ store.initial_add_round_key }}</TableData>
        <TableData><input type="checkbox" class="scale-150" /></TableData>
      </TableRow>
      <template v-for="(round, index) in store.rounds">
        <TableRow>
          <TableHeading colspan="3" class-name="bg-slate-900"
            >Round {{ index + 1 }}</TableHeading
          ></TableRow
        >
        <TableRow>
          <TableData>Sub Bytes</TableData>
          <TableData>{{ round.sub_bytes }}</TableData>
          <TableData><input type="checkbox" class="scale-150" /></TableData>
        </TableRow>
        <TableRow>
          <TableData>Shift Rows</TableData>
          <TableData>{{ round.shift_rows }}</TableData>
          <TableData><input type="checkbox" class="scale-150" /></TableData>
        </TableRow>
        <TableRow>
          <TableData>Mix Columns</TableData>
          <TableData>{{ round.mix_columns }}</TableData>
          <TableData><input type="checkbox" class="scale-150" /></TableData>
        </TableRow>
        <TableRow>
          <TableData>Add Round Key</TableData>
          <TableData>{{ round.add_round_key }}</TableData>
          <TableData><input type="checkbox" class="scale-150" /></TableData>
        </TableRow>
      </template>
      <TableRow v-if="store?.final_add_round_key">
        <TableHeading colspan="3" class-name="bg-slate-900"
          >Final Round</TableHeading
        ></TableRow
      >
      <TableRow v-if="store?.sub_bytes">
        <TableData>Sub Bytes</TableData>
        <TableData>{{ store.sub_bytes }}</TableData>
        <TableData><input type="checkbox" class="scale-150" /></TableData>
      </TableRow>
      <TableRow v-if="store?.shift_rows">
        <TableData>Shift Rows</TableData>
        <TableData>{{ store.shift_rows }}</TableData>
        <TableData><input type="checkbox" class="scale-150" /></TableData>
      </TableRow>
      <TableRow v-if="store?.final_add_round_key">
        <TableData class-name="rounded-bl-xl">Add Round Key</TableData>
        <TableData>{{ store.final_add_round_key }}</TableData>
        <TableData class-name="rounded-br-xl"
          ><input type="checkbox" class="scale-150"
        /></TableData>
      </TableRow>
    </table>
  </div>
</template>
