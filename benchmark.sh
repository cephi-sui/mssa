#! /bin/bash

cargo build --release

index="benchmark.index"

target/release/mssa build 3 3 lexicographic test_input/dmel.fasta "$index" standard-query
target/release/mssa benchmark 10000 0.9 10 "$index" standard-query

target/release/mssa build 3 3 occurrence test_input/dmel.fasta "$index" standard-query
target/release/mssa benchmark 10000 0.9 10 "$index" standard-query

target/release/mssa build 3 10 lexicographic test_input/dmel.fasta "$index" pwl-learned-query -p 10
target/release/mssa benchmark 10000 0.9 20 "$index" pwl-learned-query

target/release/mssa build 3 10 occurrence test_input/dmel.fasta "$index" pwl-learned-query -p 10
target/release/mssa benchmark 10000 0.9 20 "$index" pwl-learned-query

rm -r "$index"
