use alloc::vec::Vec;
use core::mem::size_of;
use core::ops::{BitXor, Not};
use core::{fmt::Debug, ops::BitAnd};
use numeric::{FromBeBytes, ToBeBytes, Widen, WrappingAdd, Zero};

pub(crate) fn pad<T, U>(buffer: &Vec<u8>) -> Vec<u8>
where
    T: TryFrom<usize> + ToBeBytes + Widen,
    U: TryFrom<usize> + ToBeBytes,
    U::Error: Debug,
    <T as Widen>::Output: ToBeBytes,
    T::Error: Debug,
{
    let word_bytes = size_of::<T>();
    let block_size = word_bytes * 128;
    let mut padded_buffer = buffer.clone();
    let bit_length = padded_buffer.len() * 8;
    let mut zeroes = 7;
    padded_buffer.push(0x80);

    while ((bit_length + 1 + zeroes) % block_size) != (block_size - (word_bytes * 16)) {
        padded_buffer.push(0);
        zeroes += 8;
    }

    let bit_length = U::try_from(bit_length).unwrap().to_be_bytes();

    padded_buffer.extend_from_slice(bit_length.as_ref());
    padded_buffer
}

pub(crate) fn parse<T>(buffer: &[u8]) -> Vec<[T; 16]>
where
    T: FromBeBytes + Zero + Copy,
    <T as FromBeBytes>::Bytes: for<'a> TryFrom<&'a [u8]>,
{
    let mut blocks = Vec::new();
    let word_bytes = size_of::<T>();
    let block_bytes = word_bytes * 16;

    for chunk in buffer.chunks(block_bytes) {
        let mut block: [T; 16] = [T::zero(); 16];

        for (i, word) in chunk.chunks(word_bytes).enumerate() {
            block[i] =
                T::from_be_bytes(&T::Bytes::try_from(word).unwrap_or_else(|_| unreachable!()));
        }

        blocks.push(block);
    }

    blocks
}

pub(crate) fn hash<T>(
    blocks: &[&[T]],
    hash: &mut [T],
    schedule: &mut [T],
    working_variables: &mut [T],
    schedule_fn: &dyn Fn(usize, &[T]) -> T,
    update_fn: &mut dyn FnMut(usize, &mut [T], &[T]),
) where
    T: WrappingAdd + Copy + Sized,
{
    for block in blocks {
        for t in 0..schedule.len() {
            schedule[t] = if t < 16 {
                block[t]
            } else {
                schedule_fn(t, schedule)
            }
        }

        working_variables.copy_from_slice(&hash);

        for t in 0..schedule.len() {
            update_fn(t, working_variables, schedule);
        }

        for i in 0..hash.len() {
            hash[i] = working_variables[i].wrapping_add(&hash[i]);
        }
    }
}

pub(crate) fn ch<T>(x: T, y: T, z: T) -> T
where
    T: BitAnd<Output = T> + Not<Output = T> + BitXor<Output = T> + Copy,
{
    (x & y) ^ (!x & z)
}

pub(crate) fn maj<T>(x: T, y: T, z: T) -> T
where
    T: BitAnd<Output = T> + BitXor<Output = T> + Copy,
{
    (x & y) ^ (x & z) ^ (y & z)
}

pub trait HashingAlgorithm<T, const N: usize> {
    fn new() -> Self
    where
        Self: Sized;
    fn update(&mut self, data: &[u8]);
    fn digest(&self) -> [T; N];
    fn reset(&mut self);
}
