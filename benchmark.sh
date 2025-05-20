#! /bin/bash

cargo build --release

function print_and_exec {
    echo "${command[@]}"
    "${command[@]}"
}

function get_index_size {
    du -b "$index" | awk '{printf "Suffix Array Size: "; printf $1; print " bytes"}'
}

index="benchmark.index"

inputs=( \
    test_input/Zika_virus.fasta \
    test_input/Monkeypox_virus.fasta \
    test_input/Drosophila_melanogaster_chromosome_Y.fasta \
    test_input/Mycobacterium_tuberculosis_H37Rv.fasta \
)
for input in ${inputs[@]}; do
    command=(target/release/mssa build 3 3 lexicographic "$input" "$index" standard-query)
    print_and_exec
    get_index_size
    command=(target/release/mssa benchmark 100000 0.9 10 "$index" standard-query)
    print_and_exec
    rm -r "$index"

    echo ""

    command=(target/release/mssa build 3 3 occurrence "$input" "$index" standard-query)
    print_and_exec
    get_index_size
    command=(target/release/mssa benchmark 100000 0.9 10 "$index" standard-query)
    print_and_exec
    rm -r "$index"

    echo ""
done

#target/release/mssa build 3 10 lexicographic test_input/dmel.fasta "$index" pwl-learned-query -p 10
#get_index_size
#target/release/mssa benchmark 10000 0.9 20 "$index" pwl-learned-query
#
#target/release/mssa build 3 10 occurrence test_input/dmel.fasta "$index" pwl-learned-query -p 10
#get_index_size
#target/release/mssa benchmark 10000 0.9 20 "$index" pwl-learned-query

