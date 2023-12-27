import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D

L = open(0).read().splitlines()


eqs = []
positions = []
velocities = []
constants = []
# Create a 3D plot
fig = plt.figure(figsize=(8, 6))
ax = fig.add_subplot(111, projection="3d")
for line in L:
    pos, v = line.split("@")
    pos = np.array([int(n) for n in pos.strip().split(", ")])
    v = np.array([int(n) for n in v.strip().split(", ")])
    # Create points along the line
    num_points = 10  # Number of points to plot the line
    t_values = np.linspace(-10, 10, num_points)  # Parameter values for the line

    # Calculate the line points using the direction vector and starting point
    line_points = np.array([pos + t * v for t in t_values])

    # Extract x, y, and z coordinates for plotting
    x_values = line_points[:, 0]
    y_values = line_points[:, 1]
    z_values = line_points[:, 2]

    # Plot the line
    ax.plot(
        x_values, y_values, z_values, label="Line with Direction Vector", color="blue"
    )
    ax.scatter(
        pos[0],
        pos[1],
        pos[2],
        color="red",
        label="Starting Point",
    )
pos = np.array([24, 13, 10])
v = np.array([-3, 1, 2])
num_points = 10  # Number of points to plot the line
t_values = np.linspace(-10, 10, num_points)  # Parameter values for the line

# Calculate the line points using the direction vector and starting point
line_points = np.array([pos + t * v for t in t_values])
x_values = line_points[:, 0]
y_values = line_points[:, 1]
z_values = line_points[:, 2]

# Plot the line
ax.plot(x_values, y_values, z_values, label="Line with Direction Vector", color="green")
# Set labels and title
ax.set_xlabel("X-axis")
ax.set_ylabel("Y-axis")
ax.set_zlabel("Z-axis")
ax.set_title("Plotting a 3D Line with Direction Vector")
# ax.legend()
ax.grid(True)

plt.show()
