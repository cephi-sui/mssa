mod fasta;
mod int_vec;
mod transform;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use fasta::read_sequences;
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
            let sequences = read_sequences(fasta_file)?;
            let k = 16;
            let kmers = KmerSequence::from_bytes(&sequences[0].representation, k);
            println!("kmers: {:?}", kmers)
        }
    }

    Ok(())
}
