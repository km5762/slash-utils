use alloc::vec::Vec;

use crate::sha_core::{ch, hash, maj, pad, parse, HashingAlgorithm};

pub struct Sha256 {
    buffer: Vec<u8>,
}

impl Sha256 {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    fn upper_sig0(x: u32) -> u32 {
        x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
    }

    fn upper_sig1(x: u32) -> u32 {
        x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
    }

    fn lower_sig0(x: u32) -> u32 {
        x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
    }

    fn lower_sig1(x: u32) -> u32 {
        x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
    }

    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub(crate) fn schedule_fn(t: usize, schedule: &[u32]) -> u32 {
        Self::lower_sig1(schedule[t - 2])
            .wrapping_add(schedule[t - 7])
            .wrapping_add(Self::lower_sig0(schedule[t - 15]))
            .wrapping_add(schedule[t - 16])
    }

    pub(crate) fn update_fn(t: usize, working_variables: &mut [u32], schedule: &[u32]) {
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

impl HashingAlgorithm for Sha256 {
    type Digest = [u32; 8];

    fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    fn update(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    fn reset(&mut self) {
        self.buffer.clear();
    }

    fn digest(&self) -> [u32; 8] {
        let mut initial_hash: [u32; 8] = [
            0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
            0x5be0cd19,
        ];
        let blocks = parse(&pad::<u32, u64>(&self.buffer));
        let blocks: Vec<&[u32]> = blocks.iter().map(|block| &block[..]).collect();
        hash(
            &blocks,
            &mut initial_hash,
            &mut [0u32; 64],
            &mut [0u32; 8],
            &Self::schedule_fn,
            &mut &Self::update_fn,
        );
        initial_hash
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::test_hashes;

    use super::*;

    #[test]
    fn test_hash() {
        test_hashes::<Sha256>(&[
            (
                b"abc",
                [
                    0xba7816bf, 0x8f01cfea, 0x414140de, 0x5dae2223,
                    0xb00361a3, 0x96177a9c, 0xb410ff61, 0xf20015ad
                ]
            ),
            (
                b"",
                [
                    0xe3b0c442, 0x98fc1c14, 0x9afbf4c8, 0x996fb924,
                    0x27ae41e4, 0x649b934c, 0xa495991b, 0x7852b855
                ]
            ),
            (
                b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
                [
                    0x248d6a61, 0xd20638b8, 0xe5c02693, 0x0c3e6039,
                    0xa33ce459, 0x64ff2167, 0xf6ecedd4, 0x19db06c1
                ]
            ),
            (
                b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu",
                [
                    0xcf5b16a7, 0x78af8380, 0x036ce59e, 0x7b049237,
                    0x0b249b11, 0xe8f07a51, 0xafac4503, 0x7afee9d1
                ]
            ),
            (
                &b"a".repeat(1_000_000),
                [
                    0xcdc76e5c, 0x9914fb92, 0x81a1c7e2, 0x84d73e67,
                    0xf1809a48, 0xa497200e, 0x046d39cc, 0xc7112cd0
                ]
            ),
            (
                &b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmno".repeat(16_777_216),
                [
                    0x50e72a0e, 0x26442fe2, 0x552dc393, 0x8ac58658,
                    0x228c0cbf, 0xb1d2ca87, 0x2ae43526, 0x6fcd055e
                ]
            ),
        ]);
    }
}
