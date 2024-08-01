use alloc::vec::Vec;

use crate::sha_core::{ch, hash, maj, pad, parse, HashingAlgorithm};

pub struct Sha1 {
    buffer: Vec<u8>,
}

impl Sha1 {
    const K: [u32; 4] = [0x5a827999, 0x6ed9eba1, 0x8f1bbcdc, 0xca62c1d6];
    const FUNCTIONS: [fn(u32, u32, u32) -> u32; 4] =
        [ch::<u32>, Self::parity, maj::<u32>, Self::parity];

    fn parity(x: u32, y: u32, z: u32) -> u32 {
        x ^ y ^ z
    }

    fn schedule_fn(t: usize, schedule: &[u32]) -> u32 {
        (schedule[t - 3] ^ schedule[t - 8] ^ schedule[t - 14] ^ schedule[t - 16]).rotate_left(1)
    }

    fn update_fn(t: usize, working_variables: &mut [u32], schedule: &[u32]) {
        let temp = working_variables[0]
            .rotate_left(5)
            .wrapping_add(Self::FUNCTIONS[t / 20](
                working_variables[1],
                working_variables[2],
                working_variables[3],
            ))
            .wrapping_add(working_variables[4])
            .wrapping_add(Self::K[t / 20])
            .wrapping_add(schedule[t]);
        working_variables[4] = working_variables[3];
        working_variables[3] = working_variables[2];
        working_variables[2] = working_variables[1].rotate_left(30);
        working_variables[1] = working_variables[0];
        working_variables[0] = temp;
    }
}

impl HashingAlgorithm for Sha1 {
    type Digest = [u32; 5];

    fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    fn update(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(&data);
    }

    fn reset(&mut self) {
        self.buffer.clear();
    }

    fn digest(&self) -> Self::Digest {
        let blocks = parse::<u32>(&pad::<u32, u64>(&self.buffer));
        let blocks: Vec<&[u32]> = blocks.iter().map(|block| &block[..]).collect();
        let mut initial_hash = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
        hash(
            &blocks,
            &mut initial_hash,
            &mut [0u32; 80],
            &mut [0u32; 5],
            &Self::schedule_fn,
            &mut Self::update_fn,
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
        test_hashes::<Sha1>(&[
            (
                b"abc",
                [0xa9993e36, 0x4706816a, 0xba3e2571, 0x7850c26c, 0x9cd0d89d],
            ),
            (
                b"",
                [0xda39a3ee, 0x5e6b4b0d, 0x3255bfef, 0x95601890, 0xafd80709],
            ),
            (
                b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
                [0x84983e44, 0x1c3bd26e, 0xbaae4aa1, 0xf95129e5, 0xe54670f1],
            ),
            (
                &b"a".repeat(1_000_000),
                [0x34aa973c, 0xd4c4daa4, 0xf61eeb2b, 0xdbad2731, 0x6534016f],
            ),
            (
                b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu",
                [0xa49b2446, 0xa02c645b , 0xf419f995, 0xb6709125, 0x3a04a259],
            ),
            (
                &b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmno".repeat(16_777_216),
                [0x7789f0c9, 0xef7bfc40, 0xd9331114, 0x3dfbe69e, 0x2017f592],
            ),
        ]);
    }
}
