import copy
import numpy as np


f = open(0)
L = [[char for char in line.rstrip("\n")] for line in f.readlines()]
R = len(L)
C = len(L[0])


def print_grid(L):
    L2 = ["".join(line) for line in L]
    for i, line in enumerate(L2):
        print(f"{line} {len(L2)-i}")
    print()


def string_grid(L):
    return "\n".join("".join(char for char in line) for line in L)


def tilt(L):
    # NORTH

    for i in range(1, R):
        for k in reversed(range(1, i + 1)):
            for j in range(C):
                if L[k][j] == "O" and L[k - 1][j] == ".":
                    L[k][j] = "."
                    L[k - 1][j] = "O"

    # WEST
    for j in range(1, C):
        for k in reversed(range(1, j + 1)):
            for i in range(R):
                if L[i][k] == "O" and L[i][k - 1] == ".":
                    L[i][k] = "."
                    L[i][k - 1] = "O"
    # SOUTH
    for i in range(0, R):
        for k in reversed(range(0, i)):
            for j in range(C):
                if L[k][j] == "O" and L[k + 1][j] == ".":
                    L[k][j] = "."
                    L[k + 1][j] = "O"

    # EAST
    for j in range(0, C):
        for k in reversed(range(0, j)):
            for i in range(R):
                if L[i][k] == "O" and L[i][k + 1] == ".":
                    L[i][k] = "."
                    L[i][k + 1] = "O"

    return L


def sum():
    sum = 0
    for idx, line in enumerate(L):
        for char in line:
            if char == "O":
                sum += len(L) - idx
    return sum


seen = {string_grid(L)}
array = [copy.deepcopy(L)]
iter = 0

while True:
    iter += 1
    L = tilt(L)
    if string_grid(L) in seen:
        break
    seen.add(string_grid(L))
    array.append(copy.deepcopy(L))


print(iter)
first = array.index(L)
print(first)
L = array[(1_000_000_000 - first) % (iter - first) + first]
print(sum())
