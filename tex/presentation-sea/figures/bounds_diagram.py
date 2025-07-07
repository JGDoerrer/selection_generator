import matplotlib.pyplot as plt
import numpy as np

# Data
n = np.linspace(1, 100, 500)  # Sample n values

# Plot lines
plt.figure()
plt.plot(n, n - 1, label='Selecting smallest: $n-1$')
plt.plot(n, 2.41 * n, label='Median lower bound: $2.41n$')
plt.plot(n, 2.95 * n, label='Median upper bound: $2.95n$')

# Shade the gap between lower and upper bounds
plt.fill_between(n, 2.41 * n, 2.95 * n, alpha=0.2, color='yellow')

# Annotations
mid_n = 50
mid_val = (2.41 + 2.95) / 2 * mid_n
plt.annotate('Gap between bounds',
             xy=(mid_n, mid_val),
             xytext=(mid_n + 10, mid_val + 20),
             arrowprops=dict(arrowstyle='->'))

# Labels and legend
plt.xlabel('Input size $n$')
plt.ylabel('Number of comparisons')
plt.title('Comparison Bounds for the Selection Problem')
plt.legend()
plt.tight_layout()

# Display
plt.savefig("tex/presentation-sea/figures/bounds_diagram.png")
