extern crate wasm_bindgen;

use core::array::TryFromSliceError;

use alloc::boxed::Box;
use big_num::{types::U256, types::U384, types::U640, BigUint};
use elliptic_curve::Point;
use numeric::{FromBeBytes, ToBeBytes};
use wasm_bindgen::prelude::*;

use crate::{
    configs::{p521::P521, Config, InvalidGeneratorError, P256, P384},
    Ecdsa,
};

#[wasm_bindgen(getter_with_clone)]
pub struct Signature {
    pub r: Box<[u8]>,
    pub s: Box<[u8]>,
}

#[wasm_bindgen(getter_with_clone)]
pub struct PublicKey {
    pub x: Box<[u8]>,
    pub y: Box<[u8]>,
}

#[wasm_bindgen]
#[derive(Debug)]
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
    fn from(value: crate::SigningError) -> Self {
        match value {
            crate::SigningError::InvalidPoint => SigningError::InvalidPoint,
            crate::SigningError::NoInvK => SigningError::NoInvK,
            crate::SigningError::ZeroingK => SigningError::ZeroingK,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct BytesLengthError;

impl From<TryFromSliceError> for BytesLengthError {
    fn from(_: TryFromSliceError) -> Self {
        BytesLengthError {}
    }
}

#[wasm_bindgen]
pub enum EcdsaCustomError {
    BytesLength,
    InvalidGenerator,
}

impl From<InvalidGeneratorError> for EcdsaCustomError {
    fn from(_: InvalidGeneratorError) -> Self {
        EcdsaCustomError::InvalidGenerator
    }
}

impl From<TryFromSliceError> for EcdsaCustomError {
    fn from(_: TryFromSliceError) -> Self {
        EcdsaCustomError::BytesLength
    }
}

#[wasm_bindgen]
pub struct EcdsaCustom {
    ecdsa: Ecdsa<U640>,
}

#[wasm_bindgen]
impl EcdsaCustom {
    pub fn new(
        p: &[u8],
        a: &[u8],
        b: &[u8],
        gx: &[u8],
        gy: &[u8],
        n: &[u8],
    ) -> Result<EcdsaCustom, EcdsaCustomError> {
        let p = U640::from_be_bytes(p.try_into()?);
        let a = U640::from_be_bytes(a.try_into()?);
        let b = U640::from_be_bytes(b.try_into()?);
        let gx = U640::from_be_bytes(gx.try_into()?);
        let gy = U640::from_be_bytes(gy.try_into()?);
        let n = U640::from_be_bytes(n.try_into()?);

        let config = Config::new(p, a, b, Point::new(gx, gy), n)?;

        Ok(Self {
            ecdsa: Ecdsa::new(config),
        })
    }

    pub fn sign(&self, k: &[u8], key: &[u8], hash: &[u8]) -> Result<Signature, SigningError> {
        let k = BigUint::from_be_bytes(k.try_into()?);
        let key = BigUint::from_be_bytes(key.try_into()?);
        let hash = BigUint::from_be_bytes(hash.try_into()?);

        let (r, s) = self.ecdsa.sign(&k, &key, &hash)?;

        Ok(Signature {
            r: Box::new(r.to_be_bytes()),
            s: Box::new(s.to_be_bytes()),
        })
    }

    pub fn verify(
        &self,
        key: &PublicKey,
        hash: &[u8],
        signature: &Signature,
    ) -> Result<bool, BytesLengthError> {
        let x_box: &[u8] = &key.x;
        let x_bytes: [u8; U640::BYTES] = x_box.try_into()?;

        let y_box: &[u8] = &key.y;
        let y_bytes: [u8; U640::BYTES] = y_box.try_into()?;

        let key = Point::new(
            BigUint::from_be_bytes(&x_bytes),
            BigUint::from_be_bytes(&y_bytes),
        );

        let hash = BigUint::from_be_bytes(hash.try_into()?);

        let r_box: &[u8] = &signature.r;
        let r_bytes: [u8; U640::BYTES] = r_box.try_into()?;

        let s_box: &[u8] = &signature.s;
        let s_bytes: [u8; U640::BYTES] = s_box.try_into()?;

        let signature = (
            BigUint::from_be_bytes(&r_bytes),
            BigUint::from_be_bytes(&s_bytes),
        );

        Ok(self.ecdsa.verify(&key, &hash, &signature))
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

                Ok(Signature {
                    r: Box::new(r.to_be_bytes()),
                    s: Box::new(s.to_be_bytes()),
                })
            }

            pub fn verify(
                &self,
                key: &PublicKey,
                hash: &[u8],
                signature: &Signature,
            ) -> Result<bool, BytesLengthError> {
                let x_box: &[u8] = &key.x;
                let x_bytes: [u8; <$t>::BYTES] = x_box.try_into()?;

                let y_box: &[u8] = &key.y;
                let y_bytes: [u8; <$t>::BYTES] = y_box.try_into()?;

                let key = Point::new(
                    BigUint::from_be_bytes(&x_bytes),
                    BigUint::from_be_bytes(&y_bytes),
                );

                let hash = BigUint::from_be_bytes(hash.try_into()?);

                let r_box: &[u8] = &signature.r;
                let r_bytes: [u8; <$t>::BYTES] = r_box.try_into()?;

                let s_box: &[u8] = &signature.s;
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
impl_ecdsa!(EcdsaP521, P521, U640);
