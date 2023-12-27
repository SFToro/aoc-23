import numpy as np

L = open(0).read().splitlines()


eqs = []
positions = []
velocities = []
constants = []
for line in L:
    pos, v = line.split("@")
    pos = [int(n) for n in pos.strip().split(", ")]
    v = [int(n) for n in v.strip().split(", ")]
    positions.append(pos[:-1])
    velocities.append(v[:-1])

    eqs.append([v[1], -v[0]])
    constants.append(pos[0] * v[1] - pos[1] * v[0])

limits = [2e14, 4e14]
TOTAL = 0
for i in range(len(eqs) - 1):
    for j in range(i + 1, len(eqs)):
        coeff = np.array([eqs[i], eqs[j]])
        constants_matrix = np.array([constants[i], constants[j]])
        try:
            x, y = np.linalg.solve(coeff, constants_matrix)
            if limits[0] < x < limits[1] and limits[0] < y < limits[1]:
                xi, yi = positions[i]
                vxi, vyi = velocities[i]

                xj, yj = positions[j]
                vxj, vyj = velocities[j]
                xOK = False
                yOK = False
                if (vxi > 0 and x > xi or vxi < 0 and x < xi) and (
                    vxj > 0 and x > xj or vxj < 0 and x < xj
                ):
                    xOK = True
                if (vyi > 0 and y > yi or vyi < 0 and y < yi) and (
                    vyj > 0 and y > yj or vyj < 0 and y < yj
                ):
                    yOK = True

                if xOK and yOK:
                    TOTAL += 1
        except np.linalg.LinAlgError:
            # print("parallel lines")
            continue

print(TOTAL)
