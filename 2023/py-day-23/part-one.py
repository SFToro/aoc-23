L = open(0).read().splitlines()

grid = {}
for i, line in enumerate(L):
    for j, c in enumerate(line):
        grid[(i, j)] = c

S = [(0, j) for j in range(len(L[0])) if grid[(0, j)] == "."][0]


def print_grid(points):
    for i, line in enumerate(L):
        for j, c in enumerate(line):
            if (i, j) in points:
                print("O", end="")
            else:
                print(c, end="")
        print()


def get_neighbors(pos):
    x, y = pos
    if grid[pos] == ">":
        available = [(0, 1)]
    elif grid[pos] == "<":
        available = [(0, -1)]
    elif grid[pos] == "^":
        available = [-1, 0]
    elif grid[pos] == "v":
        available = [(1, 0)]
    else:
        available = [(0, 1), (0, -1), (1, 0), (-1, 0)]
    return [
        (x + dx, y + dy)
        for (dx, dy) in available
        if (x + dx, y + dy) in grid and grid[(x + dx, y + dy)] != "#"
    ]


visited = set()
path = set((S,))
Q = [(S, 0, path)]
max_steps = 0
while Q:
    pos, steps, path = Q.pop(0)

    visited.add(pos)
    x, y = pos
    if x == len(L) - 1:
        if steps > max_steps:
            max_steps = steps

    for n in get_neighbors(pos):
        if n in path:
            continue

        new_path = path.copy()
        new_path.add(n)
        Q.append((n, steps + 1, new_path))

# print_grid(visited)
print(max_steps)
