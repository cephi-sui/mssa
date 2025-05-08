use std::{cmp::Ordering, collections::BTreeSet, fmt};

use bimap::BiMap;
use itertools::Itertools;

use crate::int_vec::IntVec;

/// A mapping from u8s in the original string to
/// u8s that have been compressed into a smaller domain
struct Alphabet(BiMap<u8, u8>);

/// Represents a single k-mer.
#[derive(Clone)]
enum Kmer {
    Data(IntVec),
    Sentinel,
}

#[derive(Debug)]
pub struct SuperKmer {
    // The starting position of the super-kmer in the underlying string
    start_pos: usize,

    // The length of the super-kmer in the underlying string
    length: usize,

    // The minimizer kmer
    minimizer: Kmer,
}

// TODO: should SuperKmerSequence be a separate type? That way we
// could more cleanly keep track of `w`.

#[derive(Debug)]
pub struct KmerSequence {
    alphabet: Alphabet,

    kmers: Vec<Kmer>,

    k: usize,
}

impl KmerSequence {
    pub fn from_bytes(sequence: &[u8], k: usize) -> Self {
        // Construct a mapping from u8 -> compressed u8 of the
        // bytes in the original sequence
        let mut alphabet = BiMap::new();
        // BTreeSet is helpful to keep ordering the same in original
        // and transformed alphabets
        let bytes_seen: BTreeSet<u8> = BTreeSet::from_iter(sequence.iter().cloned());
        for (i, b) in bytes_seen.iter().cloned().enumerate() {
            // i should obviously be up to 255 since bytes_seen is a set of unique u8's
            let i: u8 = i.try_into().unwrap();

            alphabet.insert(b, i);
        }

        // Compute the number of bits we need to store a single
        // underlying character
        let bits_underlying = u8::BITS - (bytes_seen.len() as u8).leading_zeros();

        // Construct a sequence of Kmers
        let mut kmers: Vec<_> = sequence
            .windows(k)
            .map(|window| {
                Kmer::Data(IntVec::from_iter(
                    bits_underlying.try_into().unwrap(),
                    window
                        .iter()
                        .map(|b| alphabet.get_by_left(b).unwrap().clone()),
                ))
            })
            .collect();

        // Make sure we always have a sentinel kmer
        kmers.push(Kmer::Sentinel);

        Self {
            kmers,
            k,
            alphabet: Alphabet(alphabet),
        }
    }

    // Panics if the kmer isn't a part of this KmerSequence
    fn compare_kmers(&self, left: &Kmer, right: &Kmer) -> Ordering {
        match (left, right) {
            (Kmer::Sentinel, Kmer::Sentinel) => Ordering::Equal,
            (Kmer::Sentinel, _) => Ordering::Greater,
            (_, Kmer::Sentinel) => Ordering::Less,
            (Kmer::Data(d1), Kmer::Data(d2)) => d1.iter().cmp(d2.iter()),
        }
    }

    // TODO: switch from the naive approach to something more efficient
    pub fn compute_super_kmers(&self, w: usize) -> Vec<SuperKmer> {
        assert!(self.kmers.len() >= w);
        assert!(w >= 1);

        // Find the minimizers for each k-mer window
        let mut minimizers = self.kmers.windows(w).enumerate().map(|(i, window)| {
            (
                // The start position in the original string of this window
                i,
                // The minimizer kmer in this window
                window
                    .iter()
                    .min_by(|&kmer1, &kmer2| self.compare_kmers(kmer1, kmer2))
                    .unwrap(),
            )
        });

        // De-duplication (taking the first start position and accumulating lengths)
        let mut super_kmers: Vec<SuperKmer> = Vec::new();
        let (mut curr_start_i, mut curr_minimizer) = minimizers.next().unwrap();
        let mut curr_count = 1;
        for (i, minimizer) in minimizers {
            if self.compare_kmers(minimizer, curr_minimizer) == Ordering::Equal {
                // deduplicate current minimizer
                curr_count += 1;
            } else {
                // Add the "previous" minimizer, de-duplicated
                super_kmers.push(SuperKmer {
                    start_pos: curr_start_i,
                    length: curr_count + self.k - 1,
                    minimizer: curr_minimizer.clone(),
                });

                // reset the current element
                (curr_start_i, curr_minimizer) = (i, minimizer);
                curr_count = 1;
            }
        }
        // Special case: add the last element
        super_kmers.push(SuperKmer {
            start_pos: curr_start_i,
            length: curr_count + self.k - 1,
            minimizer: curr_minimizer.clone(),
        });

        super_kmers
    }
}

impl fmt::Debug for Kmer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Kmer::Data(d) => write!(f, "Kmer [{}]", d.iter().format(", ")),
            Kmer::Sentinel => write!(f, "Kmer $"),
        }
    }
}

impl fmt::Debug for Alphabet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Alphabet [{}]",
            self.0
                .iter()
                .map(|(&from, &to)| format!("{} <> {}", from as char, to))
                .format(", ")
        )
    }
}
