use anyhow::Result;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Sequence {
    pub description: String,
    pub representation: Vec<u8>,
}

impl Sequence {
    pub fn read_from_path<P: AsRef<Path>>(path: P) -> Result<Vec<Self>> {
        Ok(fs::read_to_string(path)?
            .split('>')
            .skip(1) // First split is an empty string.
            .filter_map(|sequence| {
                let mut split = sequence.split('\n');
                Some(Self {
                    description: split.next()?.to_string(),
                    representation: split.collect::<String>().into_bytes(),
                })
            })
            .collect())
    }
}
