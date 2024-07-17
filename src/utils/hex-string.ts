import { BaseError, type Result } from "./error-types";
import { strip } from "./string-formatting";

class ParseHexError extends BaseError {
  constructor(hex: string) {
    super(`given string is not valid hex`, {
      context: { hex },
    });
  }
}

export class HexString {
  string: string;
  static readonly pattern = /^[0-9a-f]+$/i;

  constructor(string: string) {
    const stripped = strip(string);
    if (!HexString.pattern.test(stripped)) {
      throw new ParseHexError(string);
    }

    this.string = stripped;
  }

  toBytes(length: number) {
    return this.#parseInts(length, Uint8Array);
  }

  toWords(length: number) {
    return this.#parseInts(length, Uint32Array);
  }

  #parseInts<
    T extends
      | Uint8ArrayConstructor
      | Uint16ArrayConstructor
      | Uint32ArrayConstructor,
  >(length: number, TypedArrayConstructor: T): InstanceType<T> {
    const size = TypedArrayConstructor.BYTES_PER_ELEMENT * 8;
    let hex = this.string;
    let ints = new TypedArrayConstructor(length);
    let hexDigitsPerInt = size / 4;
    let hexIndex = hex.length - hexDigitsPerInt;

    for (let i = length - 1; i >= 0; i--) {
      if (hexIndex >= 0) {
        ints[i] = parseInt(
          hex.substring(hexIndex, hexIndex + hexDigitsPerInt),
          16
        );
        hexIndex -= hexDigitsPerInt;
      } else {
        break;
      }
    }
    return ints as InstanceType<T>;
  }

  static fromBytes(bytes: Uint8Array) {
    let hex = [];

    for (let i = 0; i < bytes.length; i++) {
      let current = bytes[i] < 0 ? bytes[i] + 256 : bytes[i];
      hex.push((current >>> 4).toString(16));
      hex.push((current & 0xf).toString(16));
    }
    let hexString = hex.join("");
    hexString = hexString.replace(/^0+/, "");

    return new HexString(hexString);
  }

  static empty() {
    return new HexString("");
  }
}

export class HexStringObject {
  object: Record<string, HexString>;

  constructor(object: Record<string, string>) {
    this.object = Object.fromEntries(
      Object.entries(object).map(([key, value]) => [key, new HexString(value)])
    );
  }

  toByteObject(length: number) {
    return Object.fromEntries(
      Object.entries(this.object).map(([key, value]) => [
        key,
        value.toBytes(length),
      ])
    );
  }

  toWordObject(length: number) {
    return Object.fromEntries(
      Object.entries(this.object).map(([key, value]) => [
        key,
        value.toWords(length),
      ])
    );
  }
}
