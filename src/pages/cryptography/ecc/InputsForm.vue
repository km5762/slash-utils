<script setup lang="ts">
import InputLabel from "@/components/InputLabel.vue";
import TextInput from "@/components/TextInput.vue";
import { numeric } from "@/utils/filters";
import { ref } from "vue";

const text = ref("");
</script>

<template>
  <div class="flex-col items-center flex grow content-center w-full">
    <h3 class="font-bold underline text-3xl pb-2">Inputs</h3>
    <form class="flex flex-col gap-2">
      <InputLabel for="a">
        <template #text>a</template>
        <template #tool-tip>The equation of the elliptic curve used</template>
      </InputLabel>
      <TextInput id="a" v-model="text" :filter="numeric" />
    </form>
  </div>
</template>
