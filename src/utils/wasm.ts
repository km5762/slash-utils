export function getWASMObjectProperties(object: any): string[] {
  const generatedProperties = new Set([
    "constructor",
    "__destroy_into_raw",
    "free",
  ]);
  return Object.getOwnPropertyNames(Object.getPrototypeOf(object)).filter(
    (key) => !generatedProperties.has(key)
  );
}

export function getWASMObjectValues(object: any) {
  return getWASMObjectProperties(object).map((p) => object[p]);
}
