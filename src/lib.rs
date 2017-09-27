
extern crate bit_vec;
use bit_vec::BitVec;

use std::hash::Hash;
use std::hash::Hasher;
use std::hash::BuildHasher;

use std::collections::hash_map::RandomState;

/// Calculate the probability of getting a false positive
///
/// # Arguments
/// * `n_buckets`: number of buckets
/// * `n_hashers`: number of hashers
/// * `n_elems`: number of elements
fn false_positive_rate(n_buckets: usize, n_hashers: usize, n_elems: usize)
    -> f32
{
    let k = n_hashers as f32;
    let n = n_elems as f32;
    let m = n_buckets as f32;
        
    (1. - ((-k * n) / m).exp()).powf(k)
}

fn min_n_buckets(n_elems: usize, fp_rate: f32) -> usize
{
    let n = n_elems as f32;

    (-1. * n * fp_rate.ln() / (2f32.ln().powf(2.))).ceil() as usize
}

/// Calculate the optimal number of hashers
///
/// # Arguments
/// * `n_buckets`: number of buckets
/// * `n_elems`: number of elemements
fn optimal_n_hashers(n_buckets: usize, n_elems: usize) -> usize
{
    let n = n_elems as f32;
    let m = n_buckets as f32;

    ((m / n) * 2f32.ln()).ceil() as usize
}

/// Space-efficient probabilistic hash set
#[derive(Debug)]
pub struct BloomFilter
{
    buffer: BitVec,
    size: usize,
    hashers: Vec<RandomState>
}

impl BloomFilter
{
    /// Build a Bloom Filter with a specified false positive rate
    ///
    /// # Arguments
    /// * `n_elems`: expected number of elements
    /// * `fp_rate`: desired false positive rate (0.0 -> 1.0)
    pub fn new_with_fp(n_elems: usize, fp_rate: f32) -> BloomFilter
    {
        let min_buckets = min_n_buckets(n_elems, fp_rate);
        let n_hashers = optimal_n_hashers(min_buckets, n_elems);

        BloomFilter {
            size: 0,
            buffer: BitVec::from_elem(min_buckets, false),
            hashers: (0..n_hashers).map(|_| RandomState::new()).collect()
        }
    }

    /// Create a new Bloom Filter with specified buffer size
    ///
    /// # Arguments
    /// * `n_elems`: expected number of elements
    /// * `size`: desired buffer size
    pub fn new_with_size(n_elems: usize, size: usize) -> BloomFilter
    {
        let n_hashers = optimal_n_hashers(size, n_elems);

        BloomFilter {
            size: 0,
            buffer: BitVec::from_elem(size, false),
            hashers: (0..n_hashers).map(|_| RandomState::new()).collect()
        }
    }

    /// Add a member
    pub fn add<T>(&mut self, e: &T)
        where T: Hash
    {
        for idx in self.indexes(e) {
            self.buffer.set(idx, true);
        }

        self.size += 1;
    }

    /// Check membership
    pub fn may_contain<T>(&self, e: &T) -> bool
        where T: Hash
    {
        let mut may_contain = true;

        for idx in self.indexes(e) {
            may_contain &= self.buffer.get(idx).unwrap();
        }

        may_contain
    }

    /// Number of elements in the `BloomFilter`
    pub fn size(&self) -> usize
    {
        self.size
    }

    /// Number of buckets that a memebr can occupy
    pub fn buckets(&self) -> usize
    {
        self.buffer.capacity()
    }

    /// Number of hashers being used
    pub fn n_hashers(&self) -> usize
    {
        self.hashers.len()
    }

    /// False positive rate
    pub fn fp_rate(&self) -> f32
    {
        false_positive_rate(self.buckets(), self.n_hashers(), self.size())
    }

    /// The indexes that a element hashes to
    fn indexes<T>(&self, e: &T) -> Vec<usize>
        where T: Hash
    {
        let mut idxs = vec![];
        for h in &self.hashers {
            let mut hasher = h.build_hasher();
            e.hash(&mut hasher);
            idxs.push(hasher.finish() as usize % self.buffer.len()); 
        }
        idxs
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    /// Test that the bloom filter will always return the same results
    #[test]
    fn test_is_deterministic()
    {
        let to_add = "do add this";
        let dont_add = 123;
        let mut filter = BloomFilter::new_with_size(1, 100);
        filter.add(&to_add);

        // Check membership twice to make sure that the results are reproducable
        // even though the hashers are being reset
        assert_eq!(true,  filter.may_contain(&to_add));
        assert_eq!(true,  filter.may_contain(&to_add));

        assert_eq!(false, filter.may_contain(&dont_add));
        assert_eq!(false, filter.may_contain(&dont_add));

    }

    #[test]
    fn test_size_increments()
    {
        let to_add = "do add this";

        let mut filter = BloomFilter::new_with_size(3, 100);
        filter.add(&to_add);
        filter.add(&to_add);
        filter.add(&to_add);

        assert_eq!(3, filter.size());
    }

    #[test]
    fn test_fp_rate_is_zero_no_elems()
    {
        let filter = BloomFilter::new_with_size(100, 100);
        assert_eq!(0.0, filter.fp_rate());
    }
}
