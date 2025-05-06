use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

mod fasta;
use fasta::read_sequences;
use kmer::{construct_super_kmers, to_kmers};

mod kmer;

mod sa;

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
            println!("{:?}", sequences[0].representation.len());
            let kmers = to_kmers(&sequences[0].representation, 10);
            let s_kmers = construct_super_kmers(&kmers, 10);
            println!("{:?}", s_kmers);
        }
    }

    Ok(())
}
