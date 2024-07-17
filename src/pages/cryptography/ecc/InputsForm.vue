<script setup lang="ts">
import TextInput from "@/components/TextInput.vue";
import SegmentedControl from "@/components/SegmentedControl.vue";
import SubmitButton from "@/components/SubmitButton.vue";
import InfoToolTip from "@/components/InfoToolTip.vue";
import TextArea from "@/components/TextArea.vue";
import Dropdown from "@/components/Dropdown.vue";
import { allIntegers, hex } from "@/utils/filters";
import { computed, reactive, ref, watch } from "vue";
import {
  P256,
  P384,
  P521,
  type SigningAlgorithmType,
  type SigningAlgorithmConfig,
} from "./signing-algorithms";
import {
  EcdsaCustom,
  EcdsaP256,
  EcdsaP384,
  EcdsaP521,
} from "@utils/cryptography/ecc/pkg/ecc";
import { HexString, HexStringObject } from "@/utils/hex-string";
import { useIntermediateValuesStore } from "./IntermediateValuesStore";
import { pinia } from "@/pinia";
import type { Result } from "@/utils/error-types";

type HashAlgorithmType = "SHA-1" | "SHA-256" | "SHA-384" | "SHA-512" | "none";

const intermediateValuesStore = useIntermediateValuesStore(pinia);
const signingAlgorithmType = ref<SigningAlgorithmType>("custom");
const hashAlgorithmType = ref<HashAlgorithmType>("none");
const signingAlgorithmConfig = reactive<SigningAlgorithmConfig>({
  p: "",
  a: "",
  b: "",
  gx: "",
  gy: "",
  n: "",
});
const privateKey = ref("");
const k = ref("");
const m = ref("");
const options = ["Sign", "Verify"];
const selectedOption = ref("Sign");

const signingAlgorithmSelected = computed(
  () => signingAlgorithmType.value !== "custom"
);
const hashAlgorithmSelected = computed(
  () => hashAlgorithmType.value !== "none"
);
const maxLength = computed(() => {
  switch (signingAlgorithmType.value) {
    case "P-256":
      return 64;
    case "P-384":
      return 96;
    case "P-521":
      return 160;
    case "custom":
      return 160;
  }
});

const handleSigningAlgorithmChange = (event: Event) => {
  const selectedValue = (event.target as HTMLSelectElement).value;
  const selectedSigningAlgorithmType = selectedValue as SigningAlgorithmType;
  signingAlgorithmType.value = selectedSigningAlgorithmType;

  switch (selectedSigningAlgorithmType) {
    case "P-256":
      Object.assign(signingAlgorithmConfig, P256);
      break;
    case "P-384":
      Object.assign(signingAlgorithmConfig, P384);
      break;
    case "P-521":
      Object.assign(signingAlgorithmConfig, P521);
      break;
    default:
      Object.assign(signingAlgorithmConfig, {
        p: "",
        a: "",
        b: "",
        gx: "",
        gy: "",
        n: "",
      });
  }
};

const handleHashAlgorithmChange = (event: Event) => {
  const selectedValue = (event.target as HTMLSelectElement).value;
  hashAlgorithmType.value = selectedValue as HashAlgorithmType;
  m.value = "";
};

async function computeSignature() {
  let signingAlgorithm;
  let bytes;
  switch (signingAlgorithmType.value) {
    case "P-256":
      bytes = 32;
      signingAlgorithm = EcdsaP256.new();
      break;
    case "P-384":
      bytes = 48;
      signingAlgorithm = EcdsaP384.new();
      break;
    case "P-521":
      bytes = 80;
      signingAlgorithm = EcdsaP521.new();
      break;
    case "custom":
      bytes = 80;
      const config = new HexStringObject(signingAlgorithmConfig).toByteObject(
        bytes
      );
      const { p, a, b, gx, gy, n } = config;
      signingAlgorithm = EcdsaCustom.new(p, a, b, gx, gy, n);
      break;
  }

  let e = m.value;
  if (hashAlgorithmSelected.value) {
    const encoder = new TextEncoder();
    const data = encoder.encode(m.value);
    const hash = await crypto.subtle.digest(hashAlgorithmType.value, data);
    e = HexString.fromBytes(new Uint8Array(hash)).string;
  }

  intermediateValuesStore.e = e;

  const z = e.substring(0, bytes * 2);
  intermediateValuesStore.z = z;

  let kBytes = new HexString(k.value).toBytes(bytes);
  let privateKeyBytes = new HexString(privateKey.value).toBytes(bytes);
  let zBytes = new HexString(z).toBytes(bytes);

  const signature = signingAlgorithm.sign(kBytes, privateKeyBytes, zBytes);

  intermediateValuesStore.r = HexString.fromBytes(signature.r).string;
  intermediateValuesStore.s = HexString.fromBytes(signature.s).string;
}
</script>

<template>
  <div class="flex-col items-center flex grow content-center w-full">
    <h3 class="font-bold underline text-3xl pb-4">Inputs</h3>
    <SegmentedControl
      class="flex w-full pb-4"
      :options="options"
      v-model="selectedOption"
    />
    <div class="flex gap-12">
      <div>
        <label for="config">Signing Algorithm</label>
        <Dropdown
          @change="handleSigningAlgorithmChange"
          name="signing-algorithm"
          class="ml-2"
          id="config"
        >
          <option selected value="custom">Custom</option>
          <option value="P-256">NIST P-256</option>
          <option value="P-384">NIST P-384</option>
          <option value="P-521">NIST P-521</option>
        </Dropdown>
      </div>
      <div>
        <label for="hash">Hash Algorithm</label>
        <Dropdown
          @change="handleHashAlgorithmChange"
          name="hash"
          class="ml-2"
          id="hash"
        >
          <option selected value="none">None</option>
          <option value="SHA-1">SHA-1</option>
          <option value="SHA-224">SHA-224</option>
          <option value="SHA-256">SHA-256</option>
          <option value="SHA-384">SHA-384</option>
          <option value="SHA-512">SHA-512</option>
        </Dropdown>
      </div>
    </div>
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
            <TextInput
              :disabled="signingAlgorithmSelected"
              id="a"
              v-model="signingAlgorithmConfig.a"
              :filter="hex"
              :maxLength
            /> </mtext
          ><mi>x</mi><mo>+</mo
          ><mtext>
            <label for="b" hidden>b</label>
            <TextInput
              :disabled="signingAlgorithmSelected"
              id="b"
              v-model="signingAlgorithmConfig.b"
              :filter="hex"
              :maxLength
          /></mtext>
        </math>
      </fieldset>
      <div>
        <label for="p">Modulus(p)</label>
        <InfoToolTip class="inline-block"
          >The integer order of the subgroup of elliptic curve
          points</InfoToolTip
        >
        <TextInput
          :disabled="signingAlgorithmSelected"
          id="p"
          v-model="signingAlgorithmConfig.p"
          :filter="hex"
          :maxLength
        />
      </div>
      <fieldset>
        <legend class="float-left">Base Point(G)</legend>
        <InfoToolTip class="inline-block"
          >The base point which generates all other elliptic curve points in the
          subgroup</InfoToolTip
        >
        <div>
          <span>(</span>
          <label for="base-point-x" hidden>Base Point X</label>
          <TextInput
            :disabled="signingAlgorithmSelected"
            class="inline"
            id="base-point-x"
            v-model="signingAlgorithmConfig.gx"
            :filter="hex"
            :maxLength
          />
          <span>,</span>
          <label for="base-point-y" hidden>Base Point Y</label>
          <TextInput
            :disabled="signingAlgorithmSelected"
            class="inline"
            id="base-point-y"
            v-model="signingAlgorithmConfig.gy"
            :filter="hex"
            :maxLength
          />
          <span>)</span>
        </div>
      </fieldset>
      <div>
        <label for="n">Order(n)</label>
        <InfoToolTip class="inline-block"
          >The integer order of the subgroup of elliptic curve
          points</InfoToolTip
        >
        <TextInput
          :disabled="signingAlgorithmSelected"
          id="n"
          v-model="signingAlgorithmConfig.n"
          :filter="hex"
          :maxLength
        />
      </div>
      <div v-if="selectedOption === 'Sign'">
        <label for="private-key">Private Key(d<sub>a</sub>)</label>
        <InfoToolTip class="inline-block"
          >The private key of the signer</InfoToolTip
        >
        <TextInput
          id="private-key"
          v-model="privateKey"
          :filter="hex"
          :maxLength
        />
      </div>
      <div v-if="selectedOption === 'Sign'">
        <label for="k">Random Seed(k)</label>
        <InfoToolTip class="inline-block"
          >The private key of the signer</InfoToolTip
        >
        <TextInput id="k" v-model="k" :filter="hex" :maxLength />
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
            :filter="hex"
            :maxLength
          />
          <span>,</span>
          <label for="public-key-y" hidden>Public Key Y</label>
          <TextInput
            class="inline"
            id="public-key-y"
            :filter="hex"
            :maxLength
          />
          <span>)</span>
        </div>
      </fieldset>
      <div>
        <label for="message">Message(m)</label>
        <InfoToolTip class="inline-block">{{
          selectedOption === "Sign"
            ? "The message to sign"
            : "The message that was signed"
        }}</InfoToolTip>
        <TextArea
          id="message"
          v-model="m"
          :filter="hashAlgorithmSelected ? undefined : hex"
        />
      </div>
      <fieldset v-if="selectedOption === 'Verify'">
        <legend class="float-left">Signature</legend>
        <InfoToolTip class="inline-block"
          >The signature defined by the pair
          <span class="italic">(r,s)</span></InfoToolTip
        >
        <div>
          <label class="italic">
            r:<TextInput class="inline" :filter="hex" />
          </label>
          <label class="italic">
            s:<TextInput class="inline" :filter="hex" />
          </label>
        </div>
      </fieldset>
      <div>
        <SubmitButton type="button" @click="computeSignature">{{
          selectedOption.toUpperCase()
        }}</SubmitButton>
      </div>
    </form>
  </div>
</template>
