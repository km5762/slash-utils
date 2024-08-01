use alloc::vec::Vec;

use crate::{
    sha256::Sha256,
    sha_core::{ch, hash, maj, pad, parse, HashingAlgorithm},
};

pub struct Sha224 {
    hasher: Sha256,
}

impl Sha224 {
    fn test(&self) {
        Sha256::update_fn(t, working_variables, schedule)
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
