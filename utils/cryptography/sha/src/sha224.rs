use alloc::vec::Vec;

use crate::{
    sha256::Sha256,
    sha_core::{hash, pad, parse, HashingAlgorithm},
};

pub struct Sha224 {
    buffer: Vec<u8>,
}

impl HashingAlgorithm for Sha224 {
    type Digest = [u32; 7];

    fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    fn update(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    fn reset(&mut self) {
        self.buffer.clear();
    }

    fn digest(&self) -> [u32; 7] {
        let mut initial_hash: [u32; 8] = [
            0xc1059ed8, 0x367cd507, 0x3070dd17, 0xf70e5939, 0xffc00b31, 0x68581511, 0x64f98fa7,
            0xbefa4fa4,
        ];
        let blocks = parse(&pad::<u32, u64>(&self.buffer));
        let blocks: Vec<&[u32]> = blocks.iter().map(|block| &block[..]).collect();
        hash(
            &blocks,
            &mut initial_hash,
            &mut [0u32; 64],
            &mut [0u32; 8],
            &Sha256::schedule_fn,
            &mut &Sha256::update_fn,
        );

        let mut truncated = [0u32; 7];
        truncated.copy_from_slice(&initial_hash[..7]);
        truncated
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::test_hashes;

    use super::*;

    #[test]
    fn test_hash() {
        test_hashes::<Sha224>(&[
            (
                b"abc",
                [
                    0x23097d22, 0x3405d822, 0x8642a477, 0xbda255b3, 0x2aadbce4, 0xbda0b3f7,
                    0xe36c9da7,
                ],
            ),
            (
                b"",
                [
                    0xd14a028c, 0x2a3a2bc9, 0x476102bb, 0x288234c4, 0x15a2b01f, 0x828ea62a,
                    0xc5b3e42f,
                ],
            ),
            (
                b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
                [
                    0x75388b16, 0x512776cc, 0x5dba5da1, 0xfd890150, 0xb0c6455c, 0xb4f58b19,
                    0x52522525,
                ],
            ),
            (
                b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu",
                [
                    0xc97ca9a5, 0x59850ce9, 0x7a04a96d, 0xef6d99a9, 0xe0e0e2ab, 0x14e6b8df,
                    0x265fc0b3,
                ],
            ),
            (
                &b"a".repeat(1_000_000),
                [
                    0x20794655, 0x980c91d8, 0xbbb4c1ea, 0x97618a4b, 0xf03f4258, 0x1948b2ee,
                    0x4ee7ad67,
                ],
            ),
            (
                &b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmno".repeat(16_777_216),
                [
                    0xb5989713, 0xca4fe47a, 0x009f8621, 0x980b34e6, 0xd63ed306, 0x3b2a0a2c,
                    0x867d8a85,
                ],
            ),
        ]);
    }
}
