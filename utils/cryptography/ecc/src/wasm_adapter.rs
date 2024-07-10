extern crate wasm_bindgen;

use core::array::TryFromSliceError;

use alloc::{boxed::Box, string::String, vec::Vec};
use big_num::{types::U256, types::U384, BigUint};
use elliptic_curve::Point;
use numeric::{FromBeBytes, ToBeBytes};
use wasm_bindgen::prelude::*;

use crate::{
    configs::{P256, P384},
    Ecdsa,
};

#[wasm_bindgen]
pub struct Signature(Box<[u8]>, Box<[u8]>);

#[wasm_bindgen]
pub struct PublicKey(Box<[u8]>, Box<[u8]>);

#[wasm_bindgen]
pub enum SigningError {
    BytesLength,
    InvalidPoint,
    NoInvK,
    ZeroingK,
}

impl From<TryFromSliceError> for SigningError {
    fn from(_: TryFromSliceError) -> Self {
        SigningError::BytesLength
    }
}

impl From<crate::SigningError> for SigningError {
    fn from(_: crate::SigningError) -> Self {
        todo!()
    }
}

#[wasm_bindgen]
pub enum VerifyingError {
    BytesLength,
}

impl From<TryFromSliceError> for VerifyingError {
    fn from(_: TryFromSliceError) -> Self {
        VerifyingError::BytesLength
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
                &self,
                k: &[u8],
                key: &[u8],
                hash: &[u8],
            ) -> Result<Signature, SigningError> {
                let k = BigUint::from_be_bytes(k.try_into()?);
                let key = BigUint::from_be_bytes(key.try_into()?);
                let hash = BigUint::from_be_bytes(hash.try_into()?);

                let (r, s) = self.ecdsa.sign(&k, &key, &hash)?;

                Ok(Signature(
                    Box::new(r.to_be_bytes()),
                    Box::new(s.to_be_bytes()),
                ))
            }

            pub fn verify(
                &self,
                key: &PublicKey,
                hash: &[u8],
                signature: &Signature,
            ) -> Result<bool, VerifyingError> {
                let x_box: &[u8] = &key.0;
                let x_bytes: [u8; <$t>::BYTES] = x_box.try_into()?;

                let y_box: &[u8] = &key.1;
                let y_bytes: [u8; <$t>::BYTES] = y_box.try_into()?;

                let key = Point::new(
                    BigUint::from_be_bytes(&x_bytes),
                    BigUint::from_be_bytes(&y_bytes),
                );

                let hash = BigUint::from_be_bytes(hash.try_into()?);

                let r_box: &[u8] = &signature.0;
                let r_bytes: [u8; <$t>::BYTES] = r_box.try_into()?;

                let s_box: &[u8] = &signature.1;
                let s_bytes: [u8; <$t>::BYTES] = s_box.try_into()?;

                let signature = (
                    BigUint::from_be_bytes(&r_bytes),
                    BigUint::from_be_bytes(&s_bytes),
                );

                Ok(self.ecdsa.verify(&key, &hash, &signature))
            }
        }
    };
}

impl_ecdsa!(EcdsaP256, P256, U256);
impl_ecdsa!(EcdsaP384, P384, U384);
