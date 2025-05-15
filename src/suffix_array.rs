use std::cmp::{self, Ordering};

use bloom::BloomFilter;

use crate::iter_order_by::MyIterOrderBy;
use crate::transform::{Kmer, KmerSequence, SuperKmer};

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

pub struct GroundTruthQuery;

impl QueryMode for GroundTruthQuery {
    type InitParams = ();

    fn initialize_aux_data(
        _kmers: &KmerSequence,
        _w: usize,
        _suffix_array: &[&[SuperKmer]],
        _init_params: Self::InitParams,
    ) -> Self {
        Self {}
    }
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
        Self {}
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
        let mut super_kmers = kmers.compute_super_kmers(w);
        // Push sentinel kmer.
        super_kmers.push(SuperKmer {
            start_pos: kmers.get_original_string_len(),
            length: 0,
            minimizer: Kmer::Sentinel,
        });
        let super_kmers = super_kmers; // Drop mutability.

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

// The ground truth query mode which performs an extremely inefficient query for testing purposes.
impl SuffixArray<GroundTruthQuery> {
    pub fn query(&self, query: &[u8]) -> Option<usize> {
        let ref_str = self.underlying_kmers.get_original_string();
        for (i, window) in ref_str.windows(query.len()).enumerate() {
            if query == window {
                return Some(i);
            }
        }
        None
    }
}

// The standard query mode, with no accelerant data structures
impl SuffixArray<StandardQuery> {
    pub fn query(&self, query: &[u8]) -> Option<usize> {
        assert!(query.len() >= self.w + self.underlying_kmers.k() - 1,
            "query length was shorter than minimum length required by w + k - 1");

        let suffix_array = self.get_suffix_array();

        let query_kmers = KmerSequence::from_bytes(
            query,
            self.underlying_kmers.k(),
            self.underlying_kmers.alphabet(),
        );
        let query_super_kmers = query_kmers.compute_super_kmers(self.w);

        let cmp_slice_to_query = |slice: &[SuperKmer]| {
            //let l = cmp::min(slice.len(), query_super_kmers.len());
            let l = query_super_kmers.len();
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
            return None;
        }

        // Query could be present anywhere in the range
        // TODO: can this be optimized by not constructing the entire original string for every
        // query?
        let original_string = self.underlying_kmers.get_original_string();
        for i in left_idx..right_idx {}
        for i in left_idx..right_idx {
            let super_kmers = suffix_array[i];

            let start_pos = super_kmers.first().unwrap().start_pos;
            let end_pos = super_kmers.last().unwrap().start_pos;

            for (i, w) in original_string[start_pos..end_pos]
                .windows(query.len())
                .enumerate()
            {
                if w == query {
                    return Some(start_pos + i);
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Alphabet, fasta::read_sequences};

    #[test]
    fn groundtruthquery_success() {
        let sequence = "ACTGACCCGTAGCGCTA".as_bytes();
        let k = 3;
        let w = 3;
        let alphabet = Alphabet::from_bytes(sequence);
        let kmers = KmerSequence::from_bytes(sequence, k, alphabet);
        let std_suffix_array = SuffixArray::<StandardQuery>::from_kmers(kmers, w, ());
        let alphabet = Alphabet::from_bytes(sequence);
        let kmers = KmerSequence::from_bytes(sequence, k, alphabet);
        let gt_suffix_array = SuffixArray::<GroundTruthQuery>::from_kmers(kmers, w, ());

        for query_len in 5..sequence.len() {
            for (i, window) in sequence.windows(query_len).enumerate() {
                assert_eq!(
                    std_suffix_array.query(window),
                    gt_suffix_array.query(window)
                );
            }
        }
    }

    #[test]
    fn standardquery_success() {
        let sequence = "ACTGACCCGTAGCGCTA".as_bytes();
        for k in 2..sequence.len() {
            for w in 2..sequence.len() - k + 1 {
                let alphabet = Alphabet::from_bytes(sequence);
                let kmers = KmerSequence::from_bytes(sequence, k, alphabet);
                let suffix_array = SuffixArray::<StandardQuery>::from_kmers(kmers, w, ());
                println!("{:#?}", suffix_array);

                for query_len in (k + w - 1)..sequence.len() {
                    for (i, window) in sequence.windows(query_len).enumerate() {
                        dbg!(std::str::from_utf8(&window).unwrap());
                        assert_eq!(suffix_array.query(window), Some(i));
                    }
                }
            }
        }
    }
    
    #[test]
    fn standardquery_nomatch() {
        let sequence = "ACTGACCCGTAGCGCTA".as_bytes();
        let k = 3;
        let w = 3;
        let alphabet = Alphabet::from_bytes(sequence);
        let kmers = KmerSequence::from_bytes(sequence, k, alphabet);
        let suffix_array = SuffixArray::<StandardQuery>::from_kmers(kmers, w, ());

        for query_len in 5..sequence.len() {
            for (i, window) in sequence.windows(query_len).enumerate() {
                let mut window = window.to_owned();
                window[0] = if window[0] != b'A' { b'A' } else { b'C' };
                assert_eq!(suffix_array.query(&window), None);
            }
        }

        for query_len in 5..sequence.len() {
            for (i, window) in sequence.windows(query_len).enumerate() {
                let mut window = window.to_owned();
                window[query_len - 1] = if window[query_len - 1] != b'A' { b'A' } else { b'C' };
                assert_eq!(suffix_array.query(&window), None);
            }
        }

        for query_len in 5..sequence.len() {
            for (i, window) in sequence.windows(query_len).enumerate() {
                let mut window = window.to_owned();
                window[1] = if window[1] != b'A' { b'A' } else { b'C' };
                assert_eq!(suffix_array.query(&window), None);
            }
        }
    }

    #[test]
    fn assignment1_test_data() {
        let genome_file =
            read_sequences("test_input/a1-tests/test_input/salmonella_sub.fa").unwrap();
        let query_file = read_sequences("test_input/a1-tests/test_input/reads_sal_sub.fq").unwrap();

        let sequence = genome_file.get(0).unwrap().representation.as_slice();
        let queries: Vec<&[u8]> = query_file
            .iter()
            .map(|q| q.representation.as_slice())
            .collect();

        let k = 3;
        let w = 3;
        let alphabet = Alphabet::from_bytes(sequence);
        let kmers = KmerSequence::from_bytes(sequence, k, alphabet);
        let suffix_array_standard = SuffixArray::<StandardQuery>::from_kmers(kmers, w, ());

        let alphabet = Alphabet::from_bytes(sequence);
        let kmers = KmerSequence::from_bytes(sequence, k, alphabet);
        let suffix_array_ground_truth = SuffixArray::<GroundTruthQuery>::from_kmers(kmers, w, ());

        for query in queries {
            let result = suffix_array_standard.query(query);
            match result {
                Some(i) => {
                    // ensure that the string is actually present
                    let slice = &sequence[i..(i + query.len())];
                    assert_eq!(slice, query);
                }
                None => assert!(suffix_array_ground_truth.query(query).is_none()),
            }

            // The below doesn't work because StandardQuery and GroundTruthQuery might return
            // different occurences of the same query in the genome. Duh!
            // assert_eq!(suffix_array_standard.query(query), suffix_array_ground_truth.query(query));
        }
    }
}
