import matplotlib.pyplot as plt
import numpy as np 

n = 13
i = 4

with open('durations_'+str(n)+'_'+str(i)+'.dat') as f:
    lines = f.readlines()
    lines.pop(0)
    x = []
    for line in lines:
        index = int(line.split()[0])
        dur = float(line.split()[1])
        while index >= len(x):
            x.append([])
        x[index].append(dur)

fig, ax = plt.subplots()

ax.boxplot(x)

plt.show()