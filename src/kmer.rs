use anyhow::Result;
use std::cmp::{Ord, Ordering};

#[derive(Debug, Eq)]
pub struct Kmer<'a>(&'a [u8]);

impl<'a, 'b> PartialEq<Kmer<'b>> for Kmer<'a> {
    fn eq(&self, other: &Kmer<'b>) -> bool {
        self.0.eq(other.0)
    }
}

impl<'a, 'b> PartialOrd<Kmer<'b>> for Kmer<'a> {
    fn partial_cmp(&self, other: &Kmer<'b>) -> Option<Ordering> {
        self.0.partial_cmp(other.0)
    }
}

impl<'a> Ord for Kmer<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
