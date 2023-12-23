L = open(0).read().splitlines()

grid = {}
for i, line in enumerate(L):
    for j, c in enumerate(line):
        grid[(i, j)] = c

S = (0, L[0].index("."))
end = (len(L) - 1, L[len(L) - 1].index("."))


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

    available = [(0, 1), (0, -1), (1, 0), (-1, 0)]
    return [
        (x + dx, y + dy)
        for (dx, dy) in available
        if (x + dx, y + dy) in grid and grid[(x + dx, y + dy)] != "#"
    ]


points = set((S, end))
for point, c in grid.items():
    if c == "#":
        continue
    neighbours = get_neighbors(point)
    # The maze is made so that in a given point all paths contract to a single node
    if len(neighbours) >= 3:
        points.add(point)

graph = {pt: {} for pt in points}

for pt in points:
    stack = [(0, pt)]
    seen = {pt}
    while stack:
        n, point = stack.pop()
        x, y = point

        if n != 0 and (x, y) in points:
            graph[pt][(x, y)] = n
            continue

        for neighbour in get_neighbors((x, y)):
            if neighbour not in seen:
                stack.append((n + 1, neighbour))
                seen.add((x, y))
seen = set()


def dfs(pt):
    if pt == end:
        return 0

    m = -float("inf")
    seen.add(pt)
    for nx in graph[pt]:
        if nx in seen:
            continue
        m = max(m, dfs(nx) + graph[pt][nx])
    seen.remove(pt)

    return m


print(dfs(S))
