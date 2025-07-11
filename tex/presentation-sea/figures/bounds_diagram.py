import matplotlib.pyplot as plt
import numpy as np

# Data
n = np.linspace(1, 100, 500)  # Sample n values

# Plot lines
plt.figure(figsize=(24,16))
plt.plot(n, n - 1, label='Selecting smallest: $n-1$', linewidth=5)
plt.plot(n, 2.41 * n, label='Median lower bound: $2.41n$', linewidth=5)
plt.plot(n, 2.95 * n, label='Median upper bound: $2.95n$', linewidth=5)

# Shade the gap between lower and upper bounds
plt.fill_between(n, 2.41 * n, 2.95 * n, alpha=0.2, color='yellow')

# Annotations
mid_n = 50
mid_val = (2.41 + 2.95) / 2 * mid_n
plt.annotate('Gap between bounds',
             xy=(mid_n, mid_val),
             xytext=(mid_n + 10, mid_val + 20),
             arrowprops=dict(arrowstyle='->',color='black', linewidth=4),
             fontsize=44,
             )

# Labels and legend
plt.xlabel('Input size $n$', fontsize=50, labelpad=40)
plt.ylabel('Number of comparisons', fontsize=50, labelpad=40)
plt.tick_params(axis='both', length=18, width=3, labelsize=40)

plt.legend(fontsize=48)
plt.tight_layout()

# Display
plt.savefig("tex/presentation-sea/figures/bounds_diagram.png")
