#[cfg(test)]
use crate::cryptography::aes;

#[test]
fn rot_word() {
    assert_eq!(0xcf4f3c09, aes::rot_word(0x09cf4f3c));
}

#[test]
fn sub_word() {
    assert_eq!(0x8a84eb01, aes::sub_word(0xcf4f3c09));
}

#[test]
fn expand_128_key() {
    let key = [0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c];
    let w = aes::key_expansion(&key);
    let w_expected: [u32; 44] = [
        0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c, 0xa0fafe17, 0x88542cb1, 0x23a33939,
        0x2a6c7605, 0xf2c295f2, 0x7a96b943, 0x5935807a, 0x7359f67f, 0x3d80477d, 0x4716fe3e,
        0x1e237e44, 0x6d7a883b, 0xef44a541, 0xa8525b7f, 0xb671253b, 0xdb0bad00, 0xd4d1c6f8,
        0x7c839d87, 0xcaf2b8bc, 0x11f915bc, 0x6d88a37a, 0x110b3efd, 0xdbf98641, 0xca0093fd,
        0x4e54f70e, 0x5f5fc9f3, 0x84a64fb2, 0x4ea6dc4f, 0xead27321, 0xb58dbad2, 0x312bf560,
        0x7f8d292f, 0xac7766f3, 0x19fadc21, 0x28d12941, 0x575c006e, 0xd014f9a8, 0xc9ee2589,
        0xe13f0cc8, 0xb6630ca6,
    ];
    assert_eq!(w_expected, w);
}

#[test]
fn add_round_key() {
    let mut state = [
        [0x47, 0x40, 0xa3, 0x4c],
        [0x37, 0xd4, 0x70, 0x9f],
        [0x94, 0xe4, 0x3a, 0x42],
        [0xed, 0xa5, 0xa6, 0xbc],
    ];
    let round_key = [0xac7766f3, 0x19fadc21, 0x28d12941, 0x575c006a];
    aes::add_round_key(&mut state, &round_key);
    let state_expected = [
        [0xeb, 0x59, 0x8b, 0x1b],
        [0x40, 0x2e, 0xa1, 0xc3],
        [0xf2, 0x38, 0x13, 0x42],
        [0x1e, 0x84, 0xe7, 0xd6],
    ];
    assert_eq!(state_expected, state);
}

#[test]
fn shift_rows() {
    let mut state = [
        [0x47, 0x40, 0xa3, 0x4c],
        [0x37, 0xd4, 0x70, 0x9f],
        [0x94, 0xe4, 0x3a, 0x42],
        [0xed, 0xa5, 0xa6, 0xbc],
    ];
    aes::shift_rows(&mut state);
    let state_expected = [
        [0x47, 0x40, 0xa3, 0x4c],
        [0x9f, 0x37, 0xd4, 0x70],
        [0x3a, 0x42, 0x94, 0xe4],
        [0xa5, 0xa6, 0xbc, 0xed],
    ];
    assert_eq!(state_expected, state);
}

#[test]
fn multiply_galois() {
    assert_eq!(0xfe, aes::multiply_galois(0x57, 0x13));
}

#[test]
fn double_galois() {
    assert_eq!(0x15, aes::multiply_galois(0x87, 0x2));
}

#[test]
fn triple_galois() {
    assert_eq!(0xa2, aes::multiply_galois(0x95, 0x3));
}

#[test]
fn mix_columns() {
    let mut state = [
        [0x09, 0x6f, 0x2c, 0xda],
        [0x28, 0x74, 0x4a, 0x08],
        [0x7f, 0x6a, 0x62, 0xe3],
        [0x47, 0xbf, 0x04, 0xee],
    ];
    aes::mix_columns(&mut state);
    let state_expected = [
        [0x52, 0x97, 0xe0, 0xba],
        [0x9f, 0x86, 0x1a, 0x1a],
        [0x16, 0x15, 0xae, 0x26],
        [0xc2, 0xca, 0x54, 0x59],
    ];
    assert_eq!(state_expected, state);
}
