#! /bin/python3

import re
import json

with open('output.txt', 'r') as file:
    results = list()
    result = dict()
    for line in file:
        r = re.search(r'build (\d+) (\d+) (\S+) (\S+) \S+ (\S+)', line)
        if r is not None:
            results.append(result)
            result = dict()
            result |= {"k": int(r[1]),
                       "w": int(r[2]),
                       "ordering": r[3],
                       "input": r[4],
                       "query": r[5]}
        r = re.search(r'Index build time \(ms\): (\d+\.\d+)', line)
        if r is not None:
            result |= {"build_ms": float(r[1])}
        r = re.search(r'Suffix Array Size: (\d+) bytes', line)
        if r is not None:
            result |= {"size_b": int(r[1])}
        r = re.search(r'Original string length: (\d+) bytes', line)
        if r is not None:
            result |= {"sequence_b": int(r[1])}
        r = re.search(r'False positives: (\d+)', line)
        if r is not None:
            result |= {"fp": int(r[1])}
        r = re.search(r'Total time \(ms\) for performing 100000 queries: (\d+\.\d+)', line)
        if r is not None:
            result |= {"100k_queries_ms": float(r[1])}
    results.append(result)

    #print(results)
    print(json.dumps(results[1:]))
