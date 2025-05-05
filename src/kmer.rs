use anyhow::Result;
use std::cmp::{Ord, Ordering};

#[derive(PartialEq, Eq, Debug)]
pub struct Kmer<'a>(&'a [u8]);

impl<'a, 'b> PartialOrd<Kmer<'b>> for Kmer<'a> {
    fn partial_cmp(&'a self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(other.0)
    }
}

/*
impl Ord for Kmer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
*/
