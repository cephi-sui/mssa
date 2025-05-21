#! /bin/python3

import matplotlib
import matplotlib.pyplot as plt
import argparse
import json
import subprocess

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

with open('output.json', 'r') as file:
    samples = json.load(file)
print(samples[0].keys())

match args.figure_id:
    case 1:
        y_axis = "100k_queries_ms"
        x_axis = "sequence_b"
        group_by = ['query', 'ordering']

        samples = [sample for sample in samples if y_axis in sample]
        samples = [sample for sample in samples if x_axis in sample]
        samples.sort(key=lambda sample : sample[x_axis])
        for g_name in reversed(group_by):
            samples.sort(key=lambda sample : sample[g_name])

        print(samples)

        mapping = {'color': create_cmap([sample[group_by[0]] for sample in samples]),
                   'linestyle': create_lmap([sample[group_by[1]] for sample in samples])}

        fig, ax = plt.subplots(1, 1)
        #fig, ax = plt.subplots(3, 1)
        #for i, input_file in enumerate(sorted(list(set([sample['input'] for sample in samples])))):
        #    subsamples = [sample for sample in samples if sample['input'] == input_file]
        #    print(subsamples)
        for groups in list(
                dict.fromkeys(
                    [tuple([sample[g_name] for g_name in group_by]) for sample in samples])):
            ax.plot(
                    [sample[x_axis] for sample in samples
                     if set([sample[g_name] for g_name in group_by]).issubset(groups)],
                    [sample[y_axis] for sample in samples
                     if set([sample[g_name] for g_name in group_by]).issubset(groups)],
                    label=", ".join([str(group) for group in groups]),
                    color=mapping['color'][groups[0]],
                    linestyle=mapping['linestyle'][groups[1]])

        handles, labels = ax.get_legend_handles_labels()
        ax.legend(
                handles,
                labels,
                title="Query Type, Ordering",
                #ncols=4,
                #loc='upper center',
                #bbox_to_anchor=(0.5, 1.25),
                #fontsize=6)
                )
        ax.set_ylabel("Execution Time for 100k Queries (ms)")
        ax.set_xlabel("Sequence Length (characters)")

        plt.title("Execution Time for 100k Queries vs. Sequence Length")
        title = "figs/time_vs_len.png"
        plt.savefig(title)
        subprocess.run(['xdg-open', title])
    case 2:
        y_axis = "fp"
        x_axis = "sequence_b"
        group_by = ['ordering']

        samples = [sample for sample in samples if sample['query'] != "pwl-learned-query"]
        samples = [sample for sample in samples if y_axis in sample]
        samples = [sample for sample in samples if x_axis in sample]
        samples.sort(key=lambda sample : sample[x_axis])
        for g_name in reversed(group_by):
            samples.sort(key=lambda sample : sample[g_name])

        print(samples)

        mapping = {'color': create_cmap([sample[group_by[0]] for sample in samples])}
                   #'linestyle': create_lmap([sample[group_by[1]] for sample in samples])}

        fig, ax = plt.subplots(1, 1)
        for groups in list(
                dict.fromkeys(
                    [tuple([sample[g_name] for g_name in group_by]) for sample in samples])):
            ax.plot(
                    [sample[x_axis] for sample in samples
                     if set([sample[g_name] for g_name in group_by]).issubset(groups)],
                    [sample[y_axis] for sample in samples
                     if set([sample[g_name] for g_name in group_by]).issubset(groups)],
                    label=", ".join([str(group) for group in groups]),
                    color=mapping['color'][groups[0]])
                    #linestyle=mapping['linestyle'][groups[1]])

        #ax.set_xscale('log')
        ax.set_ylabel("False Positives")
        ax.set_xlabel("Sequence Length (characters)")
        handles, labels = ax.get_legend_handles_labels()
        ax.legend(
                handles,
                labels,
                title="Ordering",
                )

        plt.title("False Positives vs. Sequence Length")
        title = "figs/fp_vs_len.png"
        plt.savefig(title)
        subprocess.run(['xdg-open', title])
    case 3:
        y_axis = "build_ms"
        x_axis = "sequence_b"
        group_by = ['query', 'ordering']

        samples = [sample for sample in samples if y_axis in sample]
        samples = [sample for sample in samples if x_axis in sample]
        samples.sort(key=lambda sample : sample[x_axis])
        for g_name in reversed(group_by):
            samples.sort(key=lambda sample : sample[g_name])

        print(samples)

        mapping = {'color': create_cmap([sample[group_by[0]] for sample in samples]),
                   'linestyle': create_lmap([sample[group_by[1]] for sample in samples])}

        fig, ax = plt.subplots(1, 1)
        for groups in list(
                dict.fromkeys(
                    [tuple([sample[g_name] for g_name in group_by]) for sample in samples])):
            ax.plot(
                    [sample[x_axis] for sample in samples
                     if set([sample[g_name] for g_name in group_by]).issubset(groups)],
                    [sample[y_axis] for sample in samples
                     if set([sample[g_name] for g_name in group_by]).issubset(groups)],
                    label=", ".join([str(group) for group in groups]),
                    color=mapping['color'][groups[0]],
                    linestyle=mapping['linestyle'][groups[1]])

        handles, labels = ax.get_legend_handles_labels()
        ax.legend(
                handles,
                labels,
                title="Query Type, Ordering")
        ax.set_ylabel("Build Time (ms)")
        ax.set_xlabel("Sequence Length (characters)")

        plt.title("Build Time vs. Sequence Length")
        title = "figs/build_vs_len.png"
        plt.savefig(title)
        subprocess.run(['xdg-open', title])
    case 4:
        y_axis = "size_b"
        x_axis = "sequence_b"
        group_by = ['query', 'ordering']

        samples = [sample for sample in samples if y_axis in sample]
        samples = [sample for sample in samples if x_axis in sample]
        samples.sort(key=lambda sample : sample[x_axis])
        for g_name in reversed(group_by):
            samples.sort(key=lambda sample : sample[g_name])

        print(samples)

        mapping = {'color': create_cmap([sample[group_by[0]] for sample in samples]),
                   'linestyle': create_lmap([sample[group_by[1]] for sample in samples])}

        fig, ax = plt.subplots(1, 1)
        for groups in list(
                dict.fromkeys(
                    [tuple([sample[g_name] for g_name in group_by]) for sample in samples])):
            ax.plot(
                    [sample[x_axis] for sample in samples
                     if set([sample[g_name] for g_name in group_by]).issubset(groups)],
                    [sample[y_axis] / 1000000 for sample in samples
                     if set([sample[g_name] for g_name in group_by]).issubset(groups)],
                    label=", ".join([str(group) for group in groups]),
                    color=mapping['color'][groups[0]],
                    linestyle=mapping['linestyle'][groups[1]])

        handles, labels = ax.get_legend_handles_labels()
        ax.legend(
                handles,
                labels,
                title="Query Type, Ordering")
        ax.set_ylabel("Suffix Array Size (MB)")
        ax.set_xlabel("Sequence Length (characters)")

        plt.title("Suffix Array Size vs. Sequence Length")
        title = "figs/size_vs_len.png"
        plt.savefig(title)
        subprocess.run(['xdg-open', title])
