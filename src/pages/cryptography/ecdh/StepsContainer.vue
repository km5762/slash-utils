<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import StepsExplorer from "../../../components/StepsExplorer.vue";
import type { Node } from "@/components/TreeView.vue";
import { Steps, StepKind } from "@utils/cryptography/ecdh/pkg/ecdh";
import { getWASMObjectProperties, getWASMObjectValues } from "@/utils/wasm";
import Typography from "@/components/Typography.vue";
import Container from "@/components/Container.vue";
import Dropdown from "@/components/Dropdown.vue";
import Label from "@/components/Label.vue";
import BaseButton from "@/components/BaseButton.vue";
import TextInput from "@/components/TextInput.vue";
import UtilModule from "@/components/UtilModule.vue";
import BaseOption from "@/components/BaseOption.vue";
import {
  CurveType,
  compute_shared_secret,
} from "@utils/cryptography/ecdh/pkg/ecdh";
import init from "@utils/cryptography/ecdh/pkg/ecdh";

const alicePrivateKey = ref("");
const bobPrivateKey = ref("");
const curve = ref<"NIST-P256" | "NIST-P384" | "NIST-P521">("NIST-P256");
const maxLength = computed(() => {
  switch (curve.value) {
    case "NIST-P256":
      return 64;
    case "NIST-P384":
      return 96;
    case "NIST-P521":
      return 132;
  }
});

function computeSteps() {
  let curveType;
  switch (curve?.value) {
    case "NIST-P256":
      curveType = CurveType.P256;
      break;
    case "NIST-P384":
      curveType = CurveType.P384;
      break;
    case "NIST-P521":
      curveType = CurveType.P521;
      break;
  }

  const steps = compute_shared_secret(
    alicePrivateKey.value,
    bobPrivateKey.value,
    curveType
  );

  return getWASMObjectValues(steps);
}

async function getPlaceholderSteps() {
  await init();
  return getWASMObjectValues(new Steps());
}
</script>

<template>
  <UtilModule
    title="ECDH"
    :get-placeholder-steps="getPlaceholderSteps"
    :compute-steps="computeSteps"
  >
    <template #input>
      <div class="space-y-4">
        <div>
          <Label for="curve">Curve</Label>
          <Dropdown v-model="curve" id="curve">
            <BaseOption value="NIST-P256">NIST-P256</BaseOption>
            <BaseOption value="NIST-P384">NIST-P384</BaseOption>
            <BaseOption value="NIST-P521">NIST-P521</BaseOption>
          </Dropdown>
        </div>
        <div>
          <Label for="alice-private-key">Alice Private Key</Label>
          <TextInput
            id="alice-private-key"
            v-model="alicePrivateKey"
            :maxLength
          />
        </div>
        <div>
          <Label for="bob-private-key">Bob Private Key</Label>
          <TextInput id="bob-private-key" v-model="bobPrivateKey" :maxLength />
        </div>
        <div>
          <BaseButton class="bg-teal-600" v-model="bobPrivateKey"
            >Submit</BaseButton
          >
        </div>
      </div>
    </template>
    <template #[StepKind.GeneratePrivateKeys]="{ step }">
      <p class="mb-2">
        First, Alice and Bob each generate their own private key. These should
        be large, random numbers. In this case, they have been provided:
      </p>
      <Typography variant="h4">Alice's Private Key:</Typography>
      <Container>
        <p class="break-all">{{ step?.value[0] }}</p>
      </Container>
      <Typography variant="h4" class="mt-2">Bob's Private Key:</Typography>
      <Container>
        <p class="break-all">{{ step?.value[1] }}</p>
      </Container>
    </template>
    <!-- Step for GeneratePublicKeys -->
    <template #[StepKind.GeneratePublicKeys]="{ step }">
      <p class="mb-2">
        Alice and Bob both generate their public keys by performing elliptic
        curve multiplication using their private keys, generating a point on the
        elliptic curve. These public keys are then shared with each other.
      </p>
      <Typography variant="h4">Alice's Public Key:</Typography>
      <div class="flex items-center mb-1">
        <span>X:</span>
        <Container>
          <p class="break-all">
            {{ step?.value?.[0]?.[0] || "Not available" }}
          </p>
        </Container>
      </div>
      <div class="flex items-center">
        <span>Y:</span>
        <Container>
          <p class="break-all">
            {{ step?.value?.[0]?.[1] || "Not available" }}
          </p>
        </Container>
      </div>
      <Typography variant="h4" class="mt-2">Bob's Public Key:</Typography>
      <div class="flex items-center mb-1">
        <span>X:</span>
        <Container>
          <p class="break-all">
            {{ step?.value?.[1]?.[0] || "Not available" }}
          </p>
        </Container>
      </div>
      <div class="flex items-center">
        <span>Y:</span>
        <Container>
          <p class="break-all">
            {{ step?.value?.[1]?.[1] || "Not available" }}
          </p>
        </Container>
      </div>
    </template>

    <template #[StepKind.ExchangeKeys]>
      <p class="mb-2">
        Alice and Bob exchange their public keys. The exchange ensures both
        parties can now calculate the shared secret using their private key and
        the other party's public key.
      </p>
    </template>

    <template #[StepKind.ComputeSharedSecret]="{ step }">
      <p class="mb-2">
        Now, Alice and Bob each compute the shared secret. This is done by
        multiplying their private key with the other party's public key. Both
        parties will end up with the same shared secret, which can be used for
        symmetric encryption or hashing.
      </p>
      <Typography variant="h4">Shared Secret:</Typography>
      <Container>
        <p class="break-all">{{ step?.value[0] }}</p>
      </Container>
    </template>
  </UtilModule>
</template>
