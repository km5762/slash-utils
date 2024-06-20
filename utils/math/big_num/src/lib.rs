#![no_std]

use alloc::string::String;
use numeric::{CheckedAdd, CheckedMul, CheckedSub, LeadingZeros, One, RemEuclid, Zero};
extern crate alloc;
const RADIX: u64 = u32::MAX as u64 + 1;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BigUInt<const N: usize> {
    limbs: [u32; N],
}

impl<const N: usize> Default for BigUInt<N> {
    fn default() -> Self {
        BigUInt { limbs: [0; N] }
    }
}

impl<const N: usize> BigUInt<N> {
    fn new(limbs: [u32; N]) -> Self {
        BigUInt { limbs }
    }

    fn digits(&self) -> usize {
        let mut digits = 0;

        for i in (0..N).rev() {
            if self.limbs[i] > 0 {
                digits = i + 1;
                break;
            }
        }

        digits
    }

    fn from_u32(n: u32) -> Self {
        let mut limbs = [0; N];
        limbs[0] = n;

        BigUInt::new(limbs)
    }

    pub fn from_str_radix(src: &str, radix: u32) -> Self {
        let mut num: BigUInt<N> = BigUInt::default();

        for char in src.chars() {
            let digit = char.to_digit(radix).expect("invalid char for given radix");
            num = num.mul_u32(radix).add_u32(digit);
        }

        num
    }

    pub fn to_str_radix(&self, radix: u32) -> String {
        let mut s = String::new();
        let mut num = *self;

        loop {
            let (quotient, remainder) = num.div_u32(radix);
            s.push(char::from_digit(remainder, radix).expect("asd"));
            num = quotient;

            if quotient == BigUInt::default() {
                break;
            };
        }

        s.chars().rev().collect()
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

        (BigUInt::new(w), k as u32)
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

        BigUInt::new(w)
    }

    fn add_u32(&self, n: u32) -> Self {
        let n64 = u64::from(n);
        let mut w = [0; N];
        let mut k = 0;

        for j in 0..N {
            let sum = u64::from(self.limbs[j]) + if j > 0 { 0 } else { n64 } + k;
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

        BigUInt::new(w)
    }

    fn shl_limb(&self, index: usize, shift: u32) -> u32 {
        let limb = self.limbs[index];
        if shift == 0 {
            return limb;
        };
        let carry = if index == 0 {
            0
        } else {
            self.limbs[index - 1] >> (32 - shift)
        };
        (limb << shift) | carry
    }

    fn div_rem(&self, rhs: Self) -> (Self, Self) {
        let mut dividend = *self;
        let divisor = rhs;
        let dividend_digits = dividend.digits();
        let divisor_digits = divisor.digits();

        if divisor_digits == 0 {
            panic!("attempt to divide by zero")
        };
        if dividend_digits == 0 {
            return (BigUInt::default(), BigUInt::default());
        };
        if divisor_digits == 1 {
            let (quotient, remainder) = self.div_u32(divisor.limbs[0]);
            return (quotient, BigUInt::from_u32(remainder));
        }

        let shift = rhs.limbs[divisor_digits - 1].leading_zeros();
        let mut quotient: BigUInt<N> = BigUInt::default();

        let divisor_msd = divisor.shl_limb(divisor_digits - 1, shift);
        let divisor_smsd = if divisor_digits == 1 {
            0
        } else {
            divisor.shl_limb(divisor_digits - 2, shift)
        };

        for i in (0..=(dividend_digits - divisor_digits)).rev() {
            let dividend_msd = if i == (dividend_digits - divisor_digits) {
                if shift == 0 {
                    0
                } else {
                    dividend.limbs[dividend_digits - 1] >> (32 - shift)
                }
            } else {
                dividend.shl_limb(divisor_digits + i, shift)
            };
            let dividend_smsd = dividend.shl_limb(divisor_digits + i - 1, shift);
            let dividend_tmsd = if divisor_digits == 1 {
                0
            } else {
                dividend.shl_limb(divisor_digits + i - 2, shift)
            };

            let dividend_msd_combined =
                (u64::from(dividend_msd) * RADIX) + (u64::from(dividend_smsd));
            let mut q_estimate = dividend_msd_combined / u64::from(divisor_msd);
            let mut r_estimate = dividend_msd_combined % u64::from(divisor_msd);

            loop {
                if q_estimate >= RADIX
                    || ((q_estimate * u64::from(divisor_smsd))
                        > ((r_estimate * RADIX) | u64::from(dividend_tmsd)))
                {
                    q_estimate -= 1;
                    r_estimate += u64::from(divisor_msd);
                    if r_estimate < RADIX {
                        continue;
                    };
                }
                break;
            }

            // Based on previous loop, q_estimate must fit within 32 bits now
            let mut borrow = 0;
            let mut carry = 0;
            for j in 0..divisor_digits {
                let product = u64::from(divisor.limbs[j]) * q_estimate + carry;
                carry = product >> 32;
                let sub = i64::from(dividend.limbs[i + j]) - (product as u32 as i64) - borrow;
                borrow = if sub < 0 { 1 } else { 0 };
                dividend.limbs[i + j] = sub as u32;
            }

            if i + divisor_digits < N {
                let sub = i64::from(dividend.limbs[i + divisor_digits]) - carry as i64 - borrow;
                borrow = if sub < 0 { 1 } else { 0 };
                dividend.limbs[i + divisor_digits] = sub as u32;
            }

            if borrow != 0 {
                q_estimate -= 1;
                carry = 0;
                for j in 0..divisor_digits {
                    let add =
                        u64::from(dividend.limbs[i + j]) + u64::from(divisor.limbs[j]) + carry;
                    carry = add >> 32;
                    dividend.limbs[i + j] = add as u32;
                }
                if i + divisor_digits < N {
                    dividend.limbs[i + divisor_digits] =
                        (u64::from(dividend.limbs[i + divisor_digits]) + carry) as u32;
                }
            }

            quotient.limbs[i] = q_estimate as u32;
        }

        (quotient, dividend)
    }
}

impl<const N: usize> core::ops::Add for BigUInt<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(&rhs).expect("integer overflow")
    }
}

impl<const N: usize> core::ops::Sub for BigUInt<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(&rhs).expect("integer overflow")
    }
}

impl<const N: usize> core::cmp::PartialOrd for BigUInt<N> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        for i in (0..N).rev() {
            if self.limbs[i] > other.limbs[i] {
                return Some(core::cmp::Ordering::Greater);
            } else if self.limbs[i] < other.limbs[i] {
                return Some(core::cmp::Ordering::Less);
            }
        }
        Some(core::cmp::Ordering::Equal)
    }
}

impl<const N: usize> core::ops::Div for BigUInt<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.div_rem(rhs).0
    }
}

impl<const N: usize> core::ops::Rem for BigUInt<N> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        self.div_rem(rhs).1
    }
}

impl<const N: usize> core::ops::Mul for BigUInt<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.checked_mul(&rhs).expect("integer overflow")
    }
}

impl<const N: usize> core::ops::Shr<usize> for BigUInt<N> {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        let mut carry = 0;
        let mut limbs = [0; N];

        for i in (0..N).rev() {
            limbs[i] = (self.limbs[i] << rhs) | carry;
            carry = self.limbs[i] & ((1 << rhs) - 1);
        }

        BigUInt::new(limbs)
    }
}

impl<const N: usize> core::ops::BitAnd for BigUInt<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut limbs = [0; N];

        for i in 0..N {
            limbs[i] = self.limbs[i] & rhs.limbs[i];
        }

        BigUInt::new(limbs)
    }
}

impl<const N: usize> One for BigUInt<N> {
    fn one() -> Self {
        let mut limbs = [0; N];
        limbs[0] = 1;

        BigUInt::new(limbs)
    }
}

impl<const N: usize> Zero for BigUInt<N> {
    fn zero() -> Self {
        BigUInt::default()
    }
}

impl<const N: usize> RemEuclid for BigUInt<N> {
    fn rem_euclid(&self, v: &Self) -> Self {
        *self % *v
    }
}

impl<const N: usize> CheckedAdd for BigUInt<N> {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        let mut w = [0; N];
        let mut k = 0;

        for j in 0..N {
            let sum = u64::from(self.limbs[j]) + u64::from(v.limbs[j]) + k;
            w[j] = (sum % RADIX) as u32;
            if sum >= RADIX {
                if j == (N - 1) {
                    return None;
                }
                k = 1;
            } else {
                k = 0;
            }
        }

        Some(BigUInt::new(w))
    }
}

impl<const N: usize> CheckedSub for BigUInt<N> {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        let mut w = [0; N];
        let mut k = 0;

        for j in 0..N {
            let difference = i64::from(self.limbs[j]) - i64::from(v.limbs[j]) + k;
            w[j] = (difference % RADIX as i64) as u32;
            if difference < 0 {
                if j == (N - 1) {
                    return None;
                }
                k = -1;
            } else {
                k = 0;
            }
        }

        Some(BigUInt::new(w))
    }
}

impl<const N: usize> CheckedMul for BigUInt<N> {
    fn checked_mul(&self, rhs: &Self) -> Option<Self> {
        let mut w = [0; N];
        let mut k = 0;

        for j in 0..N {
            k = 0;
            if rhs.limbs[j] == 0 {
                continue;
            };
            for i in 0..N {
                if i + j >= N {
                    if self.limbs[i] != 0 || rhs.limbs[i] != 0 {
                        return None;
                    }
                } else {
                    let t = u64::from(self.limbs[i]) * u64::from(rhs.limbs[j])
                        + u64::from(w[i + j])
                        + k;
                    w[i + j] = (t % RADIX) as u32;
                    k = t / RADIX;
                }
            }
        }

        if k > 0 {
            return None;
        }

        Some(BigUInt::new(w))
    }
}

impl<const N: usize> LeadingZeros for BigUInt<N> {
    fn leading_zeros(&self) -> u32 {
        let mut zeros = 0;

        for i in (0..N).rev() {
            let limb_zeros = self.limbs[i].leading_zeros();
            zeros += limb_zeros;

            if limb_zeros < 32 {
                break;
            }
        }

        zeros
    }
}

impl<const N: usize> From<u8> for BigUInt<N> {
    fn from(value: u8) -> Self {
        let mut limbs = [0; N];
        limbs[0] = value as u32;

        BigUInt::new(limbs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = BigUInt::new([100, 200, 300]);
        let b = BigUInt::new([400, 500, 600]);
        let result = a + b;
        assert_eq!(result.limbs, [500, 700, 900]);
    }

    #[test]
    fn add_with_carry() {
        let a = BigUInt::new([u32::MAX, u32::MAX, u32::MAX, 0]);
        let b = BigUInt::new([u32::MAX, u32::MAX, u32::MAX, 0]);
        let result = a + b;
        assert_eq!(result.limbs, [u32::MAX - 1, u32::MAX, u32::MAX, 1]); // Check for carry propagation
    }

    #[test]
    fn add_with_0() {
        let a = BigUInt::new([100, 200, 300]);
        let b = BigUInt::new([0, 0, 0]);
        let result = a + b;
        assert_eq!(result.limbs, [100, 200, 300]); // Adding zero should not change the value
    }

    #[test]
    fn add_with_carry_propagation() {
        let a = BigUInt::new([u32::MAX, u32::MAX, u32::MAX, 0]);
        let b = BigUInt::new([1, 0, 0, 0]);
        let result = a + b;
        assert_eq!(result.limbs, [0, 0, 0, 1]);
    }

    #[test]
    #[should_panic]
    fn add_panic_on_overflow() {
        let a = BigUInt::new([u32::MAX, u32::MAX, u32::MAX]);
        let b = BigUInt::new([1, 0, 0]);
        let _ = a + b;
    }

    #[test]
    fn mul_with_0() {
        let a = BigUInt::new([u32::MAX, u32::MAX, u32::MAX, u32::MAX]);
        let b = BigUInt::new([0, 0, 0, 0]);
        let result = a * b;
        assert_eq!(result.limbs, [0, 0, 0, 0]);
    }

    #[test]
    fn mul_with_1() {
        let a = BigUInt::new([u32::MAX, u32::MAX, u32::MAX, u32::MAX]);
        let b = BigUInt::new([1, 0, 0, 0]);
        let result = a * b;
        assert_eq!(result.limbs, [u32::MAX, u32::MAX, u32::MAX, u32::MAX]);
    }

    #[test]
    fn mul_double() {
        let a = BigUInt::new([u32::MAX, u32::MAX, 0]);
        let b = BigUInt::new([2, 0, 0]);
        let result = a * b;
        assert_eq!(result.limbs, [u32::MAX - 1, u32::MAX, 1]);
    }

    #[test]
    fn mul_single_digit() {
        let a = BigUInt::new([12345]);
        let b = BigUInt::new([6789]);
        let result = a * b;
        assert_eq!(result.limbs, [83810205]);
    }

    #[test]
    fn mul_square() {
        let a = BigUInt::new([u32::MAX, u32::MAX, u32::MAX, 0, 0, 0]);
        let b = BigUInt::new([u32::MAX, u32::MAX, u32::MAX, 0, 0, 0]);
        let result = a * b;
        assert_eq!(result.limbs, [1, 0, 0, u32::MAX - 1, u32::MAX, u32::MAX]);
    }

    #[test]
    fn mul_triple() {
        let a = BigUInt::new([u32::MAX / 3, u32::MAX / 3, u32::MAX / 3]);
        let b = BigUInt::new([3, 0, 0]);
        let result = a * b;
        assert_eq!(result.limbs, [u32::MAX, u32::MAX, u32::MAX]);
    }

    #[test]
    #[should_panic]
    fn mul_panic_on_overflow() {
        let a = BigUInt::new([0, 0, 1, 0]);
        let b = BigUInt::new([0, 0, 1, 0]);
        let _ = a * b;
    }

    #[test]
    fn mul_max_allowable_mul() {
        let a = BigUInt::new([u32::MAX, u32::MAX, u32::MAX, 0]);
        let b = BigUInt::new([0, 1, 0, 0]);
        let result = a * b;
        assert_eq!(result.limbs, [0, u32::MAX, u32::MAX, u32::MAX]);
    }

    #[test]
    fn div_u32_by_1() {
        let a = BigUInt::new([12345, 67890, 54321]);
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
        let a = BigUInt::new([123456789, 987654321]);
        let (quotient, remainder) = a.div_u32(12345);
        assert_eq!(quotient.to_str_radix(10), "343616282589837");
        assert_eq!(remainder, 5040);
    }

    #[test]
    fn div_u32_by_power_of_2() {
        let a = BigUInt::new([123456789, 987654321]);
        let (quotient, remainder) = a.div_u32(8);
        assert_eq!(quotient.to_str_radix(10), "530242876071442850");
        assert_eq!(remainder, 5);
    }

    #[test]
    fn div_u32_by_u32_max() {
        let a = BigUInt::new([u32::MAX, u32::MAX, u32::MAX]);
        let (quotient, remainder) = a.div_u32(u32::MAX);
        assert_eq!(quotient.to_str_radix(10), "18446744078004518913");
        assert_eq!(remainder, 0);
    }

    #[test]
    fn div_u32_large_number() {
        let a = BigUInt::new([0, 0, u32::MAX]);
        let (quotient, remainder) = a.div_u32(2);
        assert_eq!(quotient.to_str_radix(10), "39614081247908796759917199360");
        assert_eq!(remainder, 0);
    }

    #[test]
    fn div_u32_with_remainder() {
        let a = BigUInt::new([1, 2, 3]);
        let (quotient, remainder) = a.div_u32(2);
        assert_eq!(quotient.limbs, [0, 2147483649, 1]);
        assert_eq!(remainder, 1);
    }

    #[test]
    fn div_u32_complex() {
        let a = BigUInt::new([1, 1, 1, 1]);
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

    #[test]
    #[should_panic]
    fn sub_overflow() {
        let a = BigUInt::new([0, u32::MAX, u32::MAX]);
        let b = BigUInt::new([1, u32::MAX, u32::MAX]);
        let result = a - b;
        assert_eq!(result, BigUInt::default());
    }

    #[test]
    fn sub_to_zero() {
        let a = BigUInt::new([1, u32::MAX, u32::MAX]);
        let b = BigUInt::new([1, u32::MAX, u32::MAX]);
        let result = a - b;
        assert_eq!(result, BigUInt::default());
    }

    #[test]
    fn sub_with_borrow() {
        let a = BigUInt::new([0, 0, 1]);
        let b = BigUInt::new([1, 0, 0]);
        let result = a - b;
        assert_eq!(
            result,
            BigUInt {
                limbs: [u32::MAX, u32::MAX, 0]
            }
        );
    }

    #[test]
    fn sub_with_borrow_to_zero() {
        let a = BigUInt::new([0, 0, 1]);
        let b = BigUInt::new([1, 1, 0]);
        let result = a - b;
        assert_eq!(
            result,
            BigUInt {
                limbs: [u32::MAX, u32::MAX - 1, 0]
            }
        );
    }

    const TEST: [u32; 303] = [
        // m, n, u...,          v...,          cq...,  cr....
        1, 1, 3, 0, 1, 1, // Error, divide by 0.
        1, 2, 7, 1, 3, 0, 7, 0, // Error, n > m.
        2, 2, 0, 0, 1, 0, 0, 0, 0, // Error, incorrect remainder cr.
        1, 1, 3, 2, 1, 1, 1, 1, 3, 3, 1, 0, 1, 1, 3, 4, 0, 3, 1, 1, 0, 0xffffffff, 0, 0, 1, 1,
        0xffffffff, 1, 0xffffffff, 0, 1, 1, 0xffffffff, 0xffffffff, 1, 0, 1, 1, 0xffffffff, 3,
        0x55555555, 0, 2, 1, 0xffffffff, 0xffffffff, 1, 0xffffffff, 0xffffffff, 0, 2, 1,
        0xffffffff, 0xffffffff, 0xffffffff, 1, 1, 0, 2, 1, 0xffffffff, 0xfffffffe, 0xffffffff,
        0xffffffff, 0, 0xfffffffe, 2, 1, 0x00005678, 0x00001234, 0x00009abc, 0x1e1dba76, 0, 0x6bd0,
        2, 2, 0, 0, 0, 1, 0, 0, 0, 2, 2, 0, 7, 0, 3, 2, 0, 1, 2, 2, 5, 7, 0, 3, 2, 5, 1, 2, 2, 0,
        6, 0, 2, 3, 0, 0, 1, 1, 0x80000000, 0x40000001, 0x00000001, 0x3fffffff, 2, 1, 0x00000000,
        0x80000000, 0x40000001, 0xfffffff8, 0x00000001, 0x00000008, 2, 2, 0x00000000, 0x80000000,
        0x00000001, 0x40000000, 0x00000001, 0xffffffff, 0x3fffffff, 2, 2, 0x0000789a, 0x0000bcde,
        0x0000789a, 0x0000bcde, 1, 0, 0, 2, 2, 0x0000789b, 0x0000bcde, 0x0000789a, 0x0000bcde, 1,
        1, 0, 2, 2, 0x00007899, 0x0000bcde, 0x0000789a, 0x0000bcde, 0, 0x00007899, 0x0000bcde, 2,
        2, 0x0000ffff, 0x0000ffff, 0x0000ffff, 0x0000ffff, 1, 0, 0, 2, 2, 0x0000ffff, 0x0000ffff,
        0x00000000, 0x00000001, 0x0000ffff, 0x0000ffff, 0, 3, 2, 0x000089ab, 0x00004567,
        0x00000123, 0x00000000, 0x00000001, 0x00004567, 0x00000123, 0x000089ab, 0, 3, 2,
        0x00000000, 0x0000fffe, 0x00008000, 0x0000ffff, 0x00008000, 0xffffffff, 0x00000000,
        0x0000ffff, 0x00007fff, // Shows that first qhat can = b + 1.
        3, 3, 0x00000003, 0x00000000, 0x80000000, 0x00000001, 0x00000000, 0x20000000, 0x00000003,
        0, 0, 0x20000000, // Adding back step req'd.
        3, 3, 0x00000003, 0x00000000, 0x00008000, 0x00000001, 0x00000000, 0x00002000, 0x00000003,
        0, 0, 0x00002000, // Adding back step req'd.
        4, 3, 0, 0, 0x00008000, 0x00007fff, 1, 0, 0x00008000, 0xfffe0000, 0, 0x00020000,
        0xffffffff, 0x00007fff, // Add back req'd.
        4, 3, 0, 0x0000fffe, 0, 0x00008000, 0x0000ffff, 0, 0x00008000, 0xffffffff, 0, 0x0000ffff,
        0xffffffff, 0x00007fff, // Shows that mult-sub quantity cannot be treated as signed.
        4, 3, 0, 0xfffffffe, 0, 0x80000000, 0x0000ffff, 0, 0x80000000, 0x00000000, 1, 0x00000000,
        0xfffeffff, 0x00000000, // Shows that mult-sub quantity cannot be treated as signed.
        4, 3, 0, 0xfffffffe, 0, 0x80000000, 0xffffffff, 0, 0x80000000, 0xffffffff, 0, 0xffffffff,
        0xffffffff, 0x7fffffff, // Shows that mult-sub quantity cannot be treated as signed.
    ];

    #[test]
    fn div_tests() {
        let mut j = 23;

        while j < TEST.len() {
            let m = TEST[j] as usize;
            j += 1;
            let n = TEST[j] as usize;
            j += 1;

            let mut dividend = [0; 7];
            let mut last = 0;
            let bound = j + m;

            while j < bound {
                dividend[last] = TEST[j];
                last += 1;
                j += 1;
            }

            let mut divisor = [0; 7];
            let mut last = 0;
            let bound = j + n;

            while j < bound {
                divisor[last] = TEST[j];
                last += 1;
                j += 1;
            }

            let mut quotient = [0; 7];
            let mut last = 0;
            let bound = j + core::cmp::max(m - n + 1, 1);

            while j < bound {
                quotient[last] = TEST[j];
                last += 1;
                j += 1;
            }

            let mut remainder = [0; 7];
            let mut last = 0;
            let bound = j + n;

            while j < bound {
                remainder[last] = TEST[j];
                last += 1;
                j += 1;
            }

            let big_dividend = BigUInt::new(dividend);
            let big_divisor = BigUInt::new(divisor);
            let big_quotient = BigUInt::new(quotient);
            let big_remainder = BigUInt::new(remainder);
            let (quotient, remainder) = big_dividend.div_rem(big_divisor);

            assert_eq!(
                (big_quotient, big_remainder),
                (quotient, remainder),
                "Test failed on iteration {}: \nDividend: {:?}\nDivisor: {:?}\nExpected Quotient: {:?}\nExpected Remainder: {:?}\nComputed Quotient: {:?}\nComputed Remainder: {:?}",
                j,
                dividend,
                divisor,
                big_quotient,
                big_remainder,
                quotient,
                remainder
            );
        }
    }

    #[test]
    fn div_nonzero_by_itself() {
        {
            let a = BigUInt::new([3]);
            let b = BigUInt::new([3]);
            let expected_quotient = BigUInt::new([1]);
            let expected_remainder = BigUInt::new([0]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
        {
            let a = BigUInt::new([u32::MAX]);
            let b = BigUInt::new([u32::MAX]);
            let expected_quotient = BigUInt::new([1]);
            let expected_remainder = BigUInt::new([0]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
        {
            let a = BigUInt::new([0x0000ffff, 0x0000ffff]);
            let b = BigUInt::new([0x0000ffff, 0x0000ffff]);
            let expected_quotient = BigUInt::new([1, 0]);
            let expected_remainder = BigUInt::new([0, 0]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
        {
            let a = BigUInt::new([0x0000789a, 0x0000bcde]);
            let b = BigUInt::new([0x0000789a, 0x0000bcde]);
            let expected_quotient = BigUInt::new([1, 0]);
            let expected_remainder = BigUInt::new([0, 0]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
    }

    #[test]
    fn div_nonzero_by_1() {
        {
            let a = BigUInt::new([u32::MAX]);
            let b = BigUInt::new([1]);
            let expected_quotient = BigUInt::new([u32::MAX]);
            let expected_remainder = BigUInt::new([0]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
        {
            let a = BigUInt::new([u32::MAX, u32::MAX]);
            let b = BigUInt::new([1, 0]);
            let expected_quotient = BigUInt::new([u32::MAX, u32::MAX]);
            let expected_remainder = BigUInt::new([0, 0]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
    }

    #[test]
    fn div_dividend_repeats_divisor() {
        let a = BigUInt::new([u32::MAX, u32::MAX]);
        let b = BigUInt::new([u32::MAX, 0]);
        let expected_quotient = BigUInt::new([1, 1]);
        let expected_remainder = BigUInt::new([0, 0]);
        assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
    }

    #[test]
    fn div_divisor_exceeds_dividend() {
        {
            let a = BigUInt::new([3]);
            let b = BigUInt::new([4]);
            let expected_quotient = BigUInt::new([0]);
            let expected_remainder = BigUInt::new([3]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
        {
            let a = BigUInt::new([0x00007899, 0x0000bcde]);
            let b = BigUInt::new([0x0000789a, 0x0000bcde]);
            let expected_quotient = BigUInt::new([0, 0]);
            let expected_remainder = BigUInt::new([0x00007899, 0x0000bcde]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
    }

    #[test]
    fn div_common_cases() {
        {
            let a = BigUInt::new([3]);
            let b = BigUInt::new([2]);
            let expected_quotient = BigUInt::new([1]);
            let expected_remainder = BigUInt::new([1]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
        {
            let a = BigUInt::new([u32::MAX]);
            let b = BigUInt::new([3]);
            let expected_quotient = BigUInt::new([0x55555555]);
            let expected_remainder = BigUInt::new([0]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
        {
            let a = BigUInt::new([u32::MAX, u32::MAX - 1]);
            let b = BigUInt::new([u32::MAX, 0]);
            let expected_quotient = BigUInt::new([u32::MAX, 0]);
            let expected_remainder = BigUInt::new([u32::MAX - 1, 0]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
        {
            let a = BigUInt::new([0x00005678, 0x00001234]);
            let b = BigUInt::new([0x00009abc, 0]);
            let expected_quotient = BigUInt::new([0x1e1dba76, 0]);
            let expected_remainder = BigUInt::new([0x6bd0, 0]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
        {
            let a = BigUInt::new([0, 7]);
            let b = BigUInt::new([0, 3]);
            let expected_quotient = BigUInt::new([2, 0]);
            let expected_remainder = BigUInt::new([0, 1]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
        {
            let a = BigUInt::new([5, 7]);
            let b = BigUInt::new([0, 3]);
            let expected_quotient = BigUInt::new([2, 0]);
            let expected_remainder = BigUInt::new([5, 1]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
        {
            let a = BigUInt::new([0, 6]);
            let b = BigUInt::new([0, 2]);
            let expected_quotient = BigUInt::new([3, 0]);
            let expected_remainder = BigUInt::new([0, 0]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
        {
            let a = BigUInt::new([0x80000000]);
            let b = BigUInt::new([0x40000001]);
            let expected_quotient = BigUInt::new([1]);
            let expected_remainder = BigUInt::new([0x3fffffff]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
        {
            let a = BigUInt::new([0, 0x80000000]);
            let b = BigUInt::new([0, 0x40000001]);
            let expected_quotient = BigUInt::new([1, 0]);
            let expected_remainder = BigUInt::new([0, 0x3fffffff]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
        {
            let a = BigUInt::new([0, 0x80000000]);
            let b = BigUInt::new([0x00000001, 0x40000000]);
            let expected_quotient = BigUInt::new([1, 0]);
            let expected_remainder = BigUInt::new([0xffffffff, 0x3fffffff]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
        {
            let a = BigUInt::new([0x0000789b, 0x0000bcde]);
            let b = BigUInt::new([0x0000789a, 0x0000bcde]);
            let expected_quotient = BigUInt::new([1, 0]);
            let expected_remainder = BigUInt::new([1, 0]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
        {
            let a = BigUInt::new([0x0000ffff, 0x0000ffff]);
            let b = BigUInt::new([0, 1]);
            let expected_quotient = BigUInt::new([0x0000ffff, 0]);
            let expected_remainder = BigUInt::new([0x0000ffff, 0]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
        {
            let a = BigUInt::new([0x000089ab, 0x00004567, 0x00000123]);
            let b = BigUInt::new([0, 1, 0]);
            let expected_quotient = BigUInt::new([0x00004567, 0x00000123, 0]);
            let expected_remainder = BigUInt::new([0x000089ab, 0, 0]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
    }

    #[test]
    fn div_q_estimate_exceeds_radix() {
        {
            let a = BigUInt::new([0x00000000, 0x0000fffe, 0x00008000]);
            let b = BigUInt::new([0x0000ffff, 0x00008000, 0x00000000]);
            let expected_quotient = BigUInt::new([0xffffffff, 0x00000000, 0x00000000]);
            let expected_remainder = BigUInt::new([0x0000ffff, 0x00007fff, 0x00000000]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
    }

    #[test]
    fn div_q_estimate_off_by_one() {
        {
            let a = BigUInt::new([0x00000003, 0x00000000, 0x80000000, 0]);
            let b = BigUInt::new([0x00000001, 0x00000000, 0x20000000, 0]);
            let expected_quotient = BigUInt::new([0x00000003, 0, 0, 0]);
            let expected_remainder = BigUInt::new([0, 0, 0x20000000, 0]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
        {
            let a = BigUInt::new([0x00000003, 0x00000000, 0x00008000, 0]);
            let b = BigUInt::new([0x00000001, 0x00000000, 0x00002000, 0]);
            let expected_quotient = BigUInt::new([0x00000003, 0, 0, 0]);
            let expected_remainder = BigUInt::new([0, 0, 0x00002000, 0]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
        {
            let a = BigUInt::new([0, 0, 0x00008000, 0x00007fff]);
            let b = BigUInt::new([1, 0, 0x00008000, 0]);
            let expected_quotient = BigUInt::new([0xfffe0000, 0, 0, 0]);
            let expected_remainder = BigUInt::new([0x00020000, 0xffffffff, 0x00007fff, 0]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
    }

    #[test]
    fn div_mul_and_sub_cannot_be_signed() {
        {
            let a = BigUInt::new([0, 0x0000fffe, 0, 0x00008000]);
            let b = BigUInt::new([0x0000ffff, 0, 0x00008000, 0]);
            let expected_quotient = BigUInt::new([0xffffffff, 0, 0, 0]);
            let expected_remainder = BigUInt::new([0x0000ffff, 0xffffffff, 0x00007fff, 0]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
        {
            let a = BigUInt::new([0, 0xfffffffe, 0, 0x80000000]);
            let b = BigUInt::new([0x0000ffff, 0, 0x80000000, 0]);
            let expected_quotient = BigUInt::new([0x00000000, 1, 0, 0]);
            let expected_remainder = BigUInt::new([0x00000000, 0xfffeffff, 0x00000000, 0]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
        {
            let a = BigUInt::new([0, 0xfffffffe, 0, 0x80000000]);
            let b = BigUInt::new([0xffffffff, 0, 0x80000000, 0]);
            let expected_quotient = BigUInt::new([0xffffffff, 0, 0, 0]);
            let expected_remainder = BigUInt::new([0xffffffff, 0xffffffff, 0x7fffffff, 0]);
            assert_eq!((expected_quotient, expected_remainder), a.div_rem(b));
        }
    }
}
