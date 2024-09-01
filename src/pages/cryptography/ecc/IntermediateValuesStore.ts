import { defineStore } from "pinia";
import init, {
  EcdsaP256,
  EcdsaP384,
  EcdsaP521,
  SigningIntermediateValuesHex,
  VerifyingIntermediateValuesHex,
} from "@utils/cryptography/ecc/pkg/ecc";
import { reactive, ref, watch } from "vue";

export const useIntermediateValuesStore = defineStore(
  "EccIntermediateValues",
  () => {
    const selectedMode = ref("Sign");
    const signingIntermediateValues = ref<SigningIntermediateValuesHex>();
    const verifyingIntermediateValues = ref<VerifyingIntermediateValuesHex>();

    return {
      selectedMode,
      signingIntermediateValues,
      verifyingIntermediateValues,
    };
  }
);
