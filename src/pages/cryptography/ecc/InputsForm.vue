<script setup lang="ts">
import TextInput from "@/components/TextInput.vue";
import SegmentedControl from "@/components/SegmentedControl.vue";
import InfoToolTip from "@/components/InfoToolTip.vue";
import TextArea from "@/components/TextArea.vue";
import { allIntegers, hex } from "@/utils/filters";
import { ref } from "vue";

const text = ref("");
const options = ["Sign", "Verify"];
const selectedOption = ref("Sign");
</script>

<template>
  <div class="flex-col items-center flex grow content-center w-full">
    <h3 class="font-bold underline text-3xl pb-4">Inputs</h3>
    <SegmentedControl
      class="flex w-full pb-4"
      :options="options"
      v-model="selectedOption"
    />
    <form class="flex flex-col gap-2 text-xl">
      <fieldset>
        <legend class="float-left">Curve</legend>
        <InfoToolTip class="inline-block"
          >An equation for an elliptic curve</InfoToolTip
        >
        <math
          xmlns="http://www.w3.org/1998/Math/MathML"
          class="block text-nowrap font-mono"
          ><msup><mi>y</mi><mn>2</mn></msup
          ><mo>=</mo><msup><mi>x</mi><mn>2</mn></msup
          ><mo>+</mo
          ><mtext>
            <label for="a" hidden>a</label>
            <TextInput id="a" v-model="text" :filter="allIntegers" /> </mtext
          ><mi>x</mi><mo>+</mo
          ><mtext>
            <label for="b" hidden>b</label>
            <TextInput id="b" v-model="text" :filter="allIntegers" /></mtext
        ></math>
      </fieldset>
      <fieldset>
        <legend class="float-left">Base Point</legend>
        <InfoToolTip class="inline-block"
          >The base point which generates all other elliptic curve points in the
          subgroup</InfoToolTip
        >
        <div>
          <span>(</span>
          <label for="base-point-x" hidden>Base Point X</label>
          <TextInput
            class="inline"
            id="base-point-x"
            v-model="text"
            :filter="allIntegers"
          />
          <span>,</span>
          <label for="base-point-y" hidden>Base Point Y</label>
          <TextInput
            class="inline"
            id="base-point-y"
            v-model="text"
            :filter="allIntegers"
          />
          <span>)</span>
        </div>
      </fieldset>
      <div>
        <label for="n">Order</label>
        <InfoToolTip class="inline-block"
          >The integer order of the subgroup of elliptic curve
          points</InfoToolTip
        >
        <TextInput id="n" v-model="text" :filter="allIntegers" />
      </div>
      <div v-if="selectedOption === 'Sign'">
        <label for="private-key">Private Key</label>
        <InfoToolTip class="inline-block"
          >The private key of the signer</InfoToolTip
        >
        <TextInput id="private-key" v-model="text" :filter="allIntegers" />
      </div>
      <fieldset v-else>
        <legend class="float-left">Public Key</legend>
        <InfoToolTip class="inline-block"
          >The public key derived from the private key as a point on the
          curve</InfoToolTip
        >
        <div>
          <span>(</span>
          <label for="public-key-x" hidden>Public Key X</label>
          <TextInput
            class="inline"
            id="public-key-x"
            v-model="text"
            :filter="allIntegers"
          />
          <span>,</span>
          <label for="public-key-y" hidden>Public Key Y</label>
          <TextInput
            class="inline"
            id="public-key-y"
            v-model="text"
            :filter="allIntegers"
          />
          <span>)</span>
        </div>
      </fieldset>
      <div>
        <label for="message">Message</label>
        <InfoToolTip class="inline-block">{{
          selectedOption === "Sign"
            ? "The message to sign"
            : "The message that was signed"
        }}</InfoToolTip>
        <TextArea id="message" v-model="text" />
      </div>
      <fieldset v-if="selectedOption === 'Verify'">
        <legend class="float-left">Signature</legend>
        <InfoToolTip class="inline-block"
          >The signature defined by the pair
          <span class="italic">(r,s)</span></InfoToolTip
        >
        <div>
          <label class="italic">
            r:<TextInput class="inline" v-model="text" :filter="allIntegers" />
          </label>
          <label class="italic">
            s:<TextInput class="inline" v-model="text" :filter="allIntegers" />
          </label>
        </div>
      </fieldset>
    </form>
  </div>
</template>
