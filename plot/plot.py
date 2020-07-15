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

side_channel = get_list(sys.argv[1])

markersize=3
plt.plot(side_channel, marker="o", markersize=markersize, color="blue", linestyle="None")
plt.locator_params(nbins=50)
plt.show()
