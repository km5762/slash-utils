import { BaseError, type Result } from "./error-types";
import { strip } from "./string-formatting";

class ParseHexError extends BaseError {
  constructor(hex: string) {
    super(`given string is not valid hex`, {
      context: { hex },
    });
  }
}

class HexLengthError extends BaseError {
  constructor(hex: string, length: number, size: number) {
    super(
      `given hex string would overflow the length specified for the byte buffer`,
      {
        context: { hex, length, size },
      }
    );
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

  toBytes(length: number): Result<Uint8Array, HexLengthError> {
    return this.#parseInts(length, Uint8Array);
  }

  toWords(length: number): Result<Uint32Array, HexLengthError> {
    return this.#parseInts(length, Uint32Array);
  }

  concat(string: string) {
    if (!HexString.pattern.test(string)) {
      throw new ParseHexError(string);
    }

    this.string += string;
  }

  #parseInts<
    T extends
      | Uint8ArrayConstructor
      | Uint16ArrayConstructor
      | Uint32ArrayConstructor,
  >(
    length: number,
    TypedArrayConstructor: T
  ): Result<InstanceType<T>, HexLengthError> {
    const size = TypedArrayConstructor.BYTES_PER_ELEMENT * 8;
    let hex = this.string;
    const minBits = hex.length * 4;
    const specifiedBits = length * size;

    if (minBits > specifiedBits) {
      return { success: false, error: new HexLengthError(hex, length, size) };
    }

    if (hex.length % 2 !== 0) {
      hex = "0" + hex;
    }

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
    return { success: true, result: ints as InstanceType<T> };
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
