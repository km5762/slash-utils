export type SigningAlgorithmConfig = {
  p: string;
  a: string;
  b: string;
  gx: string;
  gy: string;
  n: string;
};

export const P256: SigningAlgorithmConfig = {
  p: "ffffffff00000001000000000000000000000000ffffffffffffffffffffffff",
  a: "ffffffff00000001000000000000000000000000fffffffffffffffffffffffc",
  b: "5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b",
  gx: "6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296",
  gy: "4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5",
  n: "ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551",
};

export const P384: SigningAlgorithmConfig = {
  p: "fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffeffffffff0000000000000000ffffffff",
  a: "fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffeffffffff0000000000000000fffffffc",
  b: "b3312fa7e23ee7e4988e056be3f82d19181d9c6efe8141120314088f5013875ac656398d8a2ed19d2a85c8edd3ec2aef",
  gx: "aa87ca22be8b05378eb1c71ef320ad746e1d3b628ba79b9859f741e082542a385502f25dbf55296c3a545e3872760ab7",
  gy: "3617de4a96262c6f5d9e98bf9292dc29f8f41dbd289a147ce9da3113b5f0b8c00a60b1ce1d7e819d7a431d7c90ea0e5f",
  n: "ffffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf581a0db248b0a77aecec196accc52973",
};

export const P521: SigningAlgorithmConfig = {
  p: "01ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
  a: "01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc",
  b: "0051953eb9618e1c9a1f929a21a0b68540eea2da725b99b315f3b8b489918ef109e156193951ec7e937b1652c0bd3bb1bf073573df883d2c34f1ef451fd46b503f00",
  gx: "00c6a727cda6d3d828bea62a0f5ed1e5c8c6a0dd9b586a170b1a5ec7b22dddc96b62c29f74621f2cbe0e4861da927fa2ef7e1bb06a12c7bd86bb46f8ab993c85ed8",
  gy: "011839296a789a3bc0045c8a5fb42c7d1b756c78b89b4f0679cc62a6eaecb4abaecc3c6e7f0597e807e9b0da93d5e8f81cf12d2134fc9b42d8462b503f00",
  n: "01fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa51868783bf2f966b7fcc0148f709a5d03bb5c9b8899c47aebeb7a0c8ee0",
};

export type SigningAlgorithmType = "P-256" | "P-384" | "P-521" | "custom";
