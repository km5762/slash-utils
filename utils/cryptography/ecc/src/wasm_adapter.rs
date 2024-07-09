extern crate wasm_bindgen;

use alloc::{boxed::Box, string::String, vec::Vec};
use big_num::{types::U256, BigUint};
use elliptic_curve::Point;
use numeric::FromBeBytes;
use wasm_bindgen::prelude::*;

use crate::{configs::P256, Ecdsa};

#[wasm_bindgen]
pub struct Signature(Box<[u8]>, Box<[u8]>);

#[wasm_bindgen]
pub struct PublicKey(Box<[u8]>, Box<[u8]>);

#[wasm_bindgen]
pub struct EcdsaP256 {
    _ecdsa_instance: Ecdsa<U256>,
}

#[wasm_bindgen]
impl EcdsaP256 {
    pub fn new(&self) -> Self {
        Self {
            _ecdsa_instance: Ecdsa::new(P256),
        }
    }

    pub fn sign(&self, k: &[u8], key: &[u8], hash: &[u8]) -> Result<Signature, SigningError> {
        let k = BigUint::from_be_bytes(k.try_into()?);
        let key = BigUint::from_be_bytes(key.try_into()?);
        let hash = BigUint::from_be_bytes(hash.try_into()?);

        self._ecdsa_instance.sign(&k, &key, &hash)
    }

    pub fn verify(
        &self,
        key: &PublicKey,
        hash: &[u8],
        signature: &Signature,
    ) -> Result<bool, VerifyingError> {
        let key = Point::new(
            BigUint::from_be_bytes(&key.0),
            BigUint::from_be_bytes(&key.1),
        );

        let hash = BigUint::from_be_bytes(hash.try_into()?);
        let signature = (
            BigUint::from_be_bytes(*signature.0.try_into()?),
            BigUint::from_be_bytes(*signature.1.try_into()?),
        );

        self._ecdsa_instance.verify(&key, &hash, &signature)
    }
}
