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

baseline_list = get_list(sys.argv[1])
normal_list = get_list(sys.argv[2])
subnormal_list = get_list(sys.argv[3])

baseline_avg = statistics.mean(baseline_list)
normal_avg = statistics.mean(normal_list)
subnormal_avg = statistics.mean(subnormal_list)

data = baseline_list + normal_list + subnormal_list
nbins = 1000
binwidth = int((max(data) - min(data))/nbins)
bins = range(min(data), max(data) + binwidth, binwidth)

sns.set(color_codes=True)
sns.distplot(normal_list, bins=bins, kde=False, rug=True, label="normal");
sns.distplot(subnormal_list, bins=bins, kde=False, rug=True, label="subnormal");
sns.distplot(baseline_list, bins=bins, kde=False, rug=True, label="baseline");
plt.vlines(baseline_avg, 0, len(baseline_list), colors = 'green', linestyles='dashed', label="baseline_avg")
plt.vlines(normal_avg, 0, len(baseline_list), colors = 'blue', linestyles='dashed', label="normal_avg")
plt.vlines(subnormal_avg, 0, len(baseline_list), colors = 'orange', linestyles='dashed', label="subnormal_avg")
plt.locator_params(nbins=20)
plt.legend()
plt.show()
