use anyhow::Result;
use std::{
    cmp::{Ord, Ordering},
    collections::{VecDeque, vec_deque},
};

#[derive(Copy, Clone, Debug)]
pub enum Kmer {
    Data { start_pos: usize, length: usize },
    Sentinel,
}

#[derive(Debug)]
pub struct SuperKmer {
    pub start_pos: usize,
    pub length: usize,
    pub minimizer: Kmer,
}

impl Kmer {
    pub fn cmp(&self, other: &Self, representation: &[u8]) -> Ordering {
        todo!()
    }
}

pub fn to_kmers(representation: &[u8], k: usize) -> Vec<Kmer> {
    let mut kmers: Vec<_> = (0..representation.len())
        .collect::<Vec<_>>()
        .windows(k)
        .map(|slice| Kmer::Data {
            start_pos: slice[0],
            length: slice.len(),
        })
        .collect();

    // Ensure that we always have the sentinel character
    kmers.push(Kmer::Sentinel);

    kmers
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
