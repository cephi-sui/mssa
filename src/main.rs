use std::path::PathBuf;

use clap::Parser;
mod fasta;
use fasta::Sequence;

#[derive(Parser)]
struct Args {
    fasta_file: PathBuf,
}

fn main() {
    let args = Args::parse();
    let sequences = Sequence::read_from_path(args.fasta_file);
}
