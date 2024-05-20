extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct IntermediateValue {
    pub transformation: String,
    pub value: String,
}

const NB: usize = 4;

const SBOX: [u8; 256] = [
    0x63, 0x7C, 0x77, 0x7B, 0xF2, 0x6B, 0x6F, 0xC5, 0x30, 0x01, 0x67, 0x2B, 0xFE, 0xD7, 0xAB, 0x76,
    0xCA, 0x82, 0xC9, 0x7D, 0xFA, 0x59, 0x47, 0xF0, 0xAD, 0xD4, 0xA2, 0xAF, 0x9C, 0xA4, 0x72, 0xC0,
    0xB7, 0xFD, 0x93, 0x26, 0x36, 0x3F, 0xF7, 0xCC, 0x34, 0xA5, 0xE5, 0xF1, 0x71, 0xD8, 0x31, 0x15,
    0x04, 0xC7, 0x23, 0xC3, 0x18, 0x96, 0x05, 0x9A, 0x07, 0x12, 0x80, 0xE2, 0xEB, 0x27, 0xB2, 0x75,
    0x09, 0x83, 0x2C, 0x1A, 0x1B, 0x6E, 0x5A, 0xA0, 0x52, 0x3B, 0xD6, 0xB3, 0x29, 0xE3, 0x2F, 0x84,
    0x53, 0xD1, 0x00, 0xED, 0x20, 0xFC, 0xB1, 0x5B, 0x6A, 0xCB, 0xBE, 0x39, 0x4A, 0x4C, 0x58, 0xCF,
    0xD0, 0xEF, 0xAA, 0xFB, 0x43, 0x4D, 0x33, 0x85, 0x45, 0xF9, 0x02, 0x7F, 0x50, 0x3C, 0x9F, 0xA8,
    0x51, 0xA3, 0x40, 0x8F, 0x92, 0x9D, 0x38, 0xF5, 0xBC, 0xB6, 0xDA, 0x21, 0x10, 0xFF, 0xF3, 0xD2,
    0xCD, 0x0C, 0x13, 0xEC, 0x5F, 0x97, 0x44, 0x17, 0xC4, 0xA7, 0x7E, 0x3D, 0x64, 0x5D, 0x19, 0x73,
    0x60, 0x81, 0x4F, 0xDC, 0x22, 0x2A, 0x90, 0x88, 0x46, 0xEE, 0xB8, 0x14, 0xDE, 0x5E, 0x0B, 0xDB,
    0xE0, 0x32, 0x3A, 0x0A, 0x49, 0x06, 0x24, 0x5C, 0xC2, 0xD3, 0xAC, 0x62, 0x91, 0x95, 0xE4, 0x79,
    0xE7, 0xC8, 0x37, 0x6D, 0x8D, 0xD5, 0x4E, 0xA9, 0x6C, 0x56, 0xF4, 0xEA, 0x65, 0x7A, 0xAE, 0x08,
    0xBA, 0x78, 0x25, 0x2E, 0x1C, 0xA6, 0xB4, 0xC6, 0xE8, 0xDD, 0x74, 0x1F, 0x4B, 0xBD, 0x8B, 0x8A,
    0x70, 0x3E, 0xB5, 0x66, 0x48, 0x03, 0xF6, 0x0E, 0x61, 0x35, 0x57, 0xB9, 0x86, 0xC1, 0x1D, 0x9E,
    0xE1, 0xF8, 0x98, 0x11, 0x69, 0xD9, 0x8E, 0x94, 0x9B, 0x1E, 0x87, 0xE9, 0xCE, 0x55, 0x28, 0xDF,
    0x8C, 0xA1, 0x89, 0x0D, 0xBF, 0xE6, 0x42, 0x68, 0x41, 0x99, 0x2D, 0x0F, 0xB0, 0x54, 0xBB, 0x16,
];

const INVSBOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

const RCON: [u32; 11] = [
    0x00000000, 0x01000000, 0x02000000, 0x04000000, 0x08000000, 0x10000000, 0x20000000, 0x40000000,
    0x80000000, 0x1B000000, 0x36000000,
];

fn rot_word(word: u32) -> u32 {
    ((word << 8) & 0xFFFFFF00) | ((word >> 24) & 0x000000FF)
}

fn sub_word(word: u32) -> u32 {
    let mut bytes = u32::to_be_bytes(word);
    for byte in &mut bytes {
        *byte = get_sub(*byte);
    }
    u32::from_be_bytes(bytes)
}

fn get_sub(byte: u8) -> u8 {
    let upper_nibble: usize = (byte >> 4 & 0xF).into();
    let lower_nibble: usize = (byte & 0xF).into();

    SBOX[upper_nibble * 16 + lower_nibble]
}

fn get_inv_sub(byte: u8) -> u8 {
    let upper_nibble: usize = (byte >> 4 & 0xF).into();
    let lower_nibble: usize = (byte & 0xF).into();

    INVSBOX[upper_nibble * 16 + lower_nibble]
}

fn key_expansion(key: &[u32]) -> Box<[u32]> {
    let nk: usize = key.len();
    let nr = nk + 6;
    let mut w = vec![0u32; NB * (nr + 1)];

    for i in 0..nk {
        w[i] = key[i];
    }

    for i in nk..w.len() {
        let mut temp = w[i - 1];
        if i % nk == 0 {
            temp = (sub_word(rot_word(temp))) ^ RCON[i / nk];
        } else if (nk > 6) && i % nk == 4 {
            temp = sub_word(temp);
        }
        w[i] = w[i - nk] ^ temp;
    }

    w.into_boxed_slice()
}

fn add_round_key(state: &mut [u8; NB * NB], round_key: &[u32]) {
    for i in 0..round_key.len() {
        let bytes = round_key[i].to_be_bytes();
        for j in 0..NB {
            state[i * NB + j] ^= bytes[j];
        }
    }
}

fn shift_rows(state: &mut [u8; NB * NB]) {
    let temp = state[1];
    state[1] = state[5];
    state[5] = state[9];
    state[9] = state[13];
    state[13] = temp;

    let temp = state[2];
    state[2] = state[10];
    state[10] = temp;
    let temp = state[6];
    state[6] = state[14];
    state[14] = temp;

    let temp = state[15];
    state[15] = state[11];
    state[11] = state[7];
    state[7] = state[3];
    state[3] = temp;
}

fn mix_columns(state: &mut [u8; NB * NB]) {
    for i in 0..NB {
        let s0 = state[i * NB];
        let s1 = state[i * NB + 1];
        let s2 = state[i * NB + 2];
        let s3 = state[i * NB + 3];

        state[i * NB] = galois_multiply(s0, 0x02) ^ galois_multiply(s1, 0x03) ^ s2 ^ s3;
        state[i * NB + 1] = s0 ^ galois_multiply(s1, 0x02) ^ galois_multiply(s2, 0x03) ^ s3;
        state[i * NB + 2] = s0 ^ s1 ^ galois_multiply(s2, 0x02) ^ galois_multiply(s3, 0x03);
        state[i * NB + 3] = galois_multiply(s0, 0x03) ^ s1 ^ s2 ^ galois_multiply(s3, 0x02);
    }
}

fn galois_multiply(a: u8, b: u8) -> u8 {
    let mut a_temp = a;
    let mut b_temp = b;
    let mut p = 0;
    for _ in 0..8 {
        if ((b_temp) & 1) == 1 {
            p ^= a_temp;
        }
        let carry = (a_temp & 0x80) != 0;
        a_temp <<= 1;
        if carry {
            a_temp ^= 0x1b;
        }
        b_temp >>= 1;
    }
    p
}

fn transform_enabled(enabled_transforms: u64, stage: usize) -> bool {
    (enabled_transforms >> stage) & 1 != 0
}

fn apply_transformation<F>(
    state: &mut [u8; NB * NB],
    transformation: F,
    stage: &mut usize,
    enabled_transforms: u64,
) -> String
where
    F: Fn(&mut [u8; NB * NB]),
{
    if transform_enabled(enabled_transforms, *stage) {
        transformation(state);
        *stage += 1;
        u8_array_to_hex_string(state)
    } else {
        *stage += 1;
        String::new()
    }
}

fn cipher(block: &[u8; 16], w: &[u32], enabled_transforms: u64) -> Box<[IntermediateValue]> {
    let mut stage = 0;
    let mut state = block.clone();
    let nr = w.len() / 4 - 1;
    let mut intermediate_values = Vec::with_capacity(nr * 4);

    intermediate_values.push(IntermediateValue {
        transformation: String::from("Initial Add Round Key"),
        value: apply_transformation(
            &mut state,
            |state| add_round_key(state, &w[0..4]),
            &mut stage,
            enabled_transforms,
        ),
    });

    for round in 1..nr {
        intermediate_values.push(IntermediateValue {
            transformation: String::from("Sub Bytes"),
            value: apply_transformation(&mut state, sub_bytes, &mut stage, enabled_transforms),
        });
        intermediate_values.push(IntermediateValue {
            transformation: String::from("Shift Rows"),
            value: apply_transformation(&mut state, shift_rows, &mut stage, enabled_transforms),
        });
        intermediate_values.push(IntermediateValue {
            transformation: String::from("Mix Columns"),
            value: apply_transformation(&mut state, mix_columns, &mut stage, enabled_transforms),
        });
        intermediate_values.push(IntermediateValue {
            transformation: String::from("Add Round Key"),
            value: apply_transformation(
                &mut state,
                |state| add_round_key(state, &w[round * 4..(round + 1) * 4]),
                &mut stage,
                enabled_transforms,
            ),
        });
    }

    intermediate_values.push(IntermediateValue {
        transformation: String::from("Sub Bytes"),
        value: apply_transformation(&mut state, sub_bytes, &mut stage, enabled_transforms),
    });
    intermediate_values.push(IntermediateValue {
        transformation: String::from("Shift Rows"),
        value: apply_transformation(&mut state, shift_rows, &mut stage, enabled_transforms),
    });
    intermediate_values.push(IntermediateValue {
        transformation: String::from("Add Round Key"),
        value: apply_transformation(
            &mut state,
            |state| add_round_key(state, &w[w.len() - 4..]),
            &mut stage,
            enabled_transforms,
        ),
    });

    intermediate_values.into_boxed_slice()
}

fn sub_bytes(state: &mut [u8; NB * NB]) {
    for i in 0..NB * NB {
        state[i] = get_sub(state[i]);
    }
}

fn inv_shift_rows(state: &mut [u8; NB * NB]) {
    let temp = state[13];
    state[13] = state[9];
    state[9] = state[5];
    state[5] = state[1];
    state[1] = temp;

    let temp = state[2];
    state[2] = state[10];
    state[10] = temp;
    let temp = state[6];
    state[6] = state[14];
    state[14] = temp;

    let temp = state[3];
    state[3] = state[7];
    state[7] = state[11];
    state[11] = state[15];
    state[15] = temp;
}

fn inv_sub_bytes(state: &mut [u8; NB * NB]) {
    for i in 0..NB * NB {
        state[i] = get_inv_sub(state[i]);
    }
}

fn inv_mix_columns(state: &mut [u8; NB * NB]) {
    for i in 0..NB {
        let s0 = state[i * NB];
        let s1 = state[i * NB + 1];
        let s2 = state[i * NB + 2];
        let s3 = state[i * NB + 3];

        state[i * NB] = galois_multiply(s0, 0x0e)
            ^ galois_multiply(s1, 0x0b)
            ^ galois_multiply(s2, 0x0d)
            ^ galois_multiply(s3, 0x09);
        state[i * NB + 1] = galois_multiply(s0, 0x09)
            ^ galois_multiply(s1, 0x0e)
            ^ galois_multiply(s2, 0x0b)
            ^ galois_multiply(s3, 0x0d);
        state[i * NB + 2] = galois_multiply(s0, 0x0d)
            ^ galois_multiply(s1, 0x09)
            ^ galois_multiply(s2, 0x0e)
            ^ galois_multiply(s3, 0x0b);
        state[i * NB + 3] = galois_multiply(s0, 0x0b)
            ^ galois_multiply(s1, 0x0d)
            ^ galois_multiply(s2, 0x09)
            ^ galois_multiply(s3, 0x0e);
    }
}


fn inv_cipher(block: &[u8; 16], w: &[u32], enabled_transforms: u64) -> Box<[IntermediateValue]> {
    let mut stage = 0;
    let mut state = block.clone();
    let nr = w.len() / 4 - 1;
    let mut intermediate_values = Vec::with_capacity(nr * 4);

    intermediate_values.push(IntermediateValue {
        transformation: String::from("Initial Add Round Key"),
        value: apply_transformation(
            &mut state,
            |state| add_round_key(state, &w[w.len() - 4..]),
            &mut stage,
            enabled_transforms,
        ),
    });

    for round in (1..nr).rev() {
        intermediate_values.push(IntermediateValue {
            transformation: String::from("Shift Rows"),
            value: apply_transformation(&mut state, inv_shift_rows, &mut stage, enabled_transforms),
        });
        intermediate_values.push(IntermediateValue {
            transformation: String::from("Sub Bytes"),
            value: apply_transformation(&mut state, inv_sub_bytes, &mut stage, enabled_transforms),
        });
        intermediate_values.push(IntermediateValue {
            transformation: String::from("Add Round Key"),
            value: apply_transformation(
                &mut state,
                |state| add_round_key(state, &w[(4 * round)..(4 * round + 4)]),
                &mut stage,
                enabled_transforms,
            ),
        });
        intermediate_values.push(IntermediateValue {
            transformation: String::from("Mix Columns"),
            value: apply_transformation(&mut state, inv_mix_columns, &mut stage, enabled_transforms),
        });
    }

    intermediate_values.push(IntermediateValue {
        transformation: String::from("Shift Rows"),
        value: apply_transformation(&mut state, inv_shift_rows, &mut stage, enabled_transforms),
    });
    intermediate_values.push(IntermediateValue {
        transformation: String::from("Sub Bytes"),
        value: apply_transformation(&mut state, inv_sub_bytes, &mut stage, enabled_transforms),
    });
    intermediate_values.push(IntermediateValue {
        transformation: String::from("Add Round Key"),
        value: apply_transformation(
            &mut state,
            |state| add_round_key(state, &w[0..4]),
            &mut stage,
            enabled_transforms,
        ),
    });

    intermediate_values.into_boxed_slice()
}

fn u8_array_to_hex_string(data: &[u8]) -> String {
    let hex_chars: Vec<String> = data.iter().map(|&byte| format!("{:02x}", byte)).collect();
    hex_chars.join("")
}

#[wasm_bindgen]
pub fn encrypt(block: &[u8], key: &[u32], enabled_stages: u64) -> Box<[IntermediateValue]> {
    let w = key_expansion(&key);
    cipher(
        block.try_into().expect("Block must be 16 byte array"),
        &w,
        enabled_stages,
    )
}

#[wasm_bindgen]
pub fn decrypt(block: &[u8], key: &[u32], enabled_stages: u64) -> Box<[IntermediateValue]> {
    let w = key_expansion(&key);
    inv_cipher(
        block.try_into().expect("Block must be 16 byte array"),
        &w,
        enabled_stages,
    )
}

#[cfg(test)]
mod tests {
    use super::*;



#[test]
fn test_rot_word() {
    assert_eq!(0xcf4f3c09, rot_word(0x09cf4f3c));
}

#[test]
fn test_sub_word() {
    assert_eq!(0x8a84eb01, sub_word(0xcf4f3c09));
}

#[test]
fn expand_128_key() {
    let key = [0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c];
    let w = key_expansion(&key);
    let w_expected: [u32; 44] = [
        0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c, 0xa0fafe17, 0x88542cb1, 0x23a33939,
        0x2a6c7605, 0xf2c295f2, 0x7a96b943, 0x5935807a, 0x7359f67f, 0x3d80477d, 0x4716fe3e,
        0x1e237e44, 0x6d7a883b, 0xef44a541, 0xa8525b7f, 0xb671253b, 0xdb0bad00, 0xd4d1c6f8,
        0x7c839d87, 0xcaf2b8bc, 0x11f915bc, 0x6d88a37a, 0x110b3efd, 0xdbf98641, 0xca0093fd,
        0x4e54f70e, 0x5f5fc9f3, 0x84a64fb2, 0x4ea6dc4f, 0xead27321, 0xb58dbad2, 0x312bf560,
        0x7f8d292f, 0xac7766f3, 0x19fadc21, 0x28d12941, 0x575c006e, 0xd014f9a8, 0xc9ee2589,
        0xe13f0cc8, 0xb6630ca6,
    ];
    assert_eq!(w_expected, *w);
}

#[test]
fn atest_dd_round_key() {
    let mut state = [
        0x6B, 0xC1, 0xBE, 0xE2, 0x2E, 0x40, 0x9F, 0x96, 0xE9, 0x3D, 0x7E, 0x11, 0x73, 0x93, 0x17,
        0x2A,
    ];
    let round_key = [0x2B7E1516, 0x28AED2A6, 0xABF71588, 0x09CF4F3C];
    add_round_key(&mut state, &round_key);
    let state_expected = [
        0x40, 0xBF, 0xAB, 0xF4, 0x06, 0xEE, 0x4D, 0x30, 0x42, 0xCA, 0x6B, 0x99, 0x7A, 0x5C, 0x58,
        0x16,
    ];
    assert_eq!(state_expected, state);
}

#[test]
fn test_shift_rows() {
    let mut state = [
        0x09, 0x08, 0x62, 0xBF, 0x6F, 0x28, 0xE3, 0x04, 0x2C, 0x74, 0x7F, 0xEE, 0xDA, 0x4A, 0x6A,
        0x47,
    ];
    shift_rows(&mut state);
    let state_expected = [
        0x09, 0x28, 0x7F, 0x47, 0x6F, 0x74, 0x6A, 0xBF, 0x2C, 0x4A, 0x62, 0x04, 0xDA, 0x08, 0xE3,
        0xEE,
    ];
    assert_eq!(state_expected, state);
}

#[test]
fn test_multiply_galois() {
    assert_eq!(0xfe, galois_multiply(0x57, 0x13));
}

#[test]
fn test_double_galois() {
    assert_eq!(0x15, galois_multiply(0x87, 0x2));
}

#[test]
fn test_triple_galois() {
    assert_eq!(0xa4, galois_multiply(0x95, 0x3));
}

#[test]
fn test_mix_columns() {
    let mut state = [
        0x09, 0x28, 0x7F, 0x47, 0x6F, 0x74, 0x6A, 0xBF, 0x2C, 0x4A, 0x62, 0x04, 0xDA, 0x08, 0xE3,
        0xEE,
    ];
    mix_columns(&mut state);
    let state_expected = [
        0x52, 0x9F, 0x16, 0xC2, 0x97, 0x86, 0x15, 0xCA, 0xE0, 0x1A, 0xAE, 0x54, 0xBA, 0x1A, 0x26,
        0x59,
    ];
    assert_eq!(state_expected, state);
}

#[test]
fn test_sub_bytes() {
    let mut state = [
        0x40, 0xBF, 0xAB, 0xF4, 0x06, 0xEE, 0x4D, 0x30, 0x42, 0xCA, 0x6B, 0x99, 0x7A, 0x5C, 0x58,
        0x16,
    ];
    sub_bytes(&mut state);
    let state_expected = [
        0x09, 0x08, 0x62, 0xBF, 0x6F, 0x28, 0xE3, 0x04, 0x2C, 0x74, 0x7F, 0xEE, 0xDA, 0x4A, 0x6A,
        0x47,
    ];
    assert_eq!(state_expected, state);
}

#[test]
fn test_cipher() {
    let block = [
        0x6B, 0xC1, 0xBE, 0xE2, 0x2E, 0x40, 0x9F, 0x96, 0xE9, 0x3D, 0x7E, 0x11, 0x73, 0x93, 0x17,
        0x2A,
    ];
    let key = [0x2B7E1516, 0x28AED2A6, 0xABF71588, 0x09CF4F3C];
    let w = key_expansion(&key);
    let intermediate_values = cipher(&block, &w, 0xFFFFFFFFFFFFFFFF);
    let cipher_text_expected = "3ad77bb40d7a3660a89ecaf32466ef97";
    assert_eq!(
        cipher_text_expected,
        intermediate_values.last().unwrap().value
    );
}

#[test]
fn test_inv_shift_rows() {
    let mut state = [
        0x47, 0x40, 0xa3, 0x4c, 0x37, 0xd4, 0x70, 0x9f, 0x94, 0xe4, 0x3a, 0x42, 0xed, 0xa5, 0xa6,
        0xbc,
    ];
    shift_rows(&mut state);
    inv_shift_rows(&mut state);
    let state_expected = [
        0x47, 0x40, 0xa3, 0x4c, 0x37, 0xd4, 0x70, 0x9f, 0x94, 0xe4, 0x3a, 0x42, 0xed, 0xa5, 0xa6,
        0xbc,
    ];
    assert_eq!(state_expected, state);
}

#[test]
fn test_inv_sub_bytes() {
    let mut state = [
        0x47, 0x40, 0xa3, 0x4c, 0x37, 0xd4, 0x70, 0x9f, 0x94, 0xe4, 0x3a, 0x42, 0xed, 0xa5, 0xa6,
        0xbc,
    ];
    sub_bytes(&mut state);
    inv_sub_bytes(&mut state);
    let state_expected = [
        0x47, 0x40, 0xa3, 0x4c, 0x37, 0xd4, 0x70, 0x9f, 0x94, 0xe4, 0x3a, 0x42, 0xed, 0xa5, 0xa6,
        0xbc,
    ];
    assert_eq!(state_expected, state);
}

#[test]
fn test_inv_mix_columns() {
    let mut state = [
        0x47, 0x40, 0xa3, 0x4c, 0x37, 0xd4, 0x70, 0x9f, 0x94, 0xe4, 0x3a, 0x42, 0xed, 0xa5, 0xa6,
        0xbc,
    ];
    mix_columns(&mut state);
    inv_mix_columns(&mut state);
    let state_expected = [
        0x47, 0x40, 0xa3, 0x4c, 0x37, 0xd4, 0x70, 0x9f, 0x94, 0xe4, 0x3a, 0x42, 0xed, 0xa5, 0xa6,
        0xbc,
    ];
    assert_eq!(state_expected, state);
}

#[test]
fn test_inv_cipher() {
    let block = [
        0x3A, 0xD7, 0x7B, 0xB4, 0x0D, 0x7A, 0x36, 0x60, 0xA8, 0x9E, 0xCA, 0xF3, 0x24, 0x66, 0xEF,
        0x97,
    ];
    let key = [0x2B7E1516, 0x28AED2A6, 0xABF71588, 0x09CF4F3C];
    let w = key_expansion(&key);
    let intermediate_values = inv_cipher(&block, &w, 0xFFFFFFFFFFFFFFFF);
    let block_expected = "6bc1bee22e409f96e93d7e117393172a";
    assert_eq!(block_expected, intermediate_values.last().unwrap().value);
}
}
