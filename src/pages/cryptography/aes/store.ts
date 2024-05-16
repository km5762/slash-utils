import { reactive } from "vue";
import { IntermediateValues } from "../../../../utils/pkg/slash_utils";

export const store = reactive<IntermediateValues>({
  free: function (): void {
    throw new Error("Function not implemented.");
  },
  final_add_round_key: "",
  initial_add_round_key: "",
  rounds: [],
  shift_rows: "",
  sub_bytes: "",
});
