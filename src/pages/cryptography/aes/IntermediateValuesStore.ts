import { defineStore } from "pinia";
import init, {
  IntermediateValue,
  encrypt,
} from "../../../../utils/pkg/slash_utils";
import { ref } from "vue";
import { toUint8Array, toUint32Array } from "../../../utils/conversions";

export const useIntermediateValuesStore = defineStore(
  "IntermediateValues",
  () => {
    const intermediateValues = ref<IntermediateValue[]>([]);
    const enabledTransforms = ref(0xffffffffffffffffn);
    const block = ref("");
    const key = ref("");

    function setTransform(step: number, enabled: boolean) {
      enabledTransforms.value ^= 1n << BigInt(step);
    }

    function computeIntermediateValues() {
      intermediateValues.value = encrypt(
        toUint8Array(block.value),
        toUint32Array(key.value),
        enabledTransforms.value
      );
    }

    return {
      intermediateValues,
      block,
      key,
      enabledTransforms,
      setTransform,
      computeIntermediateValues,
    };
  }
);
