use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

mod fasta;
use fasta::Sequence;

mod kmer;

#[derive(Parser)]
struct Args {
    fasta_file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let sequences = Sequence::read_from_path(args.fasta_file)?;

    Ok(())
}
