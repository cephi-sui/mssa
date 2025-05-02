use anyhow::Result;
use std::fs;
use std::path::Path;

pub struct Sequence {
    description: String,
    representation: Vec<u8>,
}

impl Sequence {
    pub fn read_from_path<P: AsRef<Path>>(path: P) -> Result<Vec<Self>> {
        Ok(fs::read_to_string(path)?
            .split('>')
            .map(|sequence| {
                let split = sequence.split('\n');
                Self {
                    description: split.next(),
                    representation: split.collect::<String>().into_bytes(),
                }}).collect())
    }
}
