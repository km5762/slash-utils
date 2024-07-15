import { defineStore } from "pinia";
import init, {
  IntermediateValue,
  decrypt,
  encrypt,
} from "@utils/cryptography/aes/pkg/aes";
import { ref } from "vue";
import { toUint32Array } from "../../../utils/conversions";
import { HexString } from "@/utils/hex-string";
import { strip } from "@/utils/string-formatting";

export enum Mode {
  ENCRYPTION,
  DECRYPTION,
}

export const useIntermediateValuesStore = defineStore(
  "AesIntermediateValues",
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
      let block =
        mode.value === Mode.ENCRYPTION
          ? decryptedBlock.value
          : encryptedBlock.value;
      let key =
        mode.value === Mode.ENCRYPTION
          ? encryptionKey.value
          : decryptionKey.value;

      const blockHex = new HexString(block);
      const keyHex = new HexString(key);

      const bytes = blockHex.toBytes(16);
      const words = keyHex.toWords(keyHex.string.length > 32 ? 8 : 4);

      if (bytes.success && words.success) {
        intermediateValues.value = cipherFunction(
          bytes.result,
          words.result,
          enabledTransforms.value
        );
      }
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
