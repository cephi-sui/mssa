use anyhow::Result;

use crate::kmer::{Kmer, StupidOrd, SuperKmer};

pub struct SuffixArray(Vec<usize>);

impl SuffixArray {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_superkmers(super_rep: &[SuperKmer]) -> Self {
        let mut suffixes: Vec<(usize, &[SuperKmer])> = (0..super_rep.len())
            .map(|i| (i, &super_rep[i..super_rep.len()]))
            .collect();
        suffixes.sort_by(|&(_, a), &(_, b)| a.stupid_cmp(&b));
        Self(suffixes.iter().map(|&(i, _)| i).collect())
    }

    /*
    fn binary_search(
        pattern: Kmer,
        index: &Index,
        mut left: usize,
        mut right: usize,
    ) -> usize {
        loop {
            let center = (right + left) / 2;
            let order = pattern.cmp(
                &index.sequence[index.suffix_array[center].start_idx..]
            );
            match order {
                Ordering::Less => {
                    if center == left + 1 {
                        break center;
                    } else if center == left && center == right {
                        break center;
                    }
                    right = center;
                }
                Ordering::Greater => {
                    if center == right - 1 {
                        break right;
                    }
                    left = center;
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }
    */
}
