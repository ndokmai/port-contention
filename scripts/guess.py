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

def cluster(signal, reference):
    part = mixture.BayesianGaussianMixture(n_components=2, max_iter=300)
    part.fit(signal.reshape(-1, 1))
    target = sorted(part.means_)
    answers = []
    for i in means:
        if abs(i-target[0]) < abs(i-target[1]):
            answers += [0]
        else:
            answers += [1]
    print(part.means_)
    print("reference =\t", reference)
    print("answer =\t", answers)
    correctness = 0
    for (a, b) in zip(answers, reference):
        if a == b:
            correctness += 1
    print("correctness =", correctness/len(answers)*100)

cluster(means, reference)
# for signal, reference in zip(np.array_split(means, 10), np.array_split(np.array(reference), 10)):
    # cluster(signal, reference)



