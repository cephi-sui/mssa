use anyhow::Result;
use std::{
    cmp::{self, Ord, Ordering},
    collections::{VecDeque, vec_deque},
};

#[derive(Clone, Copy, Debug)]
pub enum Kmer<'a> {
    Data(&'a [u8]),
    Sentinel,
}

#[derive(Debug)]
pub struct SuperKmer<'a> {
    pub start_pos: usize,
    pub length: usize,
    pub minimizer: Kmer<'a>,
}

pub trait StupidOrd {
    fn stupid_cmp(&self, other: &Self) -> Ordering;
}

impl<'a> StupidOrd for Kmer<'a> {
    fn stupid_cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (&Kmer::Sentinel, &Kmer::Sentinel) => Ordering::Equal,
            (&Kmer::Data(d1), &Kmer::Data(d2)) => d1.cmp(d2),
            (&Kmer::Sentinel, &Kmer::Data(_)) => Ordering::Greater,
            (&Kmer::Data(_), &Kmer::Sentinel) => Ordering::Less,
        }
    }
}

impl<'a> StupidOrd for SuperKmer<'a> {
    fn stupid_cmp(&self, other: &Self) -> Ordering {
        self.minimizer.stupid_cmp(&other.minimizer)
    }
}

impl<'a> StupidOrd for &[Kmer<'a>] {
    fn stupid_cmp(&self, other: &Self) -> Ordering {
        // Ripped from core/slice/cmp.rs
        let l = cmp::min(self.len(), other.len());

        // Slice to the loop iteration range to enable bound check
        // elimination in the compiler
        let lhs = &self[..l];
        let rhs = &other[..l];

        for i in 0..l {
            match lhs[i].stupid_cmp(&rhs[i]) {
                Ordering::Equal => (),
                non_eq => return non_eq,
            }
        }

        self.len().cmp(&other.len())
    }
}

impl<'a> StupidOrd for &[SuperKmer<'a>] {
    fn stupid_cmp(&self, other: &Self) -> Ordering {
        let s = self.iter().map(|sk| sk.minimizer).collect::<Vec<_>>();
        let o = other.iter().map(|sk| sk.minimizer).collect::<Vec<_>>();
        s.as_slice().stupid_cmp(&o.as_slice())
    }
}

pub fn to_kmers<'a>(representation: &'a [u8], k: usize) -> Vec<Kmer<'a>> {
    representation
        .windows(k)
        .map(|slice| Kmer::Data(slice))
        .collect()
}

// (Not currently) Based on DP solution at https://algo.monster/liteproblems/239
pub fn construct_super_kmers<'a>(kmers: &'a [Kmer], w: usize) -> Vec<SuperKmer<'a>> {
    assert!(kmers.len() >= w);

    let mut result = Vec::new();
    //let mut start_pos = 0;
    //let mut minimizer = &kmers[0];

    let minimizers = kmers
        .iter()
        .enumerate()
        .collect::<Vec<_>>()
        .windows(w)
        .map(|slice| {
            let mut minimizer = slice[0];
            for (i, kmer) in slice.iter().skip(1) {
                if kmer.stupid_cmp(&minimizer.1) == Ordering::Less {
                    minimizer = (*i, kmer);
                }
            }
            minimizer
        });

    for minimizer in minizers {

    }

    /*
    // Obtain minimizer for first w K-mers.
    for idx in 0..w {
        if kmers[idx].stupid_cmp(&minimizer) == Ordering::Less {
            minimizer = &kmers[idx];
        }
    }

    // Obtain minimizers for remaining windows excluding the last.
    for idx in w..kmers.len() - 1 {
        if kmers[idx].stupid_cmp(&minimizer) == Ordering::Less {
            result.push(SuperKmer{start_pos, length: idx - start_pos, minimizer: minimizer.clone()});
            start_pos = idx;
            minimizer = &kmers[idx];
        }
    }

    // Obtain minimizer for last window, pushing if it is different from the previous.
    let idx = kmers.len() - 1;
    if kmers[idx].stupid_cmp(&minimizer) == Ordering::Less {
        result.push(SuperKmer{start_pos, length: idx - start_pos, minimizer: minimizer.clone()});
        start_pos = idx;
        minimizer = &kmers[idx];
        result.push(SuperKmer{start_pos, length: idx - start_pos, minimizer: minimizer.clone()});
    }

    result
    */
}
