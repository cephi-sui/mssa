use core::fmt;

use bitvec::prelude::*;
use funty::Unsigned;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IntVec {
    inner: BitVec,
    bits: usize,
}

pub struct IntVecIterator<'a> {
    intvec: &'a IntVec,
    index: usize,
}

impl<'a> Iterator for IntVecIterator<'a> {
    type Item = u128;

    fn next(&mut self) -> Option<u128> {
        let result = self.intvec.get(self.index)?;
        self.index += 1;
        Some(result)
    }
}

impl IntVec {
    /// Create a new IntVec of integers of a generic length `bits` bits.
    /// Panics if `bits` is not between 1 and 128.
    pub fn new(bits: usize) -> Self {
        assert!(
            bits >= 1 && bits <= 128,
            "IntArray: N must be between 1 and 128"
        );

        let inner = bitvec![usize, Lsb0; 0; 0];

        Self { inner, bits }
    }

    /// Create a new IntVec of `len` integers of a generic length `bits` bits.
    /// Panics if `bits` is not between 1 and 128, or if an arithmetic
    /// overflow occurs.
    pub fn new_zeros(bits: usize, len: usize) -> Self {
        assert!(
            bits >= 1 && bits <= 128,
            "IntArray: N must be between 1 and 128"
        );

        let bit_count = len.checked_mul(bits).expect("IntArray size too large");
        let inner = bitvec![usize, Lsb0; 0; bit_count];

        Self { inner, bits }
    }

    pub fn from_iter<T: Unsigned, V: IntoIterator<Item = T>>(bits: usize, iter: V) -> Self {
        let mut ret = Self::new(bits);

        for x in iter {
            let x: u128 = x.as_u128();
            ret.push(x);
        }

        ret
    }

    pub fn integer_size(self: &Self) -> usize {
        self.bits
    }

    pub fn len(&self) -> usize {
        self.inner.len() / self.bits
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<u128> {
        let start = index * self.bits;
        let range = start..(start + self.bits);

        Some(self.inner.get(range)?.load_le())
    }

    /// Panics if `value` is larger than 2^(`integer_size`) - 1.
    pub fn set(&mut self, index: usize, value: u128) -> Option<()> {
        assert!(
            value < (1u128 << self.bits),
            "Value too large to fit in integer of specified length",
        );

        let start = index * self.bits;
        let range = start..(start + self.bits);

        self.inner.get_mut(range)?.store_le(value);

        Some(())
    }

    /// Panics if `value` is larger than 2^(`integer_size`) - 1.
    pub fn push(&mut self, value: u128) {
        assert!(
            value < (1u128 << self.bits),
            "Value too large to fit in integer of specified length",
        );

        let mut val_bits = bitvec![u8, Lsb0; 0; self.bits];
        val_bits.store_le(value);
        self.inner.extend_from_bitslice(&val_bits);
    }

    pub fn iter(&self) -> IntVecIterator {
        IntVecIterator {
            intvec: self,
            index: 0,
        }
    }
}

impl<'a> IntoIterator for &'a IntVec {
    type Item = u128;
    type IntoIter = IntVecIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            intvec: self,
            index: 0,
        }
    }
}

impl fmt::Debug for IntVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "IntVec {{ bits = {}, values = [{}] }}",
            self.bits,
            self.iter().format(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let mut int_array = IntVec::new_zeros(8, 4);

        for i in 0..int_array.len() {
            int_array.set(i, (i * 10) as u128);
        }

        for i in 0..int_array.len() {
            assert_eq!(int_array.get(i), Some((i * 10) as u128));
        }
    }

    #[test]
    fn push_test() {
        let mut int_array = IntVec::new(10);

        for i in 0..100 {
            int_array.push((i * 10) as u128);
        }

        for i in 0..100 {
            assert_eq!(int_array.get(i), Some((i * 10) as u128));
        }

        assert_eq!(int_array.len(), 100);
        assert_eq!(int_array.inner.len(), 100 * 10);
    }
}
