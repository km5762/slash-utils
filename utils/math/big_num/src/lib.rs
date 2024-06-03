#![no_std]

use alloc::string::String;
extern crate alloc;
const RADIX: u64 = u32::MAX as u64 + 1;

#[derive(Debug, PartialEq, Copy, Clone)]
struct BigUInt<const N: usize> {
    limbs: [u32; N]
}

impl<const N: usize> Default for BigUInt<N> {
    fn default() -> Self {
        BigUInt {
            limbs: [0; N],
        }
    }
}

impl <const N: usize> BigUInt<N> {
    fn from_str_radix(src: &str, radix: u32) -> Self {
        let mut num: BigUInt<N> = BigUInt::default();

        for char in src.chars() {
            let digit = char.to_digit(radix).expect("invalid char for given radix");
            num = num.mul_u32(radix).add_u32(digit);
        }

        num
    }

    fn to_str_radix(&self, radix: u32) -> String {
        let mut s = String::new();
        let mut num = *self;

        loop {
            let (quotient, remainder) = num.div_u32(radix);
            s.push(char::from_digit(remainder, radix).expect("asd"));
            num = quotient;

            if quotient == BigUInt::default() {break};
        }

        s.chars().rev().collect()
    }

    fn from_u32(n: u32) -> Self {
        let mut limbs = [0; N];
        limbs[0] = n;
        Self { limbs}
    }

    fn div_u32(&self, n: u32) -> (Self, u32) {
        let n64 = u64::from(n);
        let mut k = 0;
        let mut w = [0; N];

        for i in (0..N).rev() {
            let cur = u64::from(self.limbs[i]) + k * RADIX;
            w[i] = (cur / n64) as u32;
            k = cur % n64;
        }

        (BigUInt {limbs: w}, k as u32)
    }

    fn mul_u32(&self, n: u32) -> Self {
        let mut w = [0; N];
        let n64 = u64::from(n);
        let mut k = 0;

        for i in 0..N {
            let cur = k + u64::from(self.limbs[i]) * n64;
            w[i] = (cur % RADIX) as u32;
            k = cur / RADIX;
        }

        if k > 0 {
            panic!("integer overflow");
        }

        BigUInt {limbs: w}
    }

    fn add_u32(&self, n: u32) -> Self {
        let n64 = u64::from(n);
        let mut w = [0; N];
        let mut k = 0;

        for j in 0..N {
            let sum = u64::from(self.limbs[j]) + if j > 0 {0} else {n64} + k;
            w[j] = (sum % RADIX) as u32; 

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

impl <const N: usize> core::ops::Add for BigUInt<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut w = [0; N];
        let mut k = 0;

        for j in 0..N {
            let sum = u64::from(self.limbs[j]) + u64::from(rhs.limbs[j]) + k;
            w[j] = (sum % RADIX) as u32; // this is guaranteed to never overflow because we are reducing by the radix

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
        let mut w = [0; N];
        let mut k = 0;
        
        for j in 0..N {
            k = 0;
            if rhs.limbs[j] == 0 {continue};
            for i in 0..N {
                if i + j >= N {
                    if self.limbs[i] != 0 || rhs.limbs[i] != 0 {
                        panic!("integer overflow");
                    }
                } else {
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
    fn add_panic_on_overflow() {
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

    #[test]
    #[should_panic]
    fn mul_panic_on_overflow() {
        let a = BigUInt { limbs: [0, 0, 1, 0] };
        let b = BigUInt { limbs: [0, 0, 1, 0] };
        let _ = a * b;
    }

    #[test]
    fn mul_max_allowable_mul() {
        let a = BigUInt { limbs: [u32::MAX, u32::MAX, u32::MAX, 0] };
        let b = BigUInt { limbs: [0, 1, 0, 0] };
        let result = a * b;
        assert_eq!(result.limbs, [0, u32::MAX, u32::MAX, u32::MAX]);
    }

    #[test]
    fn div_u32_by_1() {
        let a = BigUInt { limbs: [12345, 67890, 54321] };
        let (quotient, remainder) = a.div_u32(1);
        assert_eq!(quotient.limbs, [12345, 67890, 54321]);
        assert_eq!(remainder, 0);
    }

    #[test]
    fn div_u32_by_larger_number() {
        let a: BigUInt<4> = BigUInt::from_str_radix("1002045585119561883070521", 10);
        let (quotient, remainder) = a.div_u32(u32::MAX);
        assert_eq!(quotient.to_str_radix(10), "233306918608227");
        assert_eq!(remainder, 134556);
    }

    #[test]
    fn div_u32_by_smaller_number() {
        let a = BigUInt { limbs: [123456789, 987654321] };
        let (quotient, remainder) = a.div_u32(12345);
        assert_eq!(quotient.to_str_radix(10), "343616282589837");
        assert_eq!(remainder, 5040);
    }

    #[test]
    fn div_u32_by_power_of_2() {
        let a = BigUInt { limbs: [123456789, 987654321] };
        let (quotient, remainder) = a.div_u32(8);
        assert_eq!(quotient.to_str_radix(10), "530242876071442850");
        assert_eq!(remainder, 5);
    }

    #[test]
    fn div_u32_by_u32_max() {
        let a = BigUInt { limbs: [u32::MAX, u32::MAX, u32::MAX] };
        let (quotient, remainder) = a.div_u32(u32::MAX);
        assert_eq!(quotient.to_str_radix(10), "18446744078004518913");
        assert_eq!(remainder, 0);
    }

    #[test]
    fn div_u32_large_number() {
        let a = BigUInt { limbs: [0, 0, u32::MAX] };
        let (quotient, remainder) = a.div_u32(2);
        assert_eq!(quotient.to_str_radix(10), "39614081247908796759917199360");
        assert_eq!(remainder, 0);
    }

    #[test]
    fn div_u32_with_remainder() {
        let a = BigUInt { limbs: [1, 2, 3] };
        let (quotient, remainder) = a.div_u32(2);
        assert_eq!(quotient.limbs, [0, 2147483649, 1]);
        assert_eq!(remainder, 1);
    }

    #[test]
    fn div_u32_complex() {
        let a = BigUInt { limbs: [1, 1, 1, 1] };
        let (quotient, remainder) = a.div_u32(2);
        assert_eq!(quotient.limbs, [2147483648, 2147483648, 2147483648, 0]);
        assert_eq!(remainder, 1);
    }

    #[test]
    fn to_and_from_str_radix_10() {
        let src = "12345678910111213141516171819202122232425262728293031323334353637383940414243444546474849";
        let num: BigUInt<20> = BigUInt::from_str_radix(src, 10);
        let binding = num.to_str_radix(10);
        let from = binding.as_str();
        assert_eq!(src, from);
    }

    #[test]
    fn to_and_from_str_max_4096_hex() {
        let src = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
        let num: BigUInt<128> = BigUInt::from_str_radix(src, 16);
        let binding = num.to_str_radix(16);
        let from = binding.as_str();
        assert_eq!(src, from);
    }
}

