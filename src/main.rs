use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

mod fasta;
use fasta::read_sequences;

mod kmer;

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
        }
    }

    Ok(())
}
