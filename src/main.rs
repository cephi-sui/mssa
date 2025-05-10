mod fasta;
mod int_vec;
mod iter_order_by;
mod suffix_array;
mod transform;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use transform::KmerSequence;

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
            let kmers = KmerSequence::from_bytes(sequence, k);
            println!("kmers: {:#?}", kmers);
            let super_kmers = kmers.compute_super_kmers(w);
            println!("super_kmers: {:#?}", super_kmers);
        }
    }

    Ok(())
}
