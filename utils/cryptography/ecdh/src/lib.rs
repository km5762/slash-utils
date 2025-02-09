#![no_std]

mod p521;
pub mod wasm_adapter;

extern crate alloc;
use alloc::{format, string::String};
use core::fmt::LowerHex;

use curves::Config;
use elliptic_curve::Numeric;
use modular::Widened;
use numeric::Widen;
use steps::define_steps;

use wasm_bindgen::prelude::*;
pub struct Ecdh<T> {
    config: Config<T>,
}

define_steps! {
    GeneratePrivateKeys => {
        title: String::from("Generate Private Keys"),
        value_type: HexPair,
        children_types: (),
    },
    GeneratePublicKeys => {
        title: String::from("Generate Public Keys"),
        value_type: KeyPair,
        children_types: (),
    },
    ExchangeKeys => {
        title: String::from("Exchange Keys"),
        value_type: u8,
        children_types: (),
    },
    ComputeSharedSecret => {
        title: String::from("Compute Shared Secret"),
        value_type: HexPair,
        children_types: (),
    },
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Default)]
pub struct Steps {
    pub generate_private_keys: GeneratePrivateKeys,
    pub generate_public_keys: GeneratePublicKeys,
    pub exchange_keys: ExchangeKeys,
    pub compute_shared_secret: ComputeSharedSecret,
}

#[wasm_bindgen]
impl Steps {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Steps {
        Steps::default()
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Default)]
pub struct HexPair(pub String, pub String);

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Default)]
pub struct KeyPair(pub HexPair, pub HexPair);

impl<T: Numeric + Default + LowerHex> Ecdh<T>
where
    <T as Widen>::Output: Widened<T>,
{
    pub fn new(config: Config<T>) -> Self {
        Self { config }
    }

    pub fn compute_shared_secret(&self, private_key_1: &T, private_key_2: &T) -> Option<Steps> {
        let mut steps = Steps::default();
        steps.generate_private_keys = GeneratePrivateKeys::new(
            HexPair(
                format!("{:x}", private_key_1),
                format!("{:x}", private_key_2),
            ),
            GeneratePrivateKeysChildren(),
        );

        let g = &self.config.g;
        let curve = self.config.get_curve();
        let public_key_1 = curve.mul(g, private_key_1)?;
        let public_key_2 = curve.mul(g, private_key_2)?;
        steps.generate_public_keys = GeneratePublicKeys::new(
            KeyPair(
                HexPair(
                    format!("{:x}", public_key_1.x),
                    format!("{:x}", public_key_1.y),
                ),
                HexPair(
                    format!("{:x}", public_key_2.x),
                    format!("{:x}", public_key_2.y),
                ),
            ),
            GeneratePublicKeysChildren(),
        );

        let shared_point = curve.mul(&public_key_2, private_key_1)?;
        steps.compute_shared_secret = ComputeSharedSecret::new(
            HexPair(
                format!("{:x}", shared_point.x),
                format!("{:x}", shared_point.y),
            ),
            ComputeSharedSecretChildren(),
        );

        Some(steps)
    }
}

#[cfg(test)]
mod tests {

    pub struct TestVector<'a> {
        pub private_key1: &'a str,
        pub private_key2: &'a str,
        pub shared_secret: &'a str,
    }

    #[macro_export]
    macro_rules! test_ecdh_vector {
        ($vector:expr, $ecdh: expr) => {{
            let private_key_1 = big_num::BigUint::from_be_hex($vector.private_key1).unwrap();
            let private_key_2 = big_num::BigUint::from_be_hex($vector.private_key2).unwrap();

            let computed_secret = $ecdh
                .compute_shared_secret(&private_key_1, &private_key_2)
                .unwrap();

            assert_eq!(
                computed_secret.compute_shared_secret.value.0,
                $vector.shared_secret
            );
        }};
    }
}
