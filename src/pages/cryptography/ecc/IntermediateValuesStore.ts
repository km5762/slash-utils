import { defineStore } from "pinia";
import init, {
  EcdsaP256,
  EcdsaP384,
  EcdsaP521,
} from "@utils/cryptography/ecc/pkg/ecc";
import { reactive, ref, watch } from "vue";

export const useIntermediateValuesStore = defineStore(
  "EccIntermediateValues",
  () => {
    const e = ref("");
    const z = ref("");
    const x = ref("");
    const y = ref("");
    const r = ref("");
    const s = ref("");

    watch(r, () => {
      console.log(r.value);
    });

    return { e, z, x, y, r, s };
  }
);
