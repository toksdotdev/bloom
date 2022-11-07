# 🌺 Bloom

Nothing new here. Just another [bloom filter](https://en.wikipedia.org/wiki/Bloom_filter) implementation for:

- [Classic bloom](https://en.wikipedia.org/wiki/Bloom_filter#Examples)
- [Counting bloom](https://en.wikipedia.org/wiki/Bloom_filter#Counting_Bloom_filters)

## Usage

### Bloom with [seahash](https://docs.rs/seahash/latest/seahash)

An [optimal](https://en.wikipedia.org/wiki/Bloom_filter#Optimal_number_of_hash_functions) basic filter which makes makes use of SeaHasher as the underlying hash function. If you'll like to use a customer hashing function, check [here].

```rust
use bloom_filter::seahash_bloom_filter;

let bloom = seahash_bloom_filter(100_000, 0.01);
bloom.insert("John");
bloom.insert("Doe");

assert!(bloom.check("John"));
assert!(bloom.check("Doe"));
assert!(!bloom.check("Anderson"));
```

### Counting Bloom with [seahash](https://docs.rs/seahash/latest/seahash)

An [optimal](https://en.wikipedia.org/wiki/Bloom_filter#Optimal_number_of_hash_functions) counting bloom filter which makes use of SeaHasher as the underlying hashing function.

```rust
use bloom_filter::seahash_counting_bloom_filter;

let bloom = seahash_counting_bloom_filter(100_000, 0.01);
bloom.insert("John");
bloom.insert("John"); // inserted second time
assert!(bloom.check("John"));

bloom.delete("John");
assert!(bloom.check("John"));

bloom.delete("John");
assert!(!bloom.check("John"));
```


### Bloom with custom paramaters and hash functions.
There might be cases where you wouldn't want to rely on the in-build functions (e.g. `seahash_*_bloom_filter()`), but instead rely on a different `m`, `k`, or the hash function. 

If you'll like to calculate the optimal values of bit mask size, `m`, or number of hash functions, `k`, you can rely on the `utils::optimal_*()` utils provided out of the box.

```rust
use bloom_filter::BloomFilter;
use bloom_filter::utils::*;
use std::hash::Hasher;

/// Build a total of k hashers.
fn build_hashers(k: usize) -> Vec<impl Hasher> {
    (0..k).iter().map(|_| { /* Build custom hashers here */ }).collect()
}

fn main() {
    let (m, false_positive_rate) = (100_000, 0.01);
    let bit_size = optimal_bit_size(m, false_positive_rate);
    let hash_count = optimal_hash_size(m, false_positive_rate); // also referred to as, k.

    let hashers = build_hashers(hash_count);
    let bloom = BloomFilter::new(bit_size, hashers);
    bloom.insert("hello");
}
```

### Counting bloom into bloom

`CountingBloomFilter` implements the `From<T>` trait, and can easily be converted into the default `BloomFilter` without losing the underlying bit mask data. The only information that gets truncated is the `counter` which keeps track of inserted items (only needed within the confines of a counting bloom filter).

```rust
use bloom_filter::seahash_counting_bloom_filter;

let counting_bloom_filter = seahash_counting_bloom_filter(100_000, 0.01);

counting_bloom_filter.insert("John");
assert!(counting_bloom_filter.check("John"));

let bloom_filter = BloomFilter::from(counting_bloom_filter);
assert!(bloom_filter.check("John"));
```