#![no_std]

use alloc::string::String;
extern crate alloc;
const RADIX: u64 = u32::MAX as u64 + 1;

#[derive(Debug, PartialEq, Copy, Clone)]
struct BigUInt<const N: usize> {
    limbs: [u32; N],
}

impl<const N: usize> Default for BigUInt<N> {
    fn default() -> Self {
        BigUInt {
            limbs: [0; N],
        }
    }
}

impl <const N: usize> BigUInt<N> {
    fn new(limbs: [u32; N]) -> Self{
        BigUInt {limbs}
    }

    fn from_u64(n: u64) -> Self {
        let mut limbs = [0; N];
        limbs[0] = n as u32;
        limbs[1] = (n >> 32) as u32;

        BigUInt::new(limbs)
    }

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

        BigUInt::new(w)
    }

    fn shl_carry(&self, n: u32) -> (Self, u32) {
        if n == 0 {return (BigUInt::new(self.limbs), 0)}
        let mut limbs = [0; N];
        let mut carry = 0;

        for i in 0..N {
            limbs[i] = self.limbs[i] << n | carry;
            carry = self.limbs[i] >> (32 - n);
        }

        (BigUInt::new(limbs), carry)
    }

    fn shr_carry(self, n: u32) -> (Self, u32) {
        if n == 0 {return (BigUInt::new(self.limbs), 0)}
        let mut limbs = [0; N];
        let mut carry = 0;

        for i in (0..N).rev() {
            limbs[i] = self.limbs[i] >> n | carry;
            carry = self.limbs[i] & ((1 >> n) - 1);
        }

        (BigUInt::new(limbs), carry)
    }

    fn leading_zeros(&self) -> u32 {
        let mut zeros = 0;

        for i in (0..N).rev() {
            if (self.limbs[i] > 0) {
                break;
            } else {
                zeros += 1;
            }
        }

        zeros
    }

    fn checked_mul(&self, rhs: Self) -> Option<Self> {
        let mut w = [0; N];
        let mut k = 0;
        
        for j in 0..N {
            k = 0;
            if rhs.limbs[j] == 0 {continue};
            for i in 0..N {
                if i + j >= N {
                    if self.limbs[i] != 0 || rhs.limbs[i] != 0 {
                        return None;
                    }
                } else {
                    let t = u64::from(self.limbs[i]) * u64::from(rhs.limbs[j]) + u64::from(w[i + j]) + k;
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

    fn div_rem(&self, rhs: Self) -> (Self, Self) {
        let dividend_digits = N - self.leading_zeros() as usize;
        let divisor_digits = N - rhs.leading_zeros() as usize;

        let shift = rhs.limbs[divisor_digits - 1].leading_zeros();
        let (mut dividend,  dividend_overflow) = self.shl_carry(shift);
        let (divisor,  _) = rhs.shl_carry(shift);
        let mut q: BigUInt<N> = BigUInt::default();

        for i in (0..=(dividend_digits - divisor_digits)).rev() {
            let msd = if i == (dividend_digits - divisor_digits) {
                dividend_overflow
            } else {
                dividend.limbs[divisor_digits + i]
            };
            let smsd = dividend.limbs[divisor_digits + i - 1];

            let dividend_msd_combined = u64::from(msd) * RADIX + u64::from(smsd);
            let mut q_estimate = dividend_msd_combined / u64::from(divisor.limbs[divisor_digits - 1]);
            let mut r_estimate = dividend_msd_combined % u64::from(divisor.limbs[divisor_digits - 1]);
            
        
            loop {
                if q_estimate >= RADIX 
                || ((q_estimate * u64::from(divisor.limbs[divisor_digits - 2])) > (r_estimate * RADIX + u64::from(dividend.limbs[divisor_digits - 2 + i])))
                {
                    q_estimate -= 1;
                    r_estimate += u64::from(divisor.limbs[divisor_digits - 1]);
                    if r_estimate < RADIX {continue};
                }
                break;
            }
            
            // Based on previous loop, q_estimate must fit within 32 bits now
            let mut carry: i64 = 0;
            for j in 0..divisor_digits {
                let subtrahend = q_estimate * u64::from(divisor.limbs[i]);
                let difference = i64::from(dividend.limbs[i + j]) - carry - ((subtrahend & 0xFFFFFFFF) as i64);
                dividend.limbs[i + j] = difference as u32;
                carry = i64::from((subtrahend >> 32) as u32) - i64::from((difference >> 32) as u32);
            }
            let final_digit = i64::from(divisor.limbs[i + divisor_digits]) - carry;
            
            q.limbs[i] = q_estimate as u32;
            if final_digit < 0 {
                let mut carry = 0;
                for j in 0..divisor_digits {
                    let sum = u64::from(divisor.limbs[i + j]) + u64::from(divisor.limbs[i]) + carry;
                    dividend.limbs[i + j] = sum as u32;
                    carry = sum >> 32;
                }
                dividend.limbs[i + divisor_digits] = dividend.limbs[i + divisor_digits] + carry as u32;
            }
        }
        
        (q, dividend >> shift)
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

        BigUInt::new(w)
    }
}

impl <const N: usize> core::ops::Shr<u32> for BigUInt<N> {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        self.shr_carry(rhs).0
    }
}

impl <const N: usize> core::ops::Sub for BigUInt<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut w = [0; N];
        let mut k = 0;

        for j in 0..N {
            let difference = i64::from(self.limbs[j]) - i64::from(rhs.limbs[j]) + k;
            w[j] = (difference % RADIX as i64) as u32;
            if difference < 0 {
                if j == (N - 1) {
                    panic!("integer overflow");
                }
                k = -1;
            } else {
                k = 0;
            }
        }

        BigUInt::new(w)
    }
}

impl <const N: usize> core::cmp::PartialOrd for BigUInt<N> {
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

impl <const N: usize> core::ops::Div for BigUInt<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.div_rem(rhs).0
    }
}

impl <const N: usize> core::ops::Rem for BigUInt<N> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        self.div_rem(rhs).1
    }
}

impl <const N: usize> core::ops::Mul for BigUInt<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.checked_mul(rhs).expect("integer overflow")
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
        assert_eq!(result.limbs, [u32::MAX - 1, u32::MAX , u32::MAX, 1]); // Check for carry propagation
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
        assert_eq!(result, BigUInt { limbs: [u32::MAX, u32::MAX, 0] });
    }

    #[test]
    fn sub_with_borrow_to_zero() {
        let a = BigUInt::new([0, 0, 1]);
        let b = BigUInt::new([1, 1, 0]);
        let result = a - b;
        assert_eq!(result, BigUInt { limbs: [u32::MAX, u32::MAX - 1, 0] });
    }


    #[test]
    fn div() {
        let a = BigUInt::new([u32::MAX, u32::MAX, u32::MAX, u32::MAX, 0, 0]);
        let b = BigUInt::new([u32::MAX, u32::MAX, u32::MAX, 0, 0, 0]);
        let (q, r) = a.div_rem(b);
        let c = q * b;
        let d = c + r;
        let e = a - d;
        let j = a + b;
        let f = "s";
        // assert_eq!(result.0 * b + result.1, a);
    }

}

