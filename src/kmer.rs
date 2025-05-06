use anyhow::Result;
use std::{
    cmp::{self, Ord, Ordering},
    collections::{vec_deque, VecDeque},
};

#[derive(Debug, Clone)]
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

trait StupidOrd {
    fn cmp(&self, other: &Self) -> Ordering;
}

impl<'a> StupidOrd for Kmer<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (&Kmer::Sentinel, &Kmer::Sentinel) => Ordering::Equal,
            (&Kmer::Data(d1), &Kmer::Data(d2)) => d1.cmp(d2),
            (&Kmer::Sentinel, &Kmer::Data(_)) => Ordering::Greater,
            (&Kmer::Data(_), &Kmer::Sentinel) => Ordering::Less,
        }
    }
}

impl<'a> StupidOrd for SuperKmer<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.minimizer.cmp(&other.minimizer)
    }
}

impl<'a> StupidOrd for &[Kmer<'a>] {
    // Ripped from core/slice/cmp.rs
    fn cmp(&self, other: &Self) -> Ordering {
        let l = cmp::min(self.len(), other.len());

        // Slice to the loop iteration range to enable bound check
        // elimination in the compiler
        let lhs = &self[..l];
        let rhs = &other[..l];

        for i in 0..l {
            match lhs[i].cmp(&rhs[i]) {
                Ordering::Equal => (),
                non_eq => return non_eq,
            }
        }

        self.len().cmp(&other.len())
    }
}

pub fn to_kmers<'a>(representation: &'a [u8], k: usize) -> Vec<Kmer<'a>> {
    representation.windows(k).map(|slice| Kmer::Data(slice)).collect()
}

// (Not currently) Based on DP solution at https://algo.monster/liteproblems/239
pub fn construct_super_kmers<'a>(kmers: &'a [Kmer], k: usize, w: usize) -> Vec<SuperKmer<'a>> {
    assert!(kmers.len() >= w);
    /*
    let x: Vec<_> = kmers
        .windows(k)
        .map(|window| window.iter().min().expect("k should not be 0"))
        .collect();
    */
    let mut result = Vec::new();
    let mut minimizer = (0, &kmers[0]);

    for idx in 0..w {
        if kmers[idx].cmp(&minimizer.1) == Ordering::Less {
            minimizer = (idx, &kmers[idx]);
        }
    }

    for idx in w..kmers.len() {
        if kmers[idx].cmp(&minimizer.1) == Ordering::Less {
            result.push(SuperKmer{start_pos: minimizer.0, length: idx - minimizer.0, minimizer: kmers[idx].clone()});
            minimizer = (idx, &kmers[idx]);
        }
    }

    result
}
