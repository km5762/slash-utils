use alloc::string::String;
use big_num::BigUint;
use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
use crate::{Ecdh, Steps};
use curves::Config;

#[wasm_bindgen]
pub enum CurveType {
    P256,
    P384,
    P521,
}

impl CurveType {
    fn compute_shared_secret(self, private_key_1: &str, private_key_2: &str) -> Option<Steps> {
        match self {
            CurveType::P256 => {
                let ecdh = Ecdh::new(curves::P256);
                ecdh.compute_shared_secret(
                    &BigUint::from_be_hex(&private_key_1).unwrap(),
                    &BigUint::from_be_hex(&private_key_2).unwrap(),
                )
            }
            CurveType::P384 => {
                let ecdh = Ecdh::new(curves::P384);
                ecdh.compute_shared_secret(
                    &BigUint::from_be_hex(&private_key_1).unwrap(),
                    &BigUint::from_be_hex(&private_key_2).unwrap(),
                )
            }
            CurveType::P521 => {
                let ecdh = Ecdh::new(curves::P521);
                ecdh.compute_shared_secret(
                    &BigUint::from_be_hex(&private_key_1).unwrap(),
                    &BigUint::from_be_hex(&private_key_2).unwrap(),
                )
            }
        }
    }
}

#[wasm_bindgen]
pub fn compute_shared_secret(private_key_1: &str, private_key_2: &str, curve: CurveType) -> Steps {
    console_error_panic_hook::set_once();
    curve
        .compute_shared_secret(private_key_1, private_key_2)
        .unwrap()
}
