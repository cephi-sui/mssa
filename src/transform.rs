use std::{cmp::Ordering, collections::HashSet, fmt};

use bimap::BiMap;
use itertools::Itertools;

use crate::int_vec::IntVec;

/// Represents a single k-mer.
enum Kmer {
    Data(IntVec),
    Sentinel,
}

pub struct SuperKmer {
    // The starting position of the super-kmer in the underlying string
    start_pos: usize,

    // The length of the super-kmer in the underlying string
    length: usize,

    // The minimizer kmer
    minimizer: Kmer,
}

pub struct KmerSequence {
    kmers: Vec<Kmer>,

    k: usize,

    /// A mapping from u8s in the original string to
    /// u8s that have been compressed into a smaller domain
    alphabet: BiMap<u8, u8>,
}

impl KmerSequence {
    pub fn from_bytes(sequence: &[u8], k: usize) -> Self {
        // Construct a mapping from u8 -> compressed u8 of the
        // bytes in the original sequence
        let mut alphabet = BiMap::new();
        let bytes_seen: HashSet<u8> = HashSet::from_iter(sequence.iter().cloned());
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

        Self { kmers, k, alphabet }
    }

    // Panics if the kmer isn't a part of this KmerSequence
    fn compare_kmers(&self, left: &Kmer, right: &Kmer) -> Ordering {
        match (left, right) {
            (Kmer::Sentinel, Kmer::Sentinel) => Ordering::Equal,
            (Kmer::Sentinel, _) => Ordering::Greater,
            (_, Kmer::Sentinel) => Ordering::Less,
            (Kmer::Data(vec1), Kmer::Data(vec2)) => {
                // Do the comparison in the original string alphabet for now
                let left = vec1.iter().map(|b| self.alphabet.get_by_right(&b).unwrap());
                let right = vec2.iter().map(|b| self.alphabet.get_by_right(&b).unwrap());

                left.cmp(right)
            }
        }
    }

    pub fn compute_super_kmers(&self, w: usize) -> Vec<SuperKmer> {
        assert!(self.kmers.len() >= w);
        assert!(w >= 1);

        // Find the minimizers for each k-mer window
        let minimizers = self.kmers.windows(w).enumerate().map(|(i, window)| {
            (
                // The start position in the original string of this window
                i * self.k,
                // The minimizer kmer in this window
                window
                    .iter()
                    .min_by(|&kmer1, &kmer2| self.compare_kmers(kmer1, kmer2))
                    .unwrap(),
            )
        });

        // TODO: de-duplication!

        todo!()
    }
}

impl fmt::Debug for KmerSequence {
    // uhh dont worry about this too much
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "KmerSequence {{ alphabet = {}, kmers = [\n{}\n] }}",
            self.alphabet
                .iter()
                .map(|(&from, &to)| format!("{} -> {}", to, from as char))
                .format(", "),
            self.kmers
                .iter()
                .map(|kmer| match kmer {
                    Kmer::Sentinel => format!("Kmer $"),
                    Kmer::Data(d) => format!(
                        "Kmer [{}]",
                        d.iter()
                            .map(|b| { self.alphabet.get_by_right(&b).unwrap().clone() as char })
                            .format(", ")
                    ),
                })
                .format(",\n")
        )
    }
}
