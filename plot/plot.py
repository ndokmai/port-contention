import sys
import matplotlib.pyplot as plt
import seaborn as sns
import numpy as np
import statistics
import sys

def get_list(filename):
    f = open(filename, "r")
    x = list(map(lambda x: int(x.strip()),  f.readlines()))
    return x

def remove_outliers(datapoints):
    n_stdev = 3
    mean = statistics.mean(datapoints)
    stdev = statistics.stdev(datapoints, mean)
    return [ i for i in datapoints if i < mean + n_stdev*stdev and i > mean - n_stdev*stdev]

baseline_list = get_list(sys.argv[1])
normal_list = get_list(sys.argv[2])
subnormal_list = get_list(sys.argv[3])

baseline_list = remove_outliers(baseline_list)
normal_list = remove_outliers(normal_list)
subnormal_list = remove_outliers(subnormal_list)

baseline_mean = statistics.mean(baseline_list)
normal_mean = statistics.mean(normal_list)
subnormal_mean = statistics.mean(subnormal_list)

data = baseline_list + normal_list + subnormal_list
nbins = 100
binwidth = int((max(data) - min(data) + nbins - 1)/nbins)
bins = range(min(data), max(data) + binwidth, binwidth)

sns.set(color_codes=True)
sns.distplot(normal_list, bins=bins, kde=False, rug=True, label="normal");
sns.distplot(subnormal_list, bins=bins, kde=False, rug=True, label="subnormal");
sns.distplot(baseline_list, bins=bins, kde=False, rug=True, label="baseline");
plt.vlines(baseline_mean, 0, len(baseline_list), colors = 'green', linestyles='dashed', label="baseline_mean")
plt.vlines(normal_mean, 0, len(baseline_list), colors = 'blue', linestyles='dashed', label="normal_mean")
plt.vlines(subnormal_mean, 0, len(baseline_list), colors = 'orange', linestyles='dashed', label="subnormal_mean")
plt.locator_params(nbins=30)
plt.legend()
plt.show()
