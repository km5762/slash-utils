export function numeric(input: string) {
  return input.replace(/^0+|[^\d.]/g, "");
}
