extern crate wasm_bindgen;

use alloc::{format, string::String};
use big_num::{
    types::{U256, U384, U640},
    BigUint, ParseBigIntError,
};
use elliptic_curve::Point;
use wasm_bindgen::prelude::*;

use crate::{
    configs::{p521::P521, Config, InvalidGeneratorError, P256, P384},
    Ecdsa,
};

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct SignatureHex {
    pub r: String,
    pub s: String,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct PointHex {
    pub x: String,
    pub y: String,
}

#[wasm_bindgen]
pub enum SigningError {
    ParseBigIntError,
    InvalidPoint,
    NoInvK,
    ZeroingK,
}

impl From<ParseBigIntError> for SigningError {
    fn from(_: ParseBigIntError) -> Self {
        SigningError::ParseBigIntError
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
    ParseBigIntError,
    InvalidGenerator,
}

impl From<InvalidGeneratorError> for EcdsaCustomError {
    fn from(_: InvalidGeneratorError) -> Self {
        EcdsaCustomError::InvalidGenerator
    }
}

impl From<ParseBigIntError> for EcdsaCustomError {
    fn from(_: ParseBigIntError) -> Self {
        EcdsaCustomError::ParseBigIntError
    }
}

#[wasm_bindgen]
pub enum VerifyingError {
    ParseBigIntError,
}

impl From<ParseBigIntError> for VerifyingError {
    fn from(_: ParseBigIntError) -> Self {
        VerifyingError::ParseBigIntError
    }
}

#[wasm_bindgen]
pub struct EcdsaCustom {
    ecdsa: Ecdsa<U640>,
}

#[wasm_bindgen(getter_with_clone)]
pub struct SigningIntermediateValuesBytes {
    pub generated_point: PointHex,
    pub signature: SignatureHex,
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
        &self,
        k: &str,
        key: &str,
        hash: &str,
    ) -> Result<SigningIntermediateValuesBytes, SigningError> {
        let k = BigUint::from_be_hex(k)?;
        let key = BigUint::from_be_hex(key)?;
        let hash = BigUint::from_be_hex(hash)?;

        let intermediate_values = self.ecdsa.sign(&k, &key, &hash)?;

        Ok(SigningIntermediateValuesBytes {
            generated_point: PointHex {
                x: format!("{:X}", intermediate_values.generated_point.x),
                y: format!("{:X}", intermediate_values.generated_point.y),
            },
            signature: SignatureHex {
                r: format!("{:X}", intermediate_values.signature.0),
                s: format!("{:X}", intermediate_values.signature.1),
            },
        })
    }

    pub fn verify(
        &self,
        x: &str,
        y: &str,
        hash: &str,
        r: &str,
        s: &str,
    ) -> Result<bool, VerifyingError> {
        let key = Point::new(BigUint::from_be_hex(x)?, BigUint::from_be_hex(y)?);

        let hash = BigUint::from_be_hex(hash)?;

        let signature = (BigUint::from_be_hex(r)?, BigUint::from_be_hex(s)?);

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
                k: &str,
                key: &str,
                hash: &str,
            ) -> Result<SigningIntermediateValuesBytes, SigningError> {
                let k = BigUint::from_be_hex(k)?;
                let key = BigUint::from_be_hex(key)?;
                let hash = BigUint::from_be_hex(hash)?;

                let intermediate_values = self.ecdsa.sign(&k, &key, &hash)?;

                Ok(SigningIntermediateValuesBytes {
                    generated_point: PointHex {
                        x: format!("{:X}", intermediate_values.generated_point.x),
                        y: format!("{:X}", intermediate_values.generated_point.y),
                    },
                    signature: SignatureHex {
                        r: format!("{:X}", intermediate_values.signature.0),
                        s: format!("{:X}", intermediate_values.signature.1),
                    },
                })
            }

            pub fn verify(
                &self,
                x: &str,
                y: &str,
                hash: &str,
                r: &str,
                s: &str,
            ) -> Result<bool, VerifyingError> {
                let key = Point::new(BigUint::from_be_hex(x)?, BigUint::from_be_hex(y)?);

                let hash = BigUint::from_be_hex(hash)?;

                let signature = (BigUint::from_be_hex(r)?, BigUint::from_be_hex(s)?);

                Ok(self.ecdsa.verify(&key, &hash, &signature))
            }
        }
    };
}

impl_ecdsa!(EcdsaP256, P256, U256);
impl_ecdsa!(EcdsaP384, P384, U384);
impl_ecdsa!(EcdsaP521, P521, U640);
