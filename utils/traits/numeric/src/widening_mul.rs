pub trait WideningMul<Rhs = Self> {
    type Output;

    fn widening_mul(&self, rhs: &Rhs) -> Self::Output;
}

macro_rules! impl_widening_mul {
    ($int_type:ty, $double_type:ty) => {
        impl WideningMul<$int_type> for $int_type {
            type Output = $double_type;

            fn widening_mul(&self, rhs: &$int_type) -> Self::Output {
                (*self as $double_type) * (*rhs as $double_type)
            }
        }
    };
}

impl_widening_mul!(u8, u16);
impl_widening_mul!(u16, u32);
impl_widening_mul!(u32, u64);
impl_widening_mul!(u64, u128);

impl_widening_mul!(i8, i16);
impl_widening_mul!(i16, i32);
impl_widening_mul!(i32, i64);
impl_widening_mul!(i64, i128);
