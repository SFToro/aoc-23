import copy


f = open(0)
L = [line.rstrip("\n") for line in f.readlines()]
R = len(L)
C = len(L[0])


def print_grid(L):
    L2 = ["".join(line) for line in L]
    for i, line in enumerate(L2):
        print(f"{line} {len(L2)-i}")
    print()


def string_grid(L):
    return "\n".join(line for line in L)


def tilt(L):
    for _ in range(0, 4):
        # 90 deg turn
        L = ["".join(line) for line in list(zip(*(L[::-1])))]
        for idx, line in enumerate(L):
            line = "#".join(["".join(sorted(fragment)) for fragment in line.split("#")])
            L[idx] = line

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


first = array.index(L)
L = array[(1_000_000_000 - first) % (iter - first) + first]
print(sum())
