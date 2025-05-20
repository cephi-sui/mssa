use anyhow::Result;
use std::fs;
use std::path::Path;
use rand;

#[derive(Debug)]
pub struct Sequence {
    pub description: String,
    pub representation: Vec<u8>,
}

pub fn read_sequences<P: AsRef<Path>>(path: P) -> Result<Vec<Sequence>> {
    Ok(fs::read_to_string(path)?
        .split('>')
        .skip(1) // First split is an empty string.
        .filter_map(|sequence| {
            let mut split = sequence.lines();
            Some(Sequence {
                description: split.next()?.to_string(),
                representation: split.collect::<String>().into_bytes(),
            })
        })
        .collect())
}

pub fn generate_sequences(reference: &[u8], num: usize, match_rate: f64, min_len: usize, max_len: usize) -> Vec<Sequence> {
    //let mut rng = rang::rng();
    let mut result = Vec::new();
    for i in 0..num {
        let length = rand::random_range(min_len..=max_len);
        if rand::random_bool(match_rate) {
            let start_pos = rand::random_range(0..reference.len() - length);
            let r = &reference[start_pos..start_pos + length];

            result.push(Sequence {
                description: i.to_string(),
                representation: r.to_vec(),
            });
        } else {
            let representation = vec![0; length]
                .into_iter()
                .map(|_| reference[rand::random_range(0..reference.len())])
                .collect();

            result.push(Sequence {
                description: i.to_string(),
                representation,
            });
        }
    }
    result
}
