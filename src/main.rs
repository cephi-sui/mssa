mod fasta;
mod int_vec;
mod iter_order_by;
mod suffix_array;
mod transform;

use std::{fs::File, path::PathBuf};

use anyhow::{anyhow, Context, Result};
use bincode;
use clap::{Parser, ValueEnum};

use fasta::Sequence;
use suffix_array::{
    BloomFilterQuery, GroundTruthQuery, PWLLearnedQuery, Queryable, StandardQuery,
    SuffixArray,
};
use transform::{Alphabet, KmerSequence, MinimizerOrder};

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
        minimizer_order: MinimizerOrder,
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
    Benchmark {
        num_queries: usize,
        match_rate: f64,
        max_query_length: usize,
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
            minimizer_order,
            fasta_file,
            suffix_array_file,
            query_type,
            bloom_filter_fpr,
            piecewise_linear_gamma,
        } => {
            let sequences = fasta::read_sequences(fasta_file)?;
            if sequences.len() > 1 {
                return Err(anyhow!("Expected only one sequence in FASTA file"));
            }
            let sequence = sequences.into_iter().take(1).next().context("Expected one sequence in FASTA file")?;
            let suffix_array_file = &mut File::create(suffix_array_file)?;
            match query_type {
                QueryType::GroundTruthQuery => {
                    let mut suffix_arrays = Vec::new();
                    let alphabet = Alphabet::from_bytes(&sequence.representation);
                    let kmers = KmerSequence::from_bytes(&sequence.representation, k, alphabet);

                    let suffix_array =
                        SuffixArray::<GroundTruthQuery>::from_kmers(kmers, w, minimizer_order, ());

                    suffix_arrays.push(suffix_array);

                    bincode::encode_into_std_write(
                        suffix_arrays,
                        suffix_array_file,
                        bincode_config,
                    )?;
                }
                QueryType::StandardQuery => {
                    let mut suffix_arrays = Vec::new();
                    let alphabet = Alphabet::from_bytes(&sequence.representation);
                    let kmers = KmerSequence::from_bytes(&sequence.representation, k, alphabet);

                    let suffix_array = SuffixArray::<StandardQuery>::from_kmers(kmers, w, minimizer_order, ());

                    suffix_arrays.push(suffix_array);

                    bincode::encode_into_std_write(
                        suffix_arrays,
                        suffix_array_file,
                        bincode_config,
                    )?;
                }
                QueryType::BloomFilterQuery => {
                    let mut suffix_arrays = Vec::new();
                    let alphabet = Alphabet::from_bytes(&sequence.representation);
                    let kmers = KmerSequence::from_bytes(&sequence.representation, k, alphabet);

                    let suffix_array = SuffixArray::<BloomFilterQuery>::from_kmers(
                        kmers,
                        w,
                        minimizer_order,
                        bloom_filter_fpr
                            .context("Expected Bloom filter false positive rate")?,
                    );

                    suffix_arrays.push(suffix_array);

                    bincode::encode_into_std_write(
                        suffix_arrays,
                        suffix_array_file,
                        bincode_config,
                    )?;
                }
                QueryType::PWLLearnedQuery => {
                    let mut suffix_arrays = Vec::new();
                        let alphabet = Alphabet::from_bytes(&sequence.representation);
                        let kmers = KmerSequence::from_bytes(&sequence.representation, k, alphabet);

                        let suffix_array = SuffixArray::<PWLLearnedQuery>::from_kmers(
                            kmers,
                            w,
                            minimizer_order,
                            piecewise_linear_gamma
                                .context("Expected piecewise linear regression gamma factor")?,
                        );

                        suffix_arrays.push(suffix_array);

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
                    println!("{:?}", result.0);
                    println!("False positive rate: {:?}", result.1);
                }
                QueryType::StandardQuery => {
                    let suffix_arrays: Vec<SuffixArray<StandardQuery>> =
                        bincode::decode_from_std_read(suffix_array_file, bincode_config)
                            .with_context(|| {
                                format!("Could not decode index with query type {:?}", query_type)
                            })?;

                    let result = query(suffix_arrays, sequences);
                    println!("{:?}", result.0);
                    println!("False positive rate: {:?}", result.1);
                }
                QueryType::BloomFilterQuery => {
                    todo!();
                }
                QueryType::PWLLearnedQuery => {
                    let suffix_arrays: Vec<SuffixArray<PWLLearnedQuery>> =
                        bincode::decode_from_std_read(suffix_array_file, bincode_config)
                            .with_context(|| {
                                format!("Could not decode index with query type {:?}", query_type)
                            })?;

                    let result = query(suffix_arrays, sequences);
                    println!("{:?}", result.0);
                    println!("False positive rate: {:?}", result.1);
                },
            }
        },
        Args::Benchmark {
            num_queries,
            match_rate,
            max_query_length,
            suffix_array_file,
            query_type,
        } => {
            let suffix_array_file = &mut File::open(suffix_array_file)?;

            match query_type {
                QueryType::GroundTruthQuery => {
                    let suffix_arrays: Vec<SuffixArray<GroundTruthQuery>> =
                        bincode::decode_from_std_read(suffix_array_file, bincode_config)
                            .with_context(|| {
                                format!("Could not decode index with query type {:?}", query_type)
                            })?;
                    let min_len = suffix_arrays[0].w() + suffix_arrays[0].get_underlying_kmers().k() - 1;
                    if min_len > max_query_length {
                        return Err(anyhow!("Max query length was not large enough! The minimum query length is {}", min_len));
                    }

                    let sequences = fasta::generate_sequences(suffix_arrays[0].get_underlying_kmers().get_original_string(), num_queries, match_rate, min_len, max_query_length);

                    println!("{:?}", sequences);

                    let result = query(suffix_arrays, sequences);
                    println!("{:?}", result.0);
                    println!("False positive rate: {:?}", result.1);
                },
                QueryType::StandardQuery => {
                    let suffix_arrays: Vec<SuffixArray<StandardQuery>> =
                        bincode::decode_from_std_read(suffix_array_file, bincode_config)
                            .with_context(|| {
                                format!("Could not decode index with query type {:?}", query_type)
                            })?;
                    let min_len = suffix_arrays[0].w() + suffix_arrays[0].get_underlying_kmers().k() - 1;
                    if min_len > max_query_length {
                        return Err(anyhow!("Max query length was not large enough! The minimum query length is {}", min_len));
                    }

                    let sequences = fasta::generate_sequences(suffix_arrays[0].get_underlying_kmers().get_original_string(), num_queries, match_rate, min_len, max_query_length);

                    //println!("{:?}", sequences);

                    let result = query(suffix_arrays, sequences);
                    //println!("{:?}", result.0);
                    println!("False positive rate: {:?}", result.1);
                },
                QueryType::BloomFilterQuery => {
                    todo!();
                },
                QueryType::PWLLearnedQuery => {
                    let suffix_arrays: Vec<SuffixArray<PWLLearnedQuery>> =
                        bincode::decode_from_std_read(suffix_array_file, bincode_config)
                            .with_context(|| {
                                format!("Could not decode index with query type {:?}", query_type)
                            })?;
                    let min_len = suffix_arrays[0].w() + suffix_arrays[0].get_underlying_kmers().k() - 1;
                    if min_len > max_query_length {
                        return Err(anyhow!("Max query length was not large enough! The minimum query length is {}", min_len));
                    }

                    let sequences = fasta::generate_sequences(suffix_arrays[0].get_underlying_kmers().get_original_string(), num_queries, match_rate, min_len, max_query_length);

                    //println!("{:?}", sequences);

                    let result = query(suffix_arrays, sequences);
                    //println!("{:?}", result.0);
                    println!("False positive rate: {:?}", result.1);
                },
            }
        },
    }

    Ok(())
}

fn query<T: Queryable>(suffix_arrays: Vec<T>, queries: Vec<Sequence>) -> (Vec<(String, Option<usize>)>, f32) {
    let mut result = Vec::new();
    let mut false_positives = 0;
    //for (i, suffix_array) in suffix_arrays.into_iter().enumerate() {
    for suffix_array in suffix_arrays.into_iter() {
        for query in &queries {
            let query_result = suffix_array.query(&query.representation);
            //result.push((i, query_result.0));
            result.push((query.description.clone(), query_result.0));
            false_positives += if query_result.1 { 1 } else { 0 };
        }
    }
    (result, false_positives as f32 / queries.len() as f32)
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
        let suffix_array = SuffixArray::<StandardQuery>::from_kmers(kmers, w, MinimizerOrder::Lexicographic, ());
        println!("{:#?}", suffix_array);
        let query_result = suffix_array.query("CTGAC".as_bytes());
        println!("{:#?}", query_result);
    }
}
