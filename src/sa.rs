use anyhow::Result;

use crate::kmer::{Kmer, SuperKmer};

pub struct SuffixArray(Vec<usize>);

impl SuffixArray {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_superkmers(super_rep: &[SuperKmer], rep: &[u8]) -> Self {
        let mut suffixes: Vec<&[SuperKmer]> = (0..super_rep.len())
            .map(|i| &super_rep[i..super_rep.len()])
            .collect();
        //suffixes.sort_by(|&a, &b| {});
        // Self(suffixes)
        todo!()
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
