from common import get_list_from_file, segmentation
import numpy as np
from sklearn import mixture
from sklearn import cluster
import statistics
import ast
import sys

side_channel = get_list_from_file(sys.argv[1])
reference = ast.literal_eval(sys.argv[2])

segments, _ = segmentation(side_channel)

means = []
for (a, b) in segments:
    segment = side_channel[a:b]
    means += [statistics.mean(segment)]
means = np.array(means)

def cluster(signal, ref):
    part = mixture.BayesianGaussianMixture(n_components=2, max_iter=300)
    part.fit(signal.reshape(-1, 1))
    target = sorted(part.means_)
    answers = []
    for i in signal:
        if abs(i-target[0]) < abs(i-target[1]):
            answers += [0]
        else:
            answers += [1]
    print("zero center = \t", part.means_[0])
    print("one center = \t", part.means_[1])
    print("reference =\t", ref)
    print("answer =\t", np.array(answers))
    correctness = 0
    for (a, b) in zip(answers, ref):
        if a == b:
            correctness += 1
    print("correctness =", correctness/len(answers)*100)

n_chunks = int((len(reference) + 19)/20)
# cluster(means, reference)
for signal, reference in zip(np.array_split(means, n_chunks), np.array_split(np.array(reference), n_chunks)):
    cluster(signal, reference)



