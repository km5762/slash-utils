extern crate console_error_panic_hook;
extern crate wasm_bindgen;

use core::{array::TryFromSliceError, cmp::min, fmt::LowerHex};

use alloc::{boxed::Box, format, string::String, vec::Vec};
use big_num::{
    types::{U256, U384, U640},
    ParseBigIntError,
};
use curves::{Config, InvalidGeneratorError, P256, P384, P521};
use elliptic_curve::{Numeric, Point};
use modular::Widened;
use numeric::{FromBeBytes, FromStrRadix, Widen};
use sha::{
    sha1::Sha1, sha224::Sha224, sha256::Sha256, sha384::Sha384, sha512::Sha512, HashingAlgorithm,
};
use wasm_bindgen::prelude::*;

use crate::Ecdsa;

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Default)]
pub struct SignatureHex {
    pub r: String,
    pub s: String,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Default)]
pub struct PointHex {
    pub x: String,
    pub y: String,
}

#[wasm_bindgen]
#[derive(Debug)]
pub enum SigningError {
    ParseBigInt,
    MessageTooLong,
    InvalidPoint,
    NoInvK,
    ZeroingK,
}

impl From<ParseBigIntError> for SigningError {
    fn from(_: ParseBigIntError) -> Self {
        SigningError::ParseBigInt
    }
}

impl From<TryFromSliceError> for SigningError {
    fn from(_: TryFromSliceError) -> Self {
        SigningError::MessageTooLong
    }
}

impl From<crate::SigningError> for SigningError {
    fn from(value: crate::SigningError) -> Self {
        match value {
            crate::SigningError::InvalidPoint => SigningError::InvalidPoint,
            crate::SigningError::NoInvK => SigningError::NoInvK,
            crate::SigningError::ZeroingK => SigningError::ZeroingK,
        }
    }
}

#[wasm_bindgen]
pub enum EcdsaCustomError {
    ParseBigInt,
    InvalidGenerator,
}

impl From<InvalidGeneratorError> for EcdsaCustomError {
    fn from(_: InvalidGeneratorError) -> Self {
        EcdsaCustomError::InvalidGenerator
    }
}

impl From<ParseBigIntError> for EcdsaCustomError {
    fn from(_: ParseBigIntError) -> Self {
        EcdsaCustomError::ParseBigInt
    }
}

#[wasm_bindgen]
pub enum VerifyingError {
    ParseBigInt,
    MessageTooLong,
}

impl From<ParseBigIntError> for VerifyingError {
    fn from(_: ParseBigIntError) -> Self {
        VerifyingError::ParseBigInt
    }
}

impl From<TryFromSliceError> for VerifyingError {
    fn from(_: TryFromSliceError) -> Self {
        VerifyingError::MessageTooLong
    }
}

#[wasm_bindgen]
pub struct EcdsaCustom {
    ecdsa: Ecdsa<U640>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Default)]
pub struct SigningIntermediateValuesHex {
    pub hash: String,
    pub truncated_hash: String,
    pub generated_point: PointHex,
    pub signature: SignatureHex,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Default)]
pub struct VerifyingIntermediateValuesHex {
    pub hash: String,
    pub truncated_hash: String,
    pub u1: String,
    pub u2: String,
    pub generated_point: PointHex,
    pub valid: bool,
}
trait DynHashingAlgorithm {
    fn update(&mut self, data: &[u8]);
    fn digest(&self) -> Box<[u8]>;
}

macro_rules! impl_boxed_hashing_algorithm {
    ($algo:ty, $elem_type:ty, $len:expr) => {
        impl DynHashingAlgorithm for $algo {
            fn digest(&self) -> Box<[u8]> {
                let digest = HashingAlgorithm::digest(self);
                let bytes: Vec<u8> = digest.iter().flat_map(|&v| v.to_be_bytes()).collect();
                bytes.into_boxed_slice()
            }

            fn update(&mut self, data: &[u8]) {
                HashingAlgorithm::update(self, data);
            }
        }
    };
}

impl_boxed_hashing_algorithm!(Sha1, u32, 5);
impl_boxed_hashing_algorithm!(Sha224, u32, 7);
impl_boxed_hashing_algorithm!(Sha256, u32, 8);
impl_boxed_hashing_algorithm!(Sha384, u64, 5);
impl_boxed_hashing_algorithm!(Sha512, u64, 8);

#[wasm_bindgen]
pub enum HashingAlgorithmType {
    None,
    Sha1,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
}

impl HashingAlgorithmType {
    fn to_hashing_algorithm(self) -> Option<Box<dyn DynHashingAlgorithm>> {
        match self {
            HashingAlgorithmType::None => None,
            HashingAlgorithmType::Sha1 => Some(Box::new(Sha1::new())),
            HashingAlgorithmType::Sha224 => Some(Box::new(Sha224::new())),
            HashingAlgorithmType::Sha256 => Some(Box::new(Sha256::new())),
            HashingAlgorithmType::Sha384 => Some(Box::new(Sha384::new())),
            HashingAlgorithmType::Sha512 => Some(Box::new(Sha512::new())),
        }
    }
}

#[wasm_bindgen]
impl EcdsaCustom {
    pub fn new(
        p: &str,
        a: &str,
        b: &str,
        gx: &str,
        gy: &str,
        n: &str,
    ) -> Result<EcdsaCustom, EcdsaCustomError> {
        let p = U640::from_be_hex(p)?;
        let a = U640::from_be_hex(a)?;
        let b = U640::from_be_hex(b)?;
        let gx = U640::from_be_hex(gx)?;
        let gy = U640::from_be_hex(gy)?;
        let n = U640::from_be_hex(n)?;

        let config = Config::new(p, a, b, Point::new(gx, gy), n)?;

        Ok(Self {
            ecdsa: Ecdsa::new(config),
        })
    }

    pub fn sign(
        &mut self,
        k: &str,
        key: &str,
        message: &str,
        hashing_algorithm_type: HashingAlgorithmType,
    ) -> Result<SigningIntermediateValuesHex, SigningError> {
        sign(&self.ecdsa, k, key, message, hashing_algorithm_type)
    }

    pub fn verify(
        &self,
        x: &str,
        y: &str,
        r: &str,
        s: &str,
        message: &str,
        hashing_algorithm_type: HashingAlgorithmType,
    ) -> Result<VerifyingIntermediateValuesHex, VerifyingError> {
        verify(&self.ecdsa, x, y, r, s, message, hashing_algorithm_type)
    }
}

macro_rules! impl_ecdsa {
    ($name:ident, $curve:ident, $t:ty) => {
        #[wasm_bindgen]
        pub struct $name {
            ecdsa: Ecdsa<$t>,
        }

        #[wasm_bindgen]
        impl $name {
            pub fn new() -> Self {
                Self {
                    ecdsa: Ecdsa::new($curve),
                }
            }

            pub fn sign(
                &mut self,
                k: &str,
                key: &str,
                message: &str,
                hashing_algorithm_type: HashingAlgorithmType,
            ) -> Result<SigningIntermediateValuesHex, SigningError> {
                sign(&self.ecdsa, k, key, message, hashing_algorithm_type)
            }

            pub fn verify(
                &self,
                x: &str,
                y: &str,
                r: &str,
                s: &str,
                message: &str,
                hashing_algorithm_type: HashingAlgorithmType,
            ) -> Result<VerifyingIntermediateValuesHex, VerifyingError> {
                verify(&self.ecdsa, x, y, r, s, message, hashing_algorithm_type)
            }
        }
    };
}

impl_ecdsa!(EcdsaP256, P256, U256);
impl_ecdsa!(EcdsaP384, P384, U384);
impl_ecdsa!(EcdsaP521, P521, U640);

pub fn sign<T: Numeric, const N: usize>(
    ecdsa: &Ecdsa<T>,
    k: &str,
    key: &str,
    message: &str,
    hashing_algorithm_type: HashingAlgorithmType,
) -> Result<SigningIntermediateValuesHex, SigningError>
where
    T: FromBeBytes<Bytes = [u8; N]> + LowerHex + FromStrRadix<Error = ParseBigIntError>,
    <T as Widen>::Output: Widened<T>,
{
    console_error_panic_hook::set_once();
    let mut intermediate_values = SigningIntermediateValuesHex::default();
    let hasher = hashing_algorithm_type.to_hashing_algorithm();

    let message_bytes = message.as_bytes();
    let hash = match hasher {
        Some(mut hasher) => {
            hasher.update(message_bytes);
            let digest = hasher.digest();
            intermediate_values.hash = digest.iter().map(|byte| format!("{:02x}", byte)).collect();
            let mut truncated = [0u8; N];
            let start = N.saturating_sub(digest.len());
            truncated[start..].copy_from_slice(&digest[..min(N, digest.len())]);
            T::from_be_bytes(&truncated)
        }
        None => {
            intermediate_values.hash = message_bytes
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect();
            T::from_be_bytes(message_bytes.try_into()?)
        }
    };
    intermediate_values.truncated_hash = format!("{:x}", hash);

    let k = T::from_str_radix(k, 16)?;
    let key = T::from_str_radix(key, 16)?;

    let signing_intermediate_values = ecdsa.sign(&k, &key, &hash)?;

    intermediate_values.generated_point = PointHex {
        x: format!("{:x}", signing_intermediate_values.generated_point.x),
        y: format!("{:x}", signing_intermediate_values.generated_point.y),
    };

    intermediate_values.signature = SignatureHex {
        r: format!("{:x}", signing_intermediate_values.signature.0),
        s: format!("{:x}", signing_intermediate_values.signature.1),
    };

    Ok(intermediate_values)
}

pub fn verify<T: Numeric, const N: usize>(
    ecdsa: &Ecdsa<T>,
    x: &str,
    y: &str,
    r: &str,
    s: &str,
    message: &str,
    hashing_algorithm_type: HashingAlgorithmType,
) -> Result<VerifyingIntermediateValuesHex, VerifyingError>
where
    T: FromBeBytes<Bytes = [u8; N]> + LowerHex + FromStrRadix<Error = ParseBigIntError>,
    <T as Widen>::Output: Widened<T>,
{
    let mut intermediate_values = VerifyingIntermediateValuesHex::default();
    let key = Point::new(T::from_str_radix(x, 16)?, T::from_str_radix(y, 16)?);

    let hasher = hashing_algorithm_type.to_hashing_algorithm();

    let message_bytes = message.as_bytes();
    let hash = match hasher {
        Some(mut hasher) => {
            hasher.update(message_bytes);
            let digest = hasher.digest();
            intermediate_values.hash = digest.iter().map(|byte| format!("{:02x}", byte)).collect();
            let mut truncated = [0u8; N];
            let start = N.saturating_sub(digest.len());
            truncated[start..].copy_from_slice(&digest[..min(N, digest.len())]);
            T::from_be_bytes(&truncated)
        }
        None => {
            intermediate_values.hash = message_bytes
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect();
            T::from_be_bytes(message_bytes.try_into()?)
        }
    };

    let signature = (T::from_str_radix(r, 16)?, T::from_str_radix(s, 16)?);

    let verifying_intermediate_values = ecdsa.verify(&key, &hash, &signature);

    if let Some(u) = verifying_intermediate_values.u {
        intermediate_values.u1 = format!("{:x}", u.0);
        intermediate_values.u2 = format!("{:x}", u.1);
    }

    if let Some(generated_point) = verifying_intermediate_values.generated_point {
        intermediate_values.generated_point = PointHex {
            x: format!("{:x}", generated_point.x),
            y: format!("{:x}", generated_point.y),
        };
    }

    intermediate_values.valid = verifying_intermediate_values.valid;

    Ok(intermediate_values)
}

#[test]
fn test1() {
    let mut ecdsa = EcdsaP256::new();
    let intermediate_values = ecdsa.sign(
        "882905F1227FD620FBF2ABF21244F0BA83D0DC3A9103DBBEE43A1FB858109DB4",
        "C9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721",
        "sample",
        HashingAlgorithmType::Sha1,
    );

    assert_eq!("", intermediate_values.unwrap().hash)
}
