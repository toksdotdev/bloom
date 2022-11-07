use std::hash::Hash;
use std::hash::Hasher;

use rand::thread_rng;
use rand::Rng;
use seahash::SeaHasher;

/// Hash a content with the provided hasher.
pub fn hash<H: Hasher, T: Hash>(mut hasher: H, content: T) -> usize {
    content.hash(&mut hasher);
    hasher.finish() as usize
}

/// Calculate the optimal bit size for a bloom filter.
pub fn optimal_bit_size(m: usize, false_positive_rate: f64) -> usize {
    ((-1f64 * m as f64 * false_positive_rate.ln()) / 2f64.ln().powi(2)).ceil() as usize
}

/// Calculate the optiomal hash count for a bloom filter.
pub fn optimal_hash_count(m: usize, bit_size: usize) -> usize {
    (bit_size as f64 * 2.0f64.ln() / m as f64).ceil() as usize
}

/// Build n-sized collection of seahash functions.
pub fn build_seahash_functions(n: usize) -> Vec<SeaHasher> {
    let mut rng = thread_rng();

    (0..n)
        .into_iter()
        .map(|_| {
            SeaHasher::with_seeds(
                rng.gen::<u64>(),
                rng.gen::<u64>(),
                rng.gen::<u64>(),
                rng.gen::<u64>(),
            )
        })
        .collect()
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn test_optimal_hash_paramaters() {
        let bit_size = optimal_bit_size(100_000, 0.01);
        let hash_count = optimal_hash_count(100_000, bit_size);
        assert_eq!(bit_size, 958506);
        assert_eq!(hash_count, 7);

        let bit_size = optimal_bit_size(4000, 0.02);
        let hash_count = optimal_hash_count(4000, bit_size);
        assert_eq!(bit_size, 32570);
        assert_eq!(hash_count, 6);
    }
}
