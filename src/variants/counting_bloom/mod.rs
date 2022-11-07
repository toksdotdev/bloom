mod sample;

use crate::utils::hash;
pub use crate::variants::counting_bloom::sample::*;
use crate::BloomFilter;

use bitvec::bitvec;
use bitvec::vec::BitVec;
use std::hash::{Hash, Hasher};

/// Counting bloom filter which also keeps tracking of items inserted.
pub struct CountingBloomFilter<H: Hasher + Copy> {
    /// Hashers for bloom filter contents.
    hashers: Vec<H>,

    /// Bit mask for items inserted.
    mask: BitVec,

    /// Count of items inserted for each mask item.
    counter: Vec<usize>,
}

impl<H: Hasher + Copy> CountingBloomFilter<H> {
    /// Build a new instance of a bloom filter.
    pub fn new(bit_size: usize, hashers: Vec<H>) -> Self {
        Self {
            hashers,
            mask: bitvec![0; bit_size],
            counter: vec![0; bit_size],
        }
    }

    /// Get the size of the underlying bit mask.
    pub fn bit_size(&self) -> usize {
        self.mask.len()
    }

    /// Get the total number of hash functions.
    pub fn total_hashers(&self) -> usize {
        self.hashers.len()
    }

    /// Insert the content into the bloom filter.
    pub fn insert<T: Hash>(&mut self, content: T) {
        for hasher in self.hashers.iter().copied() {
            let index = hash(hasher, &content) % self.mask.len();
            self.mask.set(index, true);
            self.counter[index] += 1;
        }
    }

    /// Check if the content probably exists in the filter.
    ///
    /// Returns ```false``` if it doesn't exist.
    pub fn check<T: Hash>(&mut self, content: T) -> bool {
        for hasher in self.hashers.iter().copied() {
            let index = hash(hasher, &content) % self.mask.len();
            if !self.mask[index] {
                return false;
            }
        }

        true
    }

    /// Delete an item from the counting bloom filter.
    pub fn delete<T: Hash>(&mut self, content: T) {
        for hasher in self.hashers.iter().copied() {
            let index = hash(hasher, &content) % self.mask.len();
            if self.counter[index] <= 0 {
                continue;
            }

            self.counter[index] -= 1;
            if self.counter[index] == 0 {
                self.mask.set(index, false);
            }
        }
    }
}

impl<H: Hasher + Copy> From<CountingBloomFilter<H>> for BloomFilter<H> {
    fn from(bloom: CountingBloomFilter<H>) -> Self {
        Self {
            hashers: bloom.hashers,
            mask: bloom.mask,
        }
    }
}
