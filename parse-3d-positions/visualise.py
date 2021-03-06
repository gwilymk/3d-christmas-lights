import matplotlib.pyplot as plt
import numpy as np
from mpl_toolkits.mplot3d import Axes3D

fig = plt.figure()
ax1 = fig.add_subplot(111,projection='3d')

x, y, z = np.loadtxt('output.csv', delimiter=',', unpack=True)

plt.plot(x,z,-y)
plt.show()