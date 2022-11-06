pub mod sample;
pub mod utils;

use crate::utils::hash;
use bitvec::bitvec;
use bitvec::vec::BitVec;
use std::hash::{Hash, Hasher};

/// Bloom filter.
pub struct BloomFilter<H: Hasher + Copy> {
    hashers: Vec<H>,
    mask: BitVec,
}

impl<H: Hasher + Copy> BloomFilter<H> {
    /// Build a new instance of a bloom filter.
    pub fn new(bit_size: usize, hashers: Vec<H>) -> Self {
        Self {
            hashers,
            mask: bitvec![0; bit_size],
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
}
