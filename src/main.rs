mod fasta;
mod int_vec;
mod iter_order_by;
mod suffix_array;
mod transform;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use suffix_array::{StandardQuery, SuffixArray};
use transform::{Alphabet, KmerSequence};

#[derive(Parser)]
enum Args {
    Build {
        fasta_file: PathBuf,
        output_file: PathBuf,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args {
        Args::Build {
            fasta_file,
            output_file,
        } => {
            // let sequences = read_sequences(fasta_file)?;
            // let k = 16;
            // let kmers = KmerSequence::from_bytes(&sequences[0].representation, k);
            // println!("kmers: {:?}", kmers);
            // let result = kmers.compute_super_kmers(3);

            let sequence = "ACTGACCCGTAGCGCTA".as_bytes();
            let k = 3;
            let w = 3;
            let alphabet = Alphabet::from_bytes(sequence);
            let kmers = KmerSequence::from_bytes(sequence, k, alphabet);
            println!("kmers: {:#?}", kmers);

            println!("----------");
            let suffix_array = SuffixArray::<StandardQuery>::from_kmers(kmers, w, ());
            println!("{:#?}", suffix_array);
            let query_result = suffix_array.query("CTGAC".as_bytes());
            println!("{:#?}", query_result);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let sequence = "ACTGACCCGTAGCGCTA".as_bytes();
        let k = 3;
        let w = 3;
        let alphabet = Alphabet::from_bytes(sequence);
        let kmers = KmerSequence::from_bytes(sequence, k, alphabet);
        println!("kmers: {:#?}", kmers);

        println!("----------");
        let suffix_array = SuffixArray::<StandardQuery>::from_kmers(kmers, w, ());
        println!("{:#?}", suffix_array);
        let query_result = suffix_array.query("CTGAC".as_bytes());
        println!("{:#?}", query_result);
    }
}
