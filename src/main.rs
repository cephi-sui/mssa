use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

mod fasta;
use fasta::Sequence;

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
            let sequences = Sequence::read_from_path(fasta_file)?;
        }
    }

    Ok(())
}
