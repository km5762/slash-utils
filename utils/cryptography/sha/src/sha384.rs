use alloc::vec::Vec;

use crate::{
    sha512::Sha512,
    sha_core::{hash, pad, parse, HashingAlgorithm},
};

pub struct Sha384 {
    buffer: Vec<u8>,
}

impl HashingAlgorithm<u64, 5> for Sha384 {
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
            &Sha512::schedule_fn,
            &mut &Sha512::update_fn,
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
        test_hashes::<Sha384, u64, 5>(&[
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
