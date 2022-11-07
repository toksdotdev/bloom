use crate::utils::build_seahash_functions;
use crate::utils::optimal_bit_size;
use crate::utils::optimal_hash_count;
use crate::CountingBloomFilter;

use seahash::SeaHasher;

/// Create an bloom filter which uses seahash as its underlying hash function.
pub fn seahash_counting_bloom_filter(
    m: usize,
    false_positive_rate: f64,
) -> CountingBloomFilter<SeaHasher> {
    let bit_size = optimal_bit_size(m, false_positive_rate);
    let hash_counts = optimal_hash_count(m, bit_size);
    let hashers = build_seahash_functions(hash_counts);
    CountingBloomFilter::new(bit_size, hashers)
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn test_seahash_counting_bloom_filter_properties() {
        let bloom = seahash_counting_bloom_filter(100_000, 0.01);
        assert_eq!(bloom.bit_size(), 958506);
        assert_eq!(bloom.total_hashers(), 7);

        let bloom = seahash_counting_bloom_filter(4000, 0.02);
        assert_eq!(bloom.bit_size(), 32570);
        assert_eq!(bloom.total_hashers(), 6);
    }

    #[test]
    fn test_check_for_seahash_counting_bloom_filter() {
        let mut bloom = seahash_counting_bloom_filter(100_000, 0.01);
        bloom.insert("Bob");
        bloom.insert("John");
        bloom.insert("Toks");
        bloom.insert("Linda");

        assert!(bloom.check("Bob"));
        assert!(bloom.check("John"));
        assert!(bloom.check("Toks"));
        assert!(bloom.check("Linda"));
        assert!(!bloom.check("john1"));
        assert!(!bloom.check("toks1"));
        assert!(!bloom.check("linda1"));
    }

    #[test]
    fn test_delete_for_seahash_counting_bloom_filter() {
        let mut bloom = seahash_counting_bloom_filter(100_000, 0.10);
        bloom.delete("Bob");
        bloom.delete("Bob");
        assert!(!bloom.check("Bob"));

        bloom.insert("Bob");
        bloom.insert("John");
        bloom.insert("John");
        bloom.insert("Toks");
        bloom.insert("Toks");
        bloom.insert("Linda");

        bloom.delete("John");
        bloom.delete("Toks");
        assert!(bloom.check("Toks"));
        assert!(bloom.check("John"));

        bloom.delete("John");
        bloom.delete("Toks");
        assert!(!bloom.check("Toks"));
        assert!(!bloom.check("John"));
        assert!(bloom.check("Linda"));
    }
}
