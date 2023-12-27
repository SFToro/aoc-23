import numpy as np
from scipy.optimize import fsolve
import sympy


L = open(0).read().splitlines()


eqs = []
positions = []
velocities = []
constants = []

for line in L:
    pos, v = line.split("@")
    pos = np.array([int(n) for n in pos.strip().split(", ")])
    v = np.array([int(n) for n in v.strip().split(", ")])
    positions.append(pos)
    velocities.append(v)

previous = (0, 0, 0, 0, 0, 0, 0, 0, 0)

x0, y0, z0, lamda, mu, alpha, t, s, u = sympy.symbols(
    "x0, y0, z0, lamda, mu, alpha, t, s, u"
)
for i in range(len(positions) - 3):
    eqs = []
    eqs.append(positions[i][0] + velocities[i][0] * t - x0 - lamda * t)
    eqs.append(positions[i][1] + velocities[i][1] * t - y0 - mu * t)
    eqs.append(positions[i][2] + velocities[i][2] * t - z0 - alpha * t)
    eqs.append(positions[i + 1][0] + velocities[i + 1][0] * s - x0 - lamda * s)
    eqs.append(positions[i + 1][1] + velocities[i + 1][1] * s - y0 - mu * s)
    eqs.append(positions[i + 1][2] + velocities[i + 1][2] * s - z0 - alpha * s)
    eqs.append(positions[i + 2][0] + velocities[i + 2][0] * u - x0 - lamda * u)
    eqs.append(positions[i + 2][1] + velocities[i + 2][1] * u - y0 - mu * u)
    eqs.append(positions[i + 2][2] + velocities[i + 2][2] * u - z0 - alpha * u)

    answers = [
        soln for soln in sympy.solve(eqs) if all(x % 1 == 0 for x in soln.values())
    ]
    answer = answers[0]

    if len(answers) == 1:
        print(answer[x0] + answer[y0] + answer[z0])a
        break
