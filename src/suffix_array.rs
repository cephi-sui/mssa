use bloom::BloomFilter;

use crate::iter_order_by::MyIterOrderBy;
use crate::transform::{KmerSequence, SuperKmer};

/// A suffix array, constructed over a sequence of kmers.
///
/// This type is generic over multiple *query modes*, which may alter
/// the creation and querying of a suffix array. Query modes are structs
/// that store any auxillary data associated with that query mode --
/// for example, the BloomFilterQuery stores bloom filters.
///
/// You can create a suffix array with a given query mode as follows:
///
/// ```
/// let kmers = some_kmer_computation();
/// let w = 3;
/// let suffix_array = SuffixArray::<StandardQuery>::from_kmers(kmers, w);
/// ```
#[derive(Debug)]
pub struct SuffixArray<T> {
    underlying_kmers: KmerSequence,

    super_kmers: Vec<SuperKmer>,
    suffix_array: Vec<usize>,

    /// Stores any auxillary data structures required by non-standard query modes
    query_mode_aux_data: T,
}

// Build a suffix array. Only meant to be used internally; code should
// create suffix arrays by calling SuffixArray::<T>::from_kmers().
fn build_suffix_array(kmers: &KmerSequence, w: usize) -> (Vec<SuperKmer>, Vec<usize>) {
    // Construct the suffix array
    let super_kmers = kmers.compute_super_kmers(w);
    let n = super_kmers.len();

    let mut suffix_array = (0..n).collect::<Vec<_>>();

    // Sort the suffix array
    suffix_array.sort_by(|&i1, &i2| {
        let suffix1 = &super_kmers[i1..n];
        let suffix2 = &super_kmers[i2..n];

        suffix1.iter().my_cmp_by(suffix2.iter(), |x, y| {
            kmers.compare_kmers(&x.minimizer, &y.minimizer)
        })
    });

    (super_kmers, suffix_array)
}

impl<T> SuffixArray<T> {
    // put any methods that don't need to touch query_mode_aux_data here
}

#[derive(Debug)]
pub struct StandardQuery;

// TODO: this is a placeholder
pub struct BloomFilterQuery {
    bloom_filter: BloomFilter,
}

// The standard query mode, with no accelerant data structures
impl SuffixArray<StandardQuery> {
    pub fn from_kmers(kmers: KmerSequence, w: usize) -> Self {
        let (super_kmers, suffix_array) = build_suffix_array(&kmers, w);

        Self {
            underlying_kmers: kmers,
            super_kmers,
            suffix_array,
            query_mode_aux_data: StandardQuery {},
        }
    }

    pub fn query(&self, query: &[u8]) -> Option<usize> {
        let query_kmers = KmerSequence::from_bytes(query, self.underlying_kmers.k());

        todo!()
    }
}

impl SuffixArray<BloomFilterQuery> {
    pub fn from_kmers(kmers: KmerSequence, w: usize) -> Self {
        let (super_kmers, suffix_array) = build_suffix_array(&kmers, w);

        // TODO: construct bloom filter
        let bloom_filter = todo!();

        Self {
            underlying_kmers: kmers,
            super_kmers,
            suffix_array,
            query_mode_aux_data: BloomFilterQuery { bloom_filter },
        }
    }
}
