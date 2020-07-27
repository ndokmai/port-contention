from common import get_list_from_file, segmentation
import matplotlib.pyplot as plt
import seaborn as sns
import sys

side_channel = get_list_from_file(sys.argv[1])

(segments, ranges) = segmentation(side_channel)

bottom = ranges[1][0]
top = ranges[1][1]

# plot
colors = ["green", "magenta"]
for (lower, upper, mean), color in zip(ranges, colors):
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
