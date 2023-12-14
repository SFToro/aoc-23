f = open(0)
L = [[char for char in line.rstrip("\n")] for line in f.readlines()]
R = len(L)
C = len(L[0])
for i in range(1, R):
    for k in reversed(range(1, i + 1)):
        print(i, k)
        for j in range(C):
            if L[k][j] == "O" and L[k - 1][j] == ".":
                L[k][j] = "."
                L[k - 1][j] = "O"

sum = 0
for idx, line in enumerate(L):
    for char in line:
        if char == "O":
            sum += len(L) - idx


print(sum)
