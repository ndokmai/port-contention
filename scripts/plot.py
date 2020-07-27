import sys
import matplotlib.pyplot as plt
import seaborn as sns
import numpy as np
import statistics
from sklearn import mixture
import sys

def get_list(filename):
    f = open(filename, "r")
    x = np.fromiter(map(lambda x: int(x.strip()),  f.readlines()), float)
    return x

side_channel = get_list(sys.argv[1])

# GMM
part = mixture.GaussianMixture(n_components=2, covariance_type="spherical", max_iter=300, tol=1e-4)
part.fit(side_channel.reshape(-1, 1))
buckets = [(mean - 3*stdev, mean + 3*stdev, mean) for mean, stdev in zip(part.means_, np.sqrt(part.covariances_))]
buckets = sorted(buckets, key=lambda x: x[1]-x[0])

# segmentation
n_windows = 10000
window_size = int(len(side_channel)/n_windows)
segment_lines = []
prev_segment = None
for i in range(n_windows):
    window = side_channel[i*window_size: (i+1)*window_size]
    med = statistics.median(window)
    if med > buckets[0][0] and med < buckets[0][1]:
        if prev_segment is None:
            prev_segment = 0
        elif prev_segment == 1:
            segment_lines += [ i*window_size ]
            prev_segment = 0
    elif med > buckets[1][0] and med < buckets[1][1]:
        if prev_segment is None:
            prev_segment = 1
        elif prev_segment == 0:
            segment_lines += [ i*window_size ]
            prev_segment = 1

# choose segmentation
segment_lens = []
for (a, b) in zip(segment_lines[:-1], segment_lines[1:]):
    segment_lens += [b-a]
segment_lens = sorted(segment_lens)
segment_groups = []
while True:
    if len(segment_lens) == 0:
        break
    first = segment_lens.pop(0)
    new_group = [first]
    to_pop = []
    for index, item in enumerate(segment_lens):
        if abs(item-first) <= first/10:
            new_group += [item]
            to_pop += [index]
    segment_lens = [x for (i, x) in enumerate(segment_lens) if i not in to_pop]
    segment_groups += [new_group]
segment_groups = [x for x in segment_groups if len(x) > 1]
chosen_group = max(segment_groups, key=lambda x: statistics.mean(x))
chosen_group = set(chosen_group)

# filter segmentation
segments = []
for a, b in zip(segment_lines[:-1], segment_lines[1:]):
    if abs(b-a) in chosen_group:
        segments += [(a, b)]

# analyze
bottom = buckets[1][0]
top = buckets[1][1]
prev_mean = None
for (a, b) in segments:
    segment = side_channel[a:b]
    # current_mean = statistics.mean([x for x in segment if x > bottom and x < top])
    current_mean = statistics.mean(segment)
    print(current_mean)
    prev_mean = current_mean
exit(0)

# plot
colors = ["green", "magenta"]
for (lower, upper, mean), color in zip(buckets, colors):
    plt.axhline(y=mean, ls = "-", color = color)
    plt.axhline(y=lower, ls = "--", color = color)
    plt.axhline(y=upper, ls = "--", color = color)

for a, b in segments:
    plt.axvline(x=a)
    plt.axvline(x=b)

markersize=2
side_channel = side_channel[:segments[-1][1]]
plt.ylim((bottom, top))
plt.plot(side_channel, marker="o", markersize=markersize, color="blue", linestyle="None")

plt.locator_params(nbins=50)
plt.show()
