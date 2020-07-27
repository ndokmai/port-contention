import numpy as np
from sklearn import mixture
import statistics

def get_list_from_file(filename):
    f = open(filename, "r")
    x = np.fromiter(map(lambda x: float(x.strip()),  f.readlines()), float)
    return x

def segmentation(side_channel, n_search_windows=10000):
    # GMM
    part = mixture.GaussianMixture(
        n_components=2,
        covariance_type="spherical",
        max_iter=300)
    part.fit(side_channel.reshape(-1, 1))
    ranges = [(mean - 3*stdev, mean + 3*stdev, mean)
               for mean, stdev in zip(part.means_, np.sqrt(part.covariances_))]
    ranges = sorted(ranges , key=lambda x: x[1]-x[0])

    # segmentation
    window_size = int(len(side_channel)/n_search_windows)
    segment_lines = []
    prev_segment = None
    for i in range(n_search_windows):
        window = side_channel[i*window_size: (i+1)*window_size]
        med = statistics.median(window)
        if med > ranges[0][0] and med < ranges[0][1]:
            if prev_segment is None:
                prev_segment = 0
            elif prev_segment == 1:
                segment_lines += [ i*window_size ]
                prev_segment = 0
        elif med > ranges[1][0] and med < ranges[1][1]:
            if prev_segment is None:
                prev_segment = 1
            elif prev_segment == 0:
                segment_lines += [ i*window_size ]
                prev_segment = 1

    print(segment_lines)
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
    print(chosen_group)

    # filter segmentation
    segments = []
    for a, b in zip(segment_lines[:-1], segment_lines[1:]):
        if abs(b-a) in chosen_group:
            segments += [(a, b)]
    return (segments, ranges)
