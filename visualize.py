#! /bin/python3

import matplotlib
import matplotlib.pyplot as plt
import argparse

#import scienceplots
#plt.style.use(['science','ieee'])

linestyle_tuple = [
        ('solid',                 (0, ())),
        #('loosely dotted',        (0, (1, 10))),
        ('dotted',                (0, (1, 1))),
        ('dashed',                (0, (5, 5))),
        ('dashdotted',            (0, (3, 5, 1, 5))),
        ('dashdotdotted',         (0, (3, 5, 1, 5, 1, 5))),
        #('densely dotted',        (0, (1, 1))),
        ('long dash with offset', (5, (10, 3))),
        ('loosely dashed',        (0, (5, 10))),
        ('loosely dashdotted',    (0, (3, 10, 1, 10))),
        ('loosely dashdotdotted', (0, (3, 10, 1, 10, 1, 10))),
        ('densely dashed',        (0, (5, 1))),
        ('densely dashdotted',    (0, (3, 1, 1, 1))),
        ('densely dashdotdotted', (0, (3, 1, 1, 1, 1, 1)))]

def create_cmap(keys):
    return dict(zip(
        sorted({key for key in keys}),
        matplotlib.colors.TABLEAU_COLORS.values()))

def create_lmap(keys):
    return dict(zip(
        sorted({key for key in keys}),
        [linestyle for (_, linestyle) in linestyle_tuple]))

parser = argparse.ArgumentParser()
parser.add_argument('figure_id', type=int)
args = parser.parse_args()

match args.figure_id:
    case 1:
        pass

from enum import Enum 
class Order(Enum):
    LEXICOGRAPHIC = "lexicographic"
    OCCURRENCE = "occurrence"
class Query(Enum):
    STANDARD = "standard"
    PWL = "pwl"

data = [
    {
        "k": 3,
        "w": 3,
        "order": Order.LEXICOGRAPHIC,
        "sequence": "Zika virus",
        "query": Query.STANDARD,
        "build_ms": 8.687049,
        "size_b": 1097839,
        "sequence_b": 10794,
        "fp": 4991042,
        "100k_queries_ms": 568.571406,
    },
    {
        "k": 3,
        "w": 3,
        "order": Order.OCCURRENCE,
        "sequence": "Zika virus",
        "query": Query.STANDARD,
        "build_ms": 12.384605,
        "size_b": 1131591,
        "sequence_b": 10794,
        "fp": 3070986,
        "100k_queries_ms": 667.840894,
    }
]
