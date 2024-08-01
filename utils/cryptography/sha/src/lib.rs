#![no_std]

pub mod sha1;
pub mod sha256;
pub mod sha384;
pub mod sha512;
pub mod sha_core;

extern crate alloc;

#[cfg(test)]
mod tests {
    use sha_core::HashingAlgorithm;

    use super::*;
    use core::fmt::Debug;

    pub(crate) fn test_hashes<T>(test_cases: &[(&[u8], T::Digest)])
    where
        T: HashingAlgorithm,
        T::Digest: PartialEq + Debug,
    {
        let mut hasher = T::new();
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
