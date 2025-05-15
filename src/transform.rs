use std::{cmp::Ordering, collections::HashSet, fmt};

use bimap::BiMap;
use itertools::Itertools;

use crate::int_vec::IntVec;

/// A mapping from u8s in the original string to
/// u8s that have been compressed into a smaller domain
#[derive(Clone)]
pub struct Alphabet(BiMap<u8, u8>);

/// Represents a single k-mer.
#[derive(Clone, Eq, PartialEq)]
pub enum Kmer {
    Data(IntVec),
    Sentinel,
}

#[derive(Debug, Eq, PartialEq)]
pub struct SuperKmer {
    // The starting position of the super-kmer in the underlying string
    pub start_pos: usize,

    // The length of the super-kmer in the underlying string
    #[allow(dead_code)]
    pub length: usize,

    // The minimizer kmer
    pub minimizer: Kmer,
}

#[derive(Debug)]
pub struct KmerSequence {
    alphabet: Alphabet,

    kmers: Vec<Kmer>,

    // TODO: can we efficiently compute this from kmers?
    original_string: Vec<u8>,

    k: usize,
}

impl Alphabet {
    pub fn from_bytes(sequence: &[u8]) -> Self {
        // Construct a mapping from u8 -> compressed u8 of the
        // bytes in the original sequence
        let mut alphabet = BiMap::new();

        // BTreeSet is helpful to keep ordering the same in original
        // and transformed alphabets
        //let bytes_seen: BTreeSet<u8> = BTreeSet::from_iter(sequence.iter().cloned());

        let bytes_seen: HashSet<u8> = HashSet::from_iter(sequence.iter().cloned());
        let mut bytes_seen: Vec<_> = Vec::from_iter(bytes_seen.iter().cloned());
        // TODO: This sort should be based on the ordering!
        bytes_seen.sort();

        for (i, b) in bytes_seen.iter().cloned().enumerate() {
            // i should obviously be up to 255 since bytes_seen is a set of unique u8's
            let i: u8 = i.try_into().unwrap();

            alphabet.insert(b, i);
        }

        Self(alphabet)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl KmerSequence {
    pub fn from_bytes(sequence: &[u8], k: usize, alphabet: Alphabet) -> Self {
        assert!(k > 0 && k <= sequence.len());

        // Compute the number of bits we need to store a single underlying character
        let bits_underlying = (alphabet.len() as u8 + 1).ilog2();

        // Construct a sequence of Kmers
        let kmers: Vec<_> = sequence
            .windows(k)
            .map(|window| {
                Kmer::Data(IntVec::from_iter(
                    bits_underlying.try_into().unwrap(),
                    window
                        .iter()
                        // TODO: Change return type to Option<Self> and remove unwrap().
                        .map(|b| alphabet.0.get_by_left(b).unwrap().clone()),
                ))
            })
            .collect();

        Self {
            kmers,
            k,
            alphabet,
            original_string: sequence.to_owned(),
        }
    }

    pub fn get_original_string(&self) -> &[u8] {
        &self.original_string
    }

    pub fn get_original_string_len(&self) -> usize {
        self.original_string.len()
    }

    // Panics if the kmer isn't a part of this KmerSequence
    pub fn compare_kmers(&self, left: &Kmer, right: &Kmer) -> Ordering {
        match (left, right) {
            (Kmer::Sentinel, Kmer::Sentinel) => Ordering::Equal,
            (Kmer::Sentinel, _) => Ordering::Greater,
            (_, Kmer::Sentinel) => Ordering::Less,
            (Kmer::Data(d1), Kmer::Data(d2)) => d1.iter().cmp(d2.iter()),
        }
    }

    // TODO: switch from the naive approach to something more efficient
    pub fn compute_minimizer_chain(&self, w: usize) -> Vec<&Kmer> {
        assert!(self.kmers.len() >= w);
        assert!(w >= 1);

        // Find the minimizers for each k-mer window
        self.kmers
            .windows(w)
            .map(|window| {
                // The minimizer kmer in this window
                window
                    .iter()
                    .min_by(|&kmer1, &kmer2| self.compare_kmers(kmer1, kmer2))
                    .unwrap()
            })
            .collect()
    }

    pub fn compute_super_kmers(&self, w: usize) -> Vec<SuperKmer> {
        // Compute the minimizer chain
        let mut minimizers = self.compute_minimizer_chain(w).into_iter().enumerate();

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
                    length: curr_count + (w + self.k - 1) - 1,
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
            length: curr_count + (w + self.k - 1) - 1,
            minimizer: curr_minimizer.clone(),
        });

        super_kmers
    }

    pub fn alphabet(&self) -> Alphabet {
        self.alphabet.clone()
    }

    pub fn k(&self) -> usize {
        self.k
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

#[cfg(test)]
mod tests {
    use super::*;
}
