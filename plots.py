import matplotlib.pyplot as plt
import numpy as np 

n = 13
i = 3

def durations(n, i): 
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

def comp_posets(n, i):
    with open('comp_posets_'+str(n)+'_'+str(i)+'.dat') as f:
        lines = f.readlines()
        lines.pop(0)
        x = []
        for line in lines:
            comparisons = int(line.split()[0])
            comp_posets = int(line.split()[1])
            amount = int(line.split()[2])
            while comparisons >= len(x):
                x.append([])
            for _ in range(amount):
                x[comparisons].append(comp_posets)

    fig, ax = plt.subplots()

    ax.boxplot(x)

    plt.show()

comp_posets(n,i)