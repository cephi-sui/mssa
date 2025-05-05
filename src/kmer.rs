use anyhow::Result;
use std::{
    cmp::{Ord, Ordering},
    collections::{VecDeque, vec_deque},
};

#[derive(Debug, Eq)]
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

impl<'a, 'b> PartialEq<Kmer<'b>> for Kmer<'a> {
    fn eq(&self, other: &Kmer<'b>) -> bool {
        match (self, other) {
            (&Kmer::Sentinel, &Kmer::Sentinel) => true,
            (&Kmer::Data(d1), &Kmer::Data(d2)) => d1 == d2,
            _ => false,
        }
    }
}

impl<'a, 'b> PartialOrd<Kmer<'b>> for Kmer<'a> {
    fn partial_cmp(&self, other: &Kmer<'b>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Kmer<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (&Kmer::Sentinel, &Kmer::Sentinel) => Ordering::Equal,
            (&Kmer::Data(d1), &Kmer::Data(d2)) => d1.cmp(d2),
            (&Kmer::Sentinel, &Kmer::Data(_)) => Ordering::Greater,
            (&Kmer::Data(_), &Kmer::Sentinel) => Ordering::Less,
        }
    }
}

pub fn to_kmers<'a>(representation: &'a [u8], k: usize) -> Vec<Kmer<'a>> {
    representation.windows(k).map(|slice| Kmer::Data(slice)).collect()
}

// Based on DP solution at https://algo.monster/liteproblems/239
pub fn construct_super_kmers<'a>(kmers: &'a [Kmer], k: usize) -> Vec<SuperKmer<'a>> {
    let x: Vec<_> = kmers
        .windows(k)
        .map(|window| window.iter().min().expect("k should not be 0"))
        .collect();

    todo!()
}
