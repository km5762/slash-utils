#![no_std]

pub mod sha1;
pub mod sha224;
pub mod sha256;
pub mod sha384;
pub mod sha512;
pub mod sha_core;

pub use sha1::*;
pub use sha224::*;
pub use sha256::*;
pub use sha384::*;
pub use sha512::*;
pub use sha_core::*;

extern crate alloc;

#[cfg(test)]
mod tests {
    use sha_core::HashingAlgorithm;

    use super::*;
    use core::fmt::Debug;

    pub(crate) fn test_hashes<H, T, const N: usize>(test_cases: &[(&[u8], [T; N])])
    where
        H: HashingAlgorithm<T, N>,
        T: Debug + PartialEq,
    {
        let mut hasher = H::new();
        for (i, case) in test_cases.iter().enumerate() {
            hasher.update(case.0);
            let result = hasher.digest();
            assert_eq!(
                &case.1, &result,
                "Test case {} failed:\nInput: {:?}\nExpected: {:?}\nGot: {:?}",
                i, case.0, case.1, result
            );
            hasher.reset();
        }
    }
}
