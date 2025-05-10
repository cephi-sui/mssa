use crate::iter_order_by::MyIterOrderBy;
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

        Self {
            underlying_kmers: kmers,
            super_kmers,
            suffix_array,
        }
    }
}
