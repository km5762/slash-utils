use alloc::vec::Vec;

use crate::sha_core::{ch, hash, maj, pad, parse, HashingAlgorithm};

pub struct Sha384 {
    buffer: Vec<u8>,
}

impl Sha384 {
    const K: [u64; 80] = [
        0x428a2f98d728ae22,
        0x7137449123ef65cd,
        0xb5c0fbcfec4d3b2f,
        0xe9b5dba58189dbbc,
        0x3956c25bf348b538,
        0x59f111f1b605d019,
        0x923f82a4af194f9b,
        0xab1c5ed5da6d8118,
        0xd807aa98a3030242,
        0x12835b0145706fbe,
        0x243185be4ee4b28c,
        0x550c7dc3d5ffb4e2,
        0x72be5d74f27b896f,
        0x80deb1fe3b1696b1,
        0x9bdc06a725c71235,
        0xc19bf174cf692694,
        0xe49b69c19ef14ad2,
        0xefbe4786384f25e3,
        0x0fc19dc68b8cd5b5,
        0x240ca1cc77ac9c65,
        0x2de92c6f592b0275,
        0x4a7484aa6ea6e483,
        0x5cb0a9dcbd41fbd4,
        0x76f988da831153b5,
        0x983e5152ee66dfab,
        0xa831c66d2db43210,
        0xb00327c898fb213f,
        0xbf597fc7beef0ee4,
        0xc6e00bf33da88fc2,
        0xd5a79147930aa725,
        0x06ca6351e003826f,
        0x142929670a0e6e70,
        0x27b70a8546d22ffc,
        0x2e1b21385c26c926,
        0x4d2c6dfc5ac42aed,
        0x53380d139d95b3df,
        0x650a73548baf63de,
        0x766a0abb3c77b2a8,
        0x81c2c92e47edaee6,
        0x92722c851482353b,
        0xa2bfe8a14cf10364,
        0xa81a664bbc423001,
        0xc24b8b70d0f89791,
        0xc76c51a30654be30,
        0xd192e819d6ef5218,
        0xd69906245565a910,
        0xf40e35855771202a,
        0x106aa07032bbd1b8,
        0x19a4c116b8d2d0c8,
        0x1e376c085141ab53,
        0x2748774cdf8eeb99,
        0x34b0bcb5e19b48a8,
        0x391c0cb3c5c95a63,
        0x4ed8aa4ae3418acb,
        0x5b9cca4f7763e373,
        0x682e6ff3d6b2b8a3,
        0x748f82ee5defb2fc,
        0x78a5636f43172f60,
        0x84c87814a1f0ab72,
        0x8cc702081a6439ec,
        0x90befffa23631e28,
        0xa4506cebde82bde9,
        0xbef9a3f7b2c67915,
        0xc67178f2e372532b,
        0xca273eceea26619c,
        0xd186b8c721c0c207,
        0xeada7dd6cde0eb1e,
        0xf57d4f7fee6ed178,
        0x06f067aa72176fba,
        0x0a637dc5a2c898a6,
        0x113f9804bef90dae,
        0x1b710b35131c471b,
        0x28db77f523047d84,
        0x32caab7b40c72493,
        0x3c9ebe0a15c9bebc,
        0x431d67c49c100d4c,
        0x4cc5d4becb3e42b6,
        0x597f299cfc657e2a,
        0x5fcb6fab3ad6faec,
        0x6c44198c4a475817,
    ];

    fn upper_sig0(x: u64) -> u64 {
        x.rotate_right(28) ^ x.rotate_right(34) ^ x.rotate_right(39)
    }

    fn upper_sig1(x: u64) -> u64 {
        x.rotate_right(14) ^ x.rotate_right(18) ^ x.rotate_right(41)
    }

    fn lower_sig0(x: u64) -> u64 {
        x.rotate_right(1) ^ x.rotate_right(8) ^ (x >> 7)
    }

    fn lower_sig1(x: u64) -> u64 {
        x.rotate_right(19) ^ x.rotate_right(61) ^ (x >> 6)
    }

    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn update(&mut self, mut data: Vec<u8>) {
        self.buffer.append(&mut data);
    }

    fn schedule_fn(t: usize, schedule: &[u64]) -> u64 {
        Self::lower_sig1(schedule[t - 2])
            .wrapping_add(schedule[t - 7])
            .wrapping_add(Self::lower_sig0(schedule[t - 15]))
            .wrapping_add(schedule[t - 16])
    }

    fn update_fn(t: usize, working_variables: &mut [u64], schedule: &[u64]) {
        let temp1 = working_variables[7]
            .wrapping_add(Self::upper_sig1(working_variables[4]))
            .wrapping_add(ch(
                working_variables[4],
                working_variables[5],
                working_variables[6],
            ))
            .wrapping_add(Self::K[t])
            .wrapping_add(schedule[t]);
        let temp2 = Self::upper_sig0(working_variables[0]).wrapping_add(maj(
            working_variables[0],
            working_variables[1],
            working_variables[2],
        ));
        working_variables[7] = working_variables[6];
        working_variables[6] = working_variables[5];
        working_variables[5] = working_variables[4];
        working_variables[4] = working_variables[3].wrapping_add(temp1);
        working_variables[3] = working_variables[2];
        working_variables[2] = working_variables[1];
        working_variables[1] = working_variables[0];
        working_variables[0] = temp1.wrapping_add(temp2);
    }
}

impl HashingAlgorithm for Sha384 {
    type Digest = [u64; 5];

    fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    fn update(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    fn reset(&mut self) {
        self.buffer.clear();
    }

    fn digest(&self) -> [u64; 5] {
        let mut initial_hash: [u64; 8] = [
            0xcbbb9d5dc1059ed8,
            0x629a292a367cd507,
            0x9159015a3070dd17,
            0x152fecd8f70e5939,
            0x67332667ffc00b31,
            0x8eb44a8768581511,
            0xdb0c2e0d64f98fa7,
            0x47b5481dbefa4fa4,
        ];
        let blocks = parse(&pad::<u64, u128>(&self.buffer));
        let blocks: Vec<&[u64]> = blocks.iter().map(|block| &block[..]).collect();
        hash(
            &blocks,
            &mut initial_hash,
            &mut [0u64; 80],
            &mut [0u64; 8],
            &Self::schedule_fn,
            &mut &Self::update_fn,
        );

        let truncated: [u64; 5] = [
            initial_hash[0],
            initial_hash[1],
            initial_hash[2],
            initial_hash[3],
            initial_hash[4],
        ];

        truncated
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::test_hashes;

    use super::*;

    #[test]
    fn test_hash() {
        test_hashes::<Sha384>(&[
            (
                b"abc",
                [
                    0xcb00753f45a35e8b, 0xb5a03d699ac65007, 0x272c32ab0eded163, 0x1a8b605a43ff5bed,
                    0x8086072ba1e7cc23
                ]
            ),
            (
                b"",
                [
                    0x38b060a751ac9638, 0x4cd9327eb1b1e36a, 0x21fdb71114be0743, 0x4c0cc7bf63f6e1da,
                    0x274edebfe76f65fb
                ]
            ),
            (
                b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
                [
                    0x3391fdddfc8dc739, 0x3707a65b1b470939, 0x7cf8b1d162af05ab, 0xfe8f450de5f36bc6,
                    0xb0455a8520bc4e6f
                ]
            ),
            (
                b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu",
                [
                    0x09330c33f71147e8, 0x3d192fc782cd1b47, 0x53111b173b3b05d2, 0x2fa08086e3b0f712,
                    0xfcc7c71a557e2db9
                ]
            ),
            (
                &b"a".repeat(1_000_000),
                [
                    0x9d0e1809716474cb, 0x086e834e310a4a1c, 0xed149e9c00f24852, 0x7972cec5704c2a5b,
                    0x07b8b3dc38ecc4eb
                ]
            ),
            (
                &b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmno".repeat(16_777_216),
                [
                    0x5441235cc0235341, 0xed806a64fb354742, 0xb5e5c02a3c5cb71b, 0x5f63fb793458d8fd,
                    0xae599c8cd8884943
                ]
            ),
        ]);
    }
}
