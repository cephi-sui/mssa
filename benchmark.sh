#! /bin/bash

cargo build --release

function get_index_size {
    du -b "$index" | awk '{printf "Suffix Array Size: "; printf $1; print " bytes"}'
}

index="benchmark.index"

inputs=(test_input/dmel.fasta \
    test_input/ENA_bacteria/SAMD00000344.contigs.fa)
for input in ${inputs[@]}; do
    command=(target/release/mssa build 3 3 lexicographic "$input" "$index" standard-query)
    echo "${command[@]}"
    "${command[@]}"
    get_index_size
    target/release/mssa benchmark 10000 0.9 10 "$index" standard-query
    rm -r "$index"

    command=(target/release/mssa build 3 3 occurrence "$input" "$index" standard-query)
    echo "${command[@]}"
    "${command[@]}"
    get_index_size
    target/release/mssa benchmark 10000 0.9 10 "$index" standard-query
    rm -r "$index"
done

#target/release/mssa build 3 10 lexicographic test_input/dmel.fasta "$index" pwl-learned-query -p 10
#get_index_size
#target/release/mssa benchmark 10000 0.9 20 "$index" pwl-learned-query
#
#target/release/mssa build 3 10 occurrence test_input/dmel.fasta "$index" pwl-learned-query -p 10
#get_index_size
#target/release/mssa benchmark 10000 0.9 20 "$index" pwl-learned-query

