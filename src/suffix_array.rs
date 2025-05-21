use std::cmp::Ordering;

use bincode::{Decode, Encode};
use fastbloom::BloomFilter;
use plr::regression::GreedyPLR;

use crate::iter_order_by::MyIterOrderBy;
use crate::transform::{Kmer, KmerSequence, SuperKmer, MinimizerOrder};

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
/// let suffix_array = SuffixArray::<StandardQuery>::from_kmers(kmers, w,
/// MinimizerOrder::Lexicographic, ());
/// ```
#[derive(Debug, Encode, Decode)]
pub struct SuffixArray<T> {
    underlying_kmers: KmerSequence,
    w: usize,
    minimizer_order: MinimizerOrder,

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

pub trait Queryable {
    fn query(&self, query: &[u8]) -> (Vec<usize>, usize);
}

#[derive(Encode, Decode)]
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

#[derive(Debug, Encode, Decode)]
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
#[derive(Encode, Decode)]
pub struct BloomFilterQuery {
    #[bincode(with_serde)]
    example: BloomFilter,
}

impl QueryMode for BloomFilterQuery {
    /// False positive rate
    type InitParams = f32;

    fn initialize_aux_data(
        _kmers: &KmerSequence,
        _w: usize,
        _suffix_array: &[&[SuperKmer]],
        _init_params: Self::InitParams,
    ) -> Self {
        todo!()
    }
}

impl<T: QueryMode> SuffixArray<T> {
    // put any methods that don't need to touch query_mode_aux_data here

    pub fn from_kmers(mut kmers: KmerSequence, w: usize, o: MinimizerOrder, init_params: T::InitParams) -> Self {
        // Generate occurrence HashMap.
        if o == MinimizerOrder::Occurrence {
            kmers.generate_occ();
        }
        let kmers = kmers; // Drop mutability.
        // Construct the suffix array
        let mut super_kmers = kmers.compute_super_kmers(w, o, None).unwrap();
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
            minimizer_order: o,
            super_kmers,
            suffix_array,
            query_mode_aux_data,
        }
    }

    pub fn get_underlying_kmers(&self) -> &KmerSequence {
        &self.underlying_kmers
    }

    pub fn w(&self) -> usize {
        self.w
    }
}

// The ground truth query mode which performs an extremely inefficient query for testing purposes.
impl Queryable for SuffixArray<GroundTruthQuery> {
    fn query(&self, query: &[u8]) -> (Vec<usize>, usize) {
        let ref_str = self.underlying_kmers.get_original_string();
        let mut result = Vec::new();
        for (i, window) in ref_str.windows(query.len()).enumerate() {
            if query == window {
                //return (Some(i), false);
                result.push(i);
            }
        }
        //(None, false)
        (result, 0)
    }
}

// The standard query mode, with no accelerant data structures
impl Queryable for SuffixArray<StandardQuery> {
    fn query(&self, query: &[u8]) -> (Vec<usize>, usize) {
        assert!(
            query.len() >= self.w + self.underlying_kmers.k() - 1,
            "query length was shorter than minimum length required by w + k - 1"
        );

        let query_kmers = KmerSequence::from_bytes(
            query,
            self.underlying_kmers.k(),
            self.underlying_kmers.alphabet(),
        );
        let query_super_kmers = query_kmers.compute_super_kmers(self.w, self.minimizer_order, Some(&self.underlying_kmers));
        let Some(query_super_kmers) = query_super_kmers else { return (Vec::new(), 0) };

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
        let left_idx = self.suffix_array.partition_point(|&s| {
            cmp_slice_to_query(&self.super_kmers[s..self.super_kmers.len()]) == Ordering::Less
        });
        // Look for first index in suffix array > kmer
        let right_idx = self.suffix_array.partition_point(|&s| {
            cmp_slice_to_query(&self.super_kmers[s..self.super_kmers.len()]) != Ordering::Greater
        });

        if left_idx == right_idx {
            // Query not present
            //return (None, false);
            return (Vec::new(), 0);
        }

        // Query could be present anywhere in the range
        let mut result = Vec::new();
        let mut false_positives = 0;
        let original_string = self.underlying_kmers.get_original_string();
        for i in left_idx..right_idx {
            let super_kmers = &self.super_kmers[self.suffix_array[i]..self.super_kmers.len()]
                [0..query_super_kmers.len()];

            let first_super_kmer = super_kmers.first().unwrap();
            let last_super_kmer = super_kmers.last().unwrap();
            let start_pos = first_super_kmer.start_pos;
            let end_pos = last_super_kmer.start_pos + last_super_kmer.length;

            let mut found = false;
            for (i, w) in original_string[start_pos..end_pos]
                .windows(query.len())
                .enumerate()
            {
                if w == query {
                    //return (Some(start_pos + i), false);
                    found = true;
                    result.push(start_pos + i);
                    break;
                }
            }

            if found == false {
                false_positives += 1;
            }
        }

        //(None, true)
        (result, false_positives)
    }
}

#[derive(Encode, Decode)]
pub struct PWLLearnedQuery {
    // TODO: find a more efficient way to do lookups among the segments?
    #[bincode(with_serde)]
    plr_begin_segments: Vec<plr::Segment>,
    #[bincode(with_serde)]
    plr_end_segments: Vec<plr::Segment>,

    gamma: f64,
}

impl QueryMode for PWLLearnedQuery {
    /// gamma, the maximum error used in piecewise linear regression
    /// TODO: add an option to take the first x super-kmers, rather than just the first one
    type InitParams = f64;

    fn initialize_aux_data(
        kmers: &KmerSequence,
        _w: usize,
        suffix_array: &[&[SuperKmer]],
        init_params: Self::InitParams,
    ) -> Self {
        let gamma: f64 = init_params;

        let sa_len = suffix_array.len();
        let mut suffix_array = suffix_array.into_iter().enumerate();

        let mut ranges: Vec<(u128, usize, usize)> = Vec::new();
        let mut curr_start_i = 0;
        let mut curr_start_kmer =
            kmers.kmer_to_integer(&suffix_array.next().unwrap().1.first().unwrap().minimizer);
        for (i, &suffix) in suffix_array {
            let start_kmer = &suffix.first().unwrap().minimizer;

            if *start_kmer == Kmer::Sentinel {
                continue;
            }

            let start_kmer = kmers.kmer_to_integer(start_kmer);

            if start_kmer != curr_start_kmer {
                // Add the "previous" range from curr_start_i to i
                ranges.push((curr_start_kmer, curr_start_i, i - 1));

                // reset the current element
                curr_start_i = i;
                curr_start_kmer = start_kmer;
            }
        }
        // Special case: add the last range
        ranges.push((curr_start_kmer, curr_start_i, sa_len - 1));

        println!("number of distinct k-kmers: {:?}", ranges.len());

        // Construct the piecewise approximation functions
        // (begin for the beginning of ranges, end for end)
        let mut plr_begin = GreedyPLR::new(gamma);
        let mut plr_end = GreedyPLR::new(gamma);
        let mut plr_begin_segments = Vec::new();
        let mut plr_end_segments = Vec::new();
        for (kmer, begin, end) in ranges.clone() {
            // handle begin
            if let Some(segment) = plr_begin.process(kmer as f64, begin as f64) {
                plr_begin_segments.push(segment);
            }

            // handle end
            if let Some(segment) = plr_end.process(kmer as f64, end as f64) {
                plr_end_segments.push(segment);
            }
        }

        // Special case: flush the buffer for last element
        if let Some(segment) = plr_begin.finish() {
            plr_begin_segments.push(segment);
        }
        if let Some(segment) = plr_end.finish() {
            plr_end_segments.push(segment);
        }

        Self {
            plr_begin_segments,
            plr_end_segments,
            gamma,
        }
    }
}

impl Queryable for SuffixArray<PWLLearnedQuery> {
    fn query(&self, query: &[u8]) -> (Vec<usize>, usize) {
        assert!(
            query.len() >= self.w + self.underlying_kmers.k() - 1,
            "query length was shorter than minimum length required by w + k - 1"
        );

        let query_kmers = KmerSequence::from_bytes(
            query,
            self.underlying_kmers.k(),
            self.underlying_kmers.alphabet(),
        );
        let query_super_kmers = query_kmers.compute_super_kmers(self.w, self.minimizer_order, Some(&self.underlying_kmers));
        let Some(query_super_kmers) = query_super_kmers else { return (Vec::new(), 0) };

        let cmp_slice_to_query = |slice: &[SuperKmer]| {
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

        let sa_len = self.suffix_array.len();

        // Get the start/end bound from the PWL function
        // TODO: should `error` be a little larger here to account for floating-point error going
        // from u128 to f64?
        let error = self.query_mode_aux_data.gamma.ceil() as usize;
        let first_kmer: u128 = self
            .underlying_kmers
            .kmer_to_integer(&query_super_kmers.first().unwrap().minimizer);
        let lookup = |segments: &[plr::Segment], x: u128| {
            let x = x as f64;
            let segment = segments
                .iter()
                .find(|&segment| segment.start <= x && x < segment.stop)
                .unwrap();
            segment.slope * x + segment.intercept
        };
        // Compute bounds from PWL function
        let left_bound = lookup(&self.query_mode_aux_data.plr_begin_segments, first_kmer) as usize;
        let right_bound =
            lookup(&self.query_mode_aux_data.plr_end_segments, first_kmer).ceil() as usize;
        // Account for error and clamp accordingly
        let left_bound = left_bound.saturating_sub(error);
        let right_bound = right_bound.saturating_add(error).clamp(0, sa_len - 1);
        let suffix_array = &self.suffix_array[left_bound..(right_bound + 1)];

        // println!(
        //     "starting binary search over bounds {:?}..{:?}",
        //     left_bound, right_bound
        // );
        // println!(
        //     "  --> {:?} elements ({:.3?}% of suffix array)",
        //     right_bound - left_bound + 1,
        //     (right_bound - left_bound + 1) as f64 / self.suffix_array.len() as f64 * 100.0
        // );

        // Look for first index in suffix array == kmer
        let left_idx = suffix_array.partition_point(|&s| {
            cmp_slice_to_query(&self.super_kmers[s..self.super_kmers.len()]) == Ordering::Less
        });
        // Look for first index in suffix array > kmer
        let right_idx = suffix_array.partition_point(|&s| {
            cmp_slice_to_query(&self.super_kmers[s..self.super_kmers.len()]) != Ordering::Greater
        });

        if left_idx == right_idx {
            // Query not present
            //return (None, false);
            return (Vec::new(), 0);
        }

        // Query could be present anywhere in the range
        let mut result = Vec::new();
        let mut false_positives = 0;
        let original_string = self.underlying_kmers.get_original_string();
        for i in left_idx..right_idx {
            let super_kmers = &self.super_kmers[suffix_array[i]..self.super_kmers.len()]
                [0..query_super_kmers.len()];

            let first_super_kmer = super_kmers.first().unwrap();
            let last_super_kmer = super_kmers.last().unwrap();
            let start_pos = first_super_kmer.start_pos;
            let end_pos = last_super_kmer.start_pos + last_super_kmer.length;

            let mut found = false;
            for (i, w) in original_string[start_pos..end_pos]
                .windows(query.len())
                .enumerate()
            {
                if w == query {
                    //return (Some(start_pos + i), false);
                    found = true;
                    result.push(start_pos + i);
                    break;
                }
            }

            if found == false {
                false_positives += 1;
            }
        }

        //(None, true)
        (result, false_positives)
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
        let std_suffix_array = SuffixArray::<StandardQuery>::from_kmers(kmers, w, MinimizerOrder::Lexicographic, ());
        let alphabet = Alphabet::from_bytes(sequence);
        let kmers = KmerSequence::from_bytes(sequence, k, alphabet);
        let gt_suffix_array = SuffixArray::<GroundTruthQuery>::from_kmers(kmers, w, MinimizerOrder::Lexicographic, ());

        for query_len in 5..sequence.len() {
            for (i, window) in sequence.windows(query_len).enumerate() {
                assert_eq!(
                    std_suffix_array.query(window).0,
                    gt_suffix_array.query(window).0
                );
            }
        }
    }

    #[test]
    fn standardquery_success() {
        let sequence = "ACTGACCCGTAGCGCTA".as_bytes();
        for o in [MinimizerOrder::Lexicographic, MinimizerOrder::Occurrence].into_iter() {
            for k in 1..sequence.len() {
                for w in 1..sequence.len() - k + 1 {
                    let alphabet = Alphabet::from_bytes(sequence);
                    let kmers = KmerSequence::from_bytes(sequence, k, alphabet);
                    let suffix_array = SuffixArray::<StandardQuery>::from_kmers(kmers, w, o, ());

                    let alphabet = Alphabet::from_bytes(sequence);
                    let kmers = KmerSequence::from_bytes(sequence, k, alphabet);
                    let suffix_array_gt = SuffixArray::<GroundTruthQuery>::from_kmers(kmers, w, o, ());

                    println!("{:#?}", suffix_array);

                    for query_len in (k + w - 1)..sequence.len() {
                        for window in sequence.windows(query_len) {
                            dbg!(std::str::from_utf8(&window).unwrap());
                            match suffix_array.query(window).0 {
                                Some(i) => assert_eq!(&sequence[i..(i + window.len())], window),
                                None => assert!(suffix_array_gt.query(window).0.is_none()),
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn standardquery_nomatch() {
        let sequence = "ACTGACCCGTAGCGCTA".as_bytes();
        for o in [MinimizerOrder::Lexicographic, MinimizerOrder::Occurrence].into_iter() {
            let k = 3;
            let w = 3;
            let alphabet = Alphabet::from_bytes(sequence);
            let kmers = KmerSequence::from_bytes(sequence, k, alphabet);
            let suffix_array = SuffixArray::<StandardQuery>::from_kmers(kmers, w, o, ());

            for query_len in 5..sequence.len() {
                for (i, window) in sequence.windows(query_len).enumerate() {
                    let mut window = window.to_owned();
                    window[0] = if window[0] != b'A' { b'A' } else { b'C' };
                    assert_eq!(suffix_array.query(&window).0, None);
                }
            }

            for query_len in 5..sequence.len() {
                for (i, window) in sequence.windows(query_len).enumerate() {
                    let mut window = window.to_owned();
                    window[query_len - 1] = if window[query_len - 1] != b'A' {
                        b'A'
                    } else {
                        b'C'
                    };
                    assert_eq!(suffix_array.query(&window).0, None);
                }
            }

            for query_len in 5..sequence.len() {
                for (i, window) in sequence.windows(query_len).enumerate() {
                    let mut window = window.to_owned();
                    window[1] = if window[1] != b'A' { b'A' } else { b'C' };
                    assert_eq!(suffix_array.query(&window).0, None);
                }
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

        let k = 5;
        let w = 3;
        let alphabet = Alphabet::from_bytes(sequence);
        let kmers = KmerSequence::from_bytes(sequence, k, alphabet);
        let suffix_array_standard = SuffixArray::<PWLLearnedQuery>::from_kmers(kmers, w, MinimizerOrder::Lexicographic, 1000.0);

        let alphabet = Alphabet::from_bytes(sequence);
        let kmers = KmerSequence::from_bytes(sequence, k, alphabet);
        let suffix_array_ground_truth = SuffixArray::<GroundTruthQuery>::from_kmers(kmers, w, MinimizerOrder::Lexicographic, ());

        for query in queries {
            let result = suffix_array_standard.query(query).0;
            match result {
                Some(i) => {
                    // ensure that the string is actually present
                    let slice = &sequence[i..(i + query.len())];
                    assert_eq!(slice, query);
                }
                None => assert!(suffix_array_ground_truth.query(query).0.is_none()),
            }

            // The below doesn't work because StandardQuery and GroundTruthQuery might return
            // different occurences of the same query in the genome. Duh!
            // assert_eq!(suffix_array_standard.query(query), suffix_array_ground_truth.query(query));
        }
    }

    #[test]
    fn assignment1_test_data_occurrence() {
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
        let suffix_array_standard = SuffixArray::<StandardQuery>::from_kmers(kmers, w, MinimizerOrder::Occurrence, ());

        let alphabet = Alphabet::from_bytes(sequence);
        let kmers = KmerSequence::from_bytes(sequence, k, alphabet);
        let suffix_array_ground_truth = SuffixArray::<GroundTruthQuery>::from_kmers(kmers, w, MinimizerOrder::Occurrence, ());

        for query in queries {
            let result = suffix_array_standard.query(query).0;
            match result {
                Some(i) => {
                    // ensure that the string is actually present
                    let slice = &sequence[i..(i + query.len())];
                    assert_eq!(slice, query);
                }
                None => assert!(suffix_array_ground_truth.query(query).0.is_none()),
            }
        }
    }
}
