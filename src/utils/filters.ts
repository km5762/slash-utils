export function allIntegers(input: string) {
  if (input[0] === "-") {
    return "-" + input.substring(1).replace(/^0+|[^\d]/g, "");
  } else {
    return input.replace(/^0+|[^\d.]/g, "");
  }
}

export function hex(input: string) {
  return input.replace(/[^0-9a-fA-F]/g, "");
}
