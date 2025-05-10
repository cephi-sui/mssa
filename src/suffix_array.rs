use crate::transform::{KmerSequence, SuperKmer};

pub struct SuffixArray {
    underlying_kmers: KmerSequence,

    super_kmers: Vec<SuperKmer>,
    suffix_array: Vec<usize>,
}

impl SuffixArray {
    pub fn from_kmers(kmers: KmerSequence, w: usize) -> Self {
        // Construct the suffix array
        let super_kmers = kmers.compute_super_kmers(w);


        Self {
            underlying_kmers: kmers,
            super_kmers: todo!(),
            suffix_array: todo!(),
        }
    }
}
