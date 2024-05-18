import { defineStore } from "pinia";
import { ref } from "vue";

export const useEnabledTransformsStore = defineStore(
  "EnabledTransforms",
  () => {
    const enabledTransforms = ref(0xffffffffffffffffn);

    function setTransform(step: number, enabled: boolean) {
      enabledTransforms.value ^= 1n << BigInt(step);
    }

    return { enabledTransforms, setTransform };
  }
);
