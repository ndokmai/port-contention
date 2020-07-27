import numpy as np
from sklearn import mixture
import ast
import sys

def get_list(filename):
    f = open(filename, "r")
    x = np.fromiter(map(lambda x: float(x.strip()),  f.readlines()), float)
    return x

guess = get_list(sys.argv[1])
reference = ast.literal_eval(sys.argv[2])
part = mixture.BayesianGaussianMixture(n_components=2, max_iter=400)
part.fit(guess.reshape(-1, 1))
target = sorted(part.means_)

answers = []
for i in guess:
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

