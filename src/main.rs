mod fasta;
mod int_vec;
mod iter_order_by;
mod suffix_array;
mod transform;

use std::{fs::File, path::PathBuf};

use anyhow::{Context, Result};
use bincode;
use clap::{Parser, ValueEnum};

use fasta::Sequence;
use suffix_array::{
    BloomFilterQuery, GroundTruthQuery, PWLLearnedQuery, QueryMode, Queryable, StandardQuery,
    SuffixArray,
};
use transform::{Alphabet, KmerSequence};

#[derive(Debug, Clone, ValueEnum)]
enum QueryType {
    GroundTruthQuery,
    StandardQuery,
    BloomFilterQuery,
    PWLLearnedQuery,
}

#[derive(Parser)]
enum Args {
    Build {
        k: usize,
        w: usize,
        fasta_file: PathBuf,
        suffix_array_file: PathBuf,
        query_type: QueryType,

        // BloomFilterQuery options
        #[arg(short, long)]
        bloom_filter_fpr: Option<f32>,

        // PWLLearnedQuery options
        #[arg(short, long)]
        piecewise_linear_gamma: Option<f64>,
    },
    Query {
        fasta_file: PathBuf,
        suffix_array_file: PathBuf,
        query_type: QueryType,
    },
}

fn main() -> Result<()> {
    let bincode_config = bincode::config::standard()
        .with_little_endian()
        .with_fixed_int_encoding();

    let args = Args::parse();
    match args {
        Args::Build {
            k,
            w,
            fasta_file,
            suffix_array_file,
            query_type,
            bloom_filter_fpr,
            piecewise_linear_gamma,
        } => {
            let sequences = fasta::read_sequences(fasta_file)?;
            let suffix_array_file = &mut File::create(suffix_array_file)?;
            match query_type {
                QueryType::GroundTruthQuery => {
                    let mut suffix_arrays = Vec::new();
                    for sequence in sequences {
                        let alphabet = Alphabet::from_bytes(&sequence.representation);
                        let kmers = KmerSequence::from_bytes(&sequence.representation, k, alphabet);

                        let suffix_array =
                            SuffixArray::<GroundTruthQuery>::from_kmers(kmers, w, ());

                        suffix_arrays.push(suffix_array);
                    }

                    bincode::encode_into_std_write(
                        suffix_arrays,
                        suffix_array_file,
                        bincode_config,
                    )?;
                }
                QueryType::StandardQuery => {
                    let mut suffix_arrays = Vec::new();
                    for sequence in sequences {
                        let alphabet = Alphabet::from_bytes(&sequence.representation);
                        let kmers = KmerSequence::from_bytes(&sequence.representation, k, alphabet);

                        let suffix_array = SuffixArray::<StandardQuery>::from_kmers(kmers, w, ());

                        suffix_arrays.push(suffix_array);
                    }

                    bincode::encode_into_std_write(
                        suffix_arrays,
                        suffix_array_file,
                        bincode_config,
                    )?;
                }
                QueryType::BloomFilterQuery => {
                    let mut suffix_arrays = Vec::new();
                    for sequence in sequences {
                        let alphabet = Alphabet::from_bytes(&sequence.representation);
                        let kmers = KmerSequence::from_bytes(&sequence.representation, k, alphabet);

                        let suffix_array = SuffixArray::<BloomFilterQuery>::from_kmers(
                            kmers,
                            w,
                            bloom_filter_fpr
                                .context("Expected Bloom filter false positive rate")?,
                        );

                        suffix_arrays.push(suffix_array);
                    }

                    bincode::encode_into_std_write(
                        suffix_arrays,
                        suffix_array_file,
                        bincode_config,
                    )?;
                }
                QueryType::PWLLearnedQuery => {
                    let mut suffix_arrays = Vec::new();
                    for sequence in sequences {
                        let alphabet = Alphabet::from_bytes(&sequence.representation);
                        let kmers = KmerSequence::from_bytes(&sequence.representation, k, alphabet);

                        let suffix_array = SuffixArray::<PWLLearnedQuery>::from_kmers(
                            kmers,
                            w,
                            piecewise_linear_gamma
                                .context("Expected piecewise linear regression gamma factor")?,
                        );

                        suffix_arrays.push(suffix_array);
                    }

                    bincode::encode_into_std_write(
                        suffix_arrays,
                        suffix_array_file,
                        bincode_config,
                    )?;
                }
            }
        }
        Args::Query {
            fasta_file,
            suffix_array_file,
            query_type,
        } => {
            let sequences = fasta::read_sequences(fasta_file)?;
            let suffix_array_file = &mut File::open(suffix_array_file)?;
            match query_type {
                QueryType::GroundTruthQuery => {
                    let suffix_arrays: Vec<SuffixArray<GroundTruthQuery>> =
                        bincode::decode_from_std_read(suffix_array_file, bincode_config)
                            .with_context(|| {
                                format!("Could not decode index with query type {:?}", query_type)
                            })?;

                    let result = query(suffix_arrays, sequences);
                    println!("{:?}", result);
                }
                QueryType::StandardQuery => {
                    let suffix_arrays: Vec<SuffixArray<StandardQuery>> =
                        bincode::decode_from_std_read(suffix_array_file, bincode_config)
                            .with_context(|| {
                                format!("Could not decode index with query type {:?}", query_type)
                            })?;

                    let result = query(suffix_arrays, sequences);
                    println!("{:?}", result);
                }
                QueryType::BloomFilterQuery => {
                    let suffix_arrays: Vec<SuffixArray<BloomFilterQuery>> =
                        bincode::decode_from_std_read(suffix_array_file, bincode_config)
                            .with_context(|| {
                                format!("Could not decode index with query type {:?}", query_type)
                            })?;
                    todo!();

                    //let result = query(suffix_arrays, sequences);
                    //println!("{:?}", result);
                }
                QueryType::PWLLearnedQuery => todo!(),
            }
        }
    }

    Ok(())
}

fn query<T>(suffix_arrays: Vec<T>, queries: Vec<Sequence>) -> Vec<(usize, Option<usize>)>
where
    T: Queryable,
{
    let mut result = Vec::new();
    for (i, suffix_array) in suffix_arrays.into_iter().enumerate() {
        for query in &queries {
            result.push((i, suffix_array.query(&query.representation)));
        }
    }
    result
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
