use anyhow::Result;
use std::{
    cmp::{Ord, Ordering},
    collections::{VecDeque, vec_deque},
};

#[derive(Debug)]
pub enum Kmer {
    Data { start_pos: usize, length: usize },
    Sentinel,
}

pub struct SuperKmer {
    start_pos: usize,
    length: usize,
    minimizer: Vec<u8>,
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
pub fn construct_super_kmers(kmers: &[Kmer], k: usize) -> Vec<SuperKmer> {
    todo!();
    let mut index_queue: VecDeque<usize> = VecDeque::new();

    let ret: Vec<SuperKmer> = Vec::new();

    for (i, kmer) in kmers.iter().enumerate() {
        if index_queue.get(0) < Some(i - k + 1).as_ref() {
            index_queue.pop_front();
        }

        while index_queue.len() > 0 {
            // if kmers[*index_queue.iter().last().unwrap()] {
        }
    }

    ret
}
