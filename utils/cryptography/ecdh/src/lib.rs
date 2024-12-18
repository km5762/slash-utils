use curves::Config;
use elliptic_curve::{Numeric, Point};
use modular::Widened;
use numeric::Widen;

pub struct Ecdh<T> {
    config: Config<T>,
}

pub struct IntermediateValues<T> {
    public_key_1: T,
    public_key_2: T,
    shared_point: Point<T>,
}

impl<T: Numeric> Ecdh<T>
where
    <T as Widen>::Output: Widened<T>,
{
    pub fn new(config: Config<T>) -> Self {
        Self { config }
    }

    pub fn compute_shared_secret(&self, private_key_1: &T, private_key_2: &T) -> Option<T> {
        let g = &self.config.g;
        let curve = self.config.get_curve();
        let public_key_1 = curve.mul(g, private_key_1)?;
        let public_key_2 = curve.mul(g, private_key_2)?;

        let shared_point = curve.mul(&public_key_2, private_key_1)?;

        Some(shared_point.x)
    }
}

#[cfg(test)]
mod tests {
    use big_num::{types::U256, BigUint};
    use curves::P256;
    use numeric::FromBeBytes;

    use super::*;

    #[test]
    fn it_works() {
        let ecdh = Ecdh::new(P256);
        let private_key_1 = BigUint::from_be_hex(
            "5DCDF2A33538D7CF93A3680D4CB4E86A6814DA67AC5D8323EDBAAA59FAAA2DE4",
        )
        .unwrap();
        let private_key_2 = BigUint::from_be_hex(
            "8FED78431D82558D9006C3DEB06AB58D9DCCC97106875C6725D5F166255BA90A",
        )
        .unwrap();

        let shared = ecdh.compute_shared_secret(&private_key_1, &private_key_2);

        assert_eq!("", shared.unwrap().to_str_radix(16));
    }
}
