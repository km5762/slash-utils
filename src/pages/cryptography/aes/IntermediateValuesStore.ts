import { defineStore } from "pinia";
import init, {
  IntermediateValue,
  decrypt,
  encrypt,
} from "@utils/cryptography/aes/pkg/aes";
import { ref } from "vue";
import { toUint8Array, toUint32Array } from "../../../utils/conversions";

export enum Mode {
  ENCRYPTION,
  DECRYPTION,
}

export const useIntermediateValuesStore = defineStore(
  "IntermediateValues",
  () => {
    const intermediateValues = ref<IntermediateValue[]>([]);
    const enabledTransforms = ref(0xffffffffffffffffn);
    const decryptedBlock = ref("");
    const encryptedBlock = ref("");
    const encryptionKey = ref("");
    const decryptionKey = ref("");
    const mode = ref<Mode>();

    function setTransform(step: number, enabled: boolean) {
      enabledTransforms.value ^= 1n << BigInt(step);
    }

    function computeIntermediateValues() {
      const cipherFunction = mode.value === Mode.ENCRYPTION ? encrypt : decrypt;
      const block =
        mode.value === Mode.ENCRYPTION
          ? decryptedBlock.value
          : encryptedBlock.value;
      const key =
        mode.value === Mode.ENCRYPTION
          ? encryptionKey.value
          : decryptionKey.value;
      intermediateValues.value = cipherFunction(
        toUint8Array(block),
        toUint32Array(key),
        enabledTransforms.value
      );
    }

    return {
      intermediateValues,
      decryptedBlock,
      encryptedBlock,
      encryptionKey,
      decryptionKey,
      enabledTransforms,
      mode,
      setTransform,
      computeIntermediateValues,
    };
  }
);
