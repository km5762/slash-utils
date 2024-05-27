export function allIntegers(input: string) {
  if (input[0] === "-") {
    return "-" + input.substring(1).replace(/^0+|[^\d.]/g, "");
  } else {
    return input.replace(/^0+|[^\d.]/g, "");
  }
}
