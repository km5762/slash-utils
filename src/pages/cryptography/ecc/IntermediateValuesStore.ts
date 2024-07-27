import { defineStore } from "pinia";
import init, {
  EcdsaP256,
  EcdsaP384,
  EcdsaP521,
} from "@utils/cryptography/ecc/pkg/ecc";
import { reactive, ref, watch } from "vue";
import type { NestedObject } from "@/utils/hex-string";

export const useIntermediateValuesStore = defineStore(
  "EccIntermediateValues",
  () => {
    const e = ref("");
    const z = ref("");
    const x = ref("");
    const y = ref("");
    const r = ref("");
    const s = ref("");

    return { e, z, x, y, r, s };
  }
);

export class EcdsaParameters<T> {
  p: T;
  a: T;
  b: T;
  gx: T;
  gy: T;
  n: T;

  constructor(p: T, a: T, b: T, gx: T, gy: T, n: T) {
    this.p = p;
    this.a = a;
    this.b = b;
    this.gx = gx;
    this.gy = gy;
    this.n = n;
  }

  static schema(): NestedObject<undefined> {
    return {
      p: undefined,
      a: undefined,
      b: undefined,
      gx: undefined,
      gy: undefined,
      n: undefined,
    };
  }
}
