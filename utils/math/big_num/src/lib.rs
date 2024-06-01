#![no_std]
const RADIX: u64 = u32::MAX as u64 + 1;

#[derive(Debug, PartialEq, Copy, Clone)]
struct BigUInt<const N: usize> {
    limbs: [u32; N]
}

impl <const N: usize> core::str::FromStr for BigUInt<N> {
    type Err = core::num::IntErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl <const N: usize> core::ops::Add for BigUInt<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut w = [0u32; N];
        let mut k = 0;

        for j in 0..N {
            let sum = u64::from(self.limbs[j]) + u64::from(rhs.limbs[j]) + k;
            w[j] = (sum % RADIX) as u32; // This is guaranteed to never overflow because we are reducing by the radix

            if sum >= RADIX {
                if j == (N - 1) {
                    panic!("integer overflow");
                }
                k = 1;
            } else {
                k = 0;
            }
        }

        BigUInt { limbs: w }
    }
}

impl <const N: usize> core::ops::Mul for BigUInt<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut w = [0u32; N];
        let mut k = 0;
        
        for j in 0..N {
            if rhs.limbs[j] == 0 {continue};
            for i in 0..N {
                if i + j < N {
                    let t = u64::from(self.limbs[i]) * u64::from(rhs.limbs[j]) + u64::from(w[i + j]) + k;
                    w[i + j] = (t % RADIX) as u32;
                    k = t / RADIX;
                }
            }
        }

        if k > 0 {
            panic!("integer overflow");
        }

        BigUInt { limbs: w }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = BigUInt { limbs: [100, 200, 300] };
        let b = BigUInt { limbs: [400, 500, 600] };
        let result = a + b;
        assert_eq!(result.limbs, [500, 700, 900]);
    }

    #[test]
    fn add_with_carry() {
        let a = BigUInt { limbs: [u32::MAX, u32::MAX, u32::MAX, 0] };
        let b = BigUInt { limbs: [u32::MAX, u32::MAX, u32::MAX, 0] };
        let result = a + b;
        assert_eq!(result.limbs, [u32::MAX - 1, u32::MAX , u32::MAX, 1]); // Check for carry propagation
    }

    #[test]
    fn add_with_0() {
        let a = BigUInt { limbs: [100, 200, 300] };
        let b = BigUInt { limbs: [0, 0, 0] };
        let result = a + b;
        assert_eq!(result.limbs, [100, 200, 300]); // Adding zero should not change the value
    }

    #[test]
    fn add_with_carry_propagation() {
        let a = BigUInt { limbs: [u32::MAX, u32::MAX, u32::MAX, 0] };
        let b = BigUInt { limbs: [1, 0, 0, 0] };
        let result = a + b;
        assert_eq!(result.limbs, [0, 0, 0, 1]);
    }

    #[test]
    #[should_panic]
    fn panic_on_overflow() {
        let a = BigUInt { limbs: [u32::MAX, u32::MAX, u32::MAX] };
        let b = BigUInt { limbs: [1, 0, 0] };
        let _ = a + b;
    }

    #[test]
    fn mul_with_0() {
        let a = BigUInt { limbs: [u32::MAX, u32::MAX, u32::MAX, u32::MAX] };
        let b = BigUInt { limbs: [0, 0, 0, 0] };
        let result = a * b;
        assert_eq!(result.limbs, [0, 0, 0, 0]);
    }

    #[test]
    fn mul_with_1() {
        let a = BigUInt { limbs: [u32::MAX, u32::MAX, u32::MAX, u32::MAX] };
        let b = BigUInt { limbs: [1, 0, 0, 0] };
        let result = a * b;
        assert_eq!(result.limbs, [u32::MAX, u32::MAX, u32::MAX, u32::MAX]);
    }

    #[test]
    fn mul_double() {
        let a = BigUInt { limbs: [u32::MAX, u32::MAX, 0] };
        let b = BigUInt { limbs: [2, 0, 0] };
        let result = a * b;
        assert_eq!(result.limbs, [u32::MAX - 1, u32::MAX, 1]);
    }

    #[test]
    fn mul_single_digit() {
        let a = BigUInt { limbs: [12345] };
        let b = BigUInt { limbs: [6789] };
        let result = a * b;
        assert_eq!(result.limbs, [83810205]);
    }

    #[test]
    fn mul_square() {
        let a = BigUInt { limbs: [u32::MAX, u32::MAX, u32::MAX, 0, 0, 0] };
        let b = BigUInt { limbs: [u32::MAX, u32::MAX, u32::MAX, 0, 0, 0] };
        let result = a * b;
        assert_eq!(result.limbs, [1, 0, 0, u32::MAX - 1, u32::MAX, u32::MAX]);
    }


    #[test]
    fn mul_triple() {
        let a = BigUInt { limbs: [u32::MAX / 3, u32::MAX / 3, u32::MAX / 3] };
        let b = BigUInt { limbs: [3, 0, 0] };
        let result = a * b;
        assert_eq!(result.limbs, [u32::MAX, u32::MAX, u32::MAX]);
    }
}

