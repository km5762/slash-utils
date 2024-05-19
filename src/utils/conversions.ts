import { strip } from "./string-formatting";

export function toUint8Array(string: string) {
  return Uint8Array.from(parseBytes(strip(string), 8));
}

export function toUint32Array(string: string) {
  return Uint32Array.from(parseBytes(strip(string), 32));
}

function parseBytes(string: string, size: number) {
  const groupLength = size / 4;
  const regex = new RegExp(`.{1,${groupLength}}`, "g");
  return string.match(regex)!.map((byte) => parseInt(byte, 16));
}
