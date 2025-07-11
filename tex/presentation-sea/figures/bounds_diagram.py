import matplotlib.pyplot as plt
import numpy as np

# Data
n = np.linspace(0, 21, 100)  # Sample n values

# Plot lines
plt.figure(figsize=(35,16))
plt.plot(n, 2.95 * n, label='Median upper bound: $2.95n$', linewidth=5)
plt.plot(n, 2.41 * n, label='Median lower bound: $2.41n$', linewidth=5)
plt.plot(n, n - 1, label='Selecting smallest: $n-1$', linewidth=5)

# Shade the gap between lower and upper bounds
plt.fill_between(n, n - 1, 2.41*n, alpha=0.2, color='orange')
plt.fill_between(n, 2.41 * n, 2.95 * n, alpha=0.2, color='yellow')

# Annotations
mid_n = 19
mid_val = 50
plt.annotate('Gap between bounds',
             xy=(mid_n, mid_val),
             xytext=(mid_n + 2, mid_val + 4),
             arrowprops=dict(arrowstyle='->',color='black', linewidth=4),
             fontsize=55,
             )

mid_n = 19
mid_val = 25
plt.annotate('Gap between i=0 and median',
             xy=(mid_n, mid_val),
             xytext=(mid_n + 2, mid_val + 4),
             arrowprops=dict(arrowstyle='->',color='black', linewidth=4),
             fontsize=55,
             )

# Labels and legend
plt.xlabel('Input size $n$', fontsize=60, labelpad=40)
plt.ylabel('Number of comparisons', fontsize=60, labelpad=40)
plt.tick_params(axis='both', length=18, width=3, labelsize=40)
plt.xticks(np.arange(0, 21, 5), fontsize=40)
plt.legend(fontsize=48)
plt.tight_layout()

# Display
plt.savefig("tex/presentation-sea/figures/bounds_diagram.png")
