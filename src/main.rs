use clap::Parser; 
mod fasta;
use fasta::Sequence;

#[derive(Parser)]
struct Args {
    fasta_file: String,
}
fn main() {
    let args = Args::parse();
    Sequence::read_from_path(args.fasta_file);
}
