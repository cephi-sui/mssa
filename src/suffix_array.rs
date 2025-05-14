use std::cmp::{self, Ordering};

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
/// let suffix_array = SuffixArray::<StandardQuery>::from_kmers(kmers, w, ());
/// ```
#[derive(Debug)]
pub struct SuffixArray<T> {
    underlying_kmers: KmerSequence,
    w: usize,

    // NOTE: we avoid storing a Vec<&[SuperKmer]> to make serialization easier
    super_kmers: Vec<SuperKmer>,
    suffix_array: Vec<usize>,

    /// Stores any auxillary data structures required by non-standard query modes
    query_mode_aux_data: T,
}

pub trait QueryMode {
    type InitParams;

    fn initialize_aux_data(
        kmers: &KmerSequence,
        w: usize,
        suffix_array: &[&[SuperKmer]],
        init_params: Self::InitParams,
    ) -> Self;
}

#[derive(Debug)]
pub struct StandardQuery;

impl QueryMode for StandardQuery {
    type InitParams = ();

    fn initialize_aux_data(
        _kmers: &KmerSequence,
        _w: usize,
        _suffix_array: &[&[SuperKmer]],
        _init_params: Self::InitParams,
    ) -> Self {
        StandardQuery {}
    }
}

// NOTE: this is a placeholder
pub struct BloomFilterQuery {
    example: BloomFilter,
}

impl QueryMode for BloomFilterQuery {
    /// False positive rate
    type InitParams = f32;

    fn initialize_aux_data(
        kmers: &KmerSequence,
        w: usize,
        suffix_array: &[&[SuperKmer]],
        init_params: Self::InitParams,
    ) -> Self {
        todo!()
    }
}

impl<T: QueryMode> SuffixArray<T> {
    // put any methods that don't need to touch query_mode_aux_data here

    pub fn from_kmers(kmers: KmerSequence, w: usize, init_params: T::InitParams) -> Self {
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

        let suffix_array_slices: Vec<_> =
            suffix_array.iter().map(|&i| &super_kmers[i..n]).collect();

        let query_mode_aux_data =
            T::initialize_aux_data(&kmers, w, &suffix_array_slices, init_params);

        Self {
            underlying_kmers: kmers,
            w,
            super_kmers,
            suffix_array,
            query_mode_aux_data,
        }
    }

    pub fn get_suffix_array(&self) -> Vec<&[SuperKmer]> {
        let n = self.super_kmers.len();

        self.suffix_array
            .iter()
            .map(|&i| &self.super_kmers[i..n])
            .collect()
    }
}

// The standard query mode, with no accelerant data structures
impl SuffixArray<StandardQuery> {
    pub fn dumbquery(&self, query: &[u8]) -> Option<usize> {
        todo!();
        let suffix_array = self.get_suffix_array();

    }

    pub fn query(&self, query: &[u8]) -> Option<usize> {
        let suffix_array = self.get_suffix_array();

        let query_kmers = KmerSequence::from_bytes(query, self.underlying_kmers.k());
        let query_super_kmers = query_kmers.compute_super_kmers(self.w);

        let cmp_slice_to_query = |slice: &[SuperKmer]| {
            let l = cmp::min(slice.len(), query_super_kmers.len());
            slice.iter().take(l).my_cmp_by(
                query_super_kmers.iter().take(l),
                |SuperKmer {
                     minimizer: minimizer1,
                     ..
                 },
                 SuperKmer {
                     minimizer: minimizer2,
                     ..
                 }| self.underlying_kmers.compare_kmers(minimizer1, minimizer2),
            )
        };

        // Look for first index in suffix array == kmer
        let left_idx =
            suffix_array.partition_point(|&slice| cmp_slice_to_query(slice) == Ordering::Less);
        // Look for first index in suffix array > kmer
        let right_idx =
            suffix_array.partition_point(|&slice| cmp_slice_to_query(slice) != Ordering::Greater);

        if left_idx == right_idx {
            // Query not present
            println!("AAAAAAAAAAAAAAAAAAAAAA: {:?}", left_idx);
            return None;
        }

        // Query could be present anywhere in the range
        // TODO: can this be optimized by not constructing the entire original string for every
        // query?
        let original_string = self.underlying_kmers.get_original_string();
        println!("left_idx: {:?}", left_idx);
        println!("right_idx: {:?}", right_idx);
        for i in left_idx..right_idx {
            let super_kmers = suffix_array[i];

            let start_pos = super_kmers.first().unwrap().start_pos;
            let end_pos =
                super_kmers.last().unwrap().start_pos + super_kmers.last().unwrap().length - 1;
            println!("start_pos: {:?}", start_pos);
            println!("end_pos: {:?}", end_pos);

            for (i, w) in original_string[start_pos..end_pos]
                .windows(query.len())
                .enumerate()
            {
                println!("COMPARING {:?} WITH {:?}", w, query);
                if w == query {
                    return Some(start_pos + i);
                }
            }
        }

        None
    }
}
