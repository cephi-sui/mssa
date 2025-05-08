use std::{collections::HashSet, fmt};

use bimap::BiMap;
use itertools::Itertools;

use crate::int_vec::IntVec;

/// Represents a single k-mer.
enum Kmer {
    Data(IntVec),
    Sentinel,
}

pub struct KmerSequence {
    kmers: Vec<Kmer>,

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

        Self { kmers, alphabet }
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
                            .map(|b| {
                                self.alphabet
                                    .get_by_right(&b.try_into().unwrap())
                                    .unwrap()
                                    .clone() as char
                            })
                            .format(", ")
                    ),
                })
                .format(",\n")
        )
    }
}
