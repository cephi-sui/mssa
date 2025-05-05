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
pub fn construct_super_kmers(kmers: &[Kmer], representation: &[u8], k: usize) -> Vec<SuperKmer> {
    let mut index_queue: VecDeque<usize> = VecDeque::new();

    let mut minimums: Vec<Kmer> = Vec::new();

    for (i, kmer) in kmers.iter().enumerate() {
        while index_queue.len() > 0 {
            let popped = index_queue.pop_back().unwrap();
        }
    }

    let mut ret: Vec<SuperKmer> = Vec::new();

    // De-duplicate the minimums into super kmers
    let mut count = 1;
    let mut curr_kmer = &minimums[0];
    let mut curr_kmer_i = 0;
    for (i, minimum) in minimums.iter().enumerate().skip(1) {
        if minimum.cmp(&curr_kmer, representation) == Ordering::Equal {
            count += 1;
        } else {
            // retire the previous minimum
            ret.push(SuperKmer {
                start_pos: curr_kmer_i,
                length: count,
                minimizer: curr_kmer.clone(),
            });
            count = 1;
            curr_kmer = minimum;
            curr_kmer_i = i;
        }
    }
    if count != 0 {
        ret.push(SuperKmer {
            start_pos: curr_kmer_i,
            length: count,
            minimizer: curr_kmer.clone(),
        });
    }

    ret
}
