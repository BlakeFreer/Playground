import numpy as np
import matplotlib.pyplot as plt
from skimage.filters import threshold_otsu

NOISE = 6
WINDOW = 1000

signal = np.zeros(4000, dtype=int)
signal[1000:1500] = 100
signal[2000:2200] = 100 * np.random.randint(0, 1 + 1, 200)
signal += np.random.randint(-NOISE, NOISE, signal.shape)

time = np.arange(signal.size)
t_time = time[1:]

t = [threshold_otsu(signal[max(0, k - WINDOW) : k].reshape((1, -1))) for k in t_time]

plt.plot(time, signal)
plt.axhline(y=50, ls="--", c="grey")
plt.plot(t_time, t)

plt.show()
