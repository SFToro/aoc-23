from typing import List, Tuple

L = open(0).read().split("\n")
m = len(L)
n = len(L[0])


new_beams = {"|": [(1, 0), (-1, 0)], "-": [(0, 1), (0, -1)]}
new_directions = {
    (-1, 0): {"\\": (0, -1), "/": (0, 1)},
    (1, 0): {"/": (0, -1), "\\": (0, 1)},
    (0, -1): {"\\": (-1, 0), "/": (1, 0)},
    (0, 1): {"/": (-1, 0), "\\": (1, 0)},
}


def print_grid(visited):
    grid = [["."] * n for _ in range(m)]
    for pos in visited.keys():
        x, y = pos
        grid[x][y] = "#"

    string_grid = "\n".join("".join(item for item in row) for row in grid)
    print(string_grid)
    print()


def beam(
    L: List[str],
    pos: Tuple[int, int],
    direction: Tuple[int, int],
):
    while 0 <= pos[0] < m and 0 <= pos[1] < n:
        if visited.get(pos) and direction in visited.get(pos):
            return
        if visited.get(pos):
            visited[pos].append(direction)
        else:
            visited[pos] = [direction]

        x, y = pos
        if L[x][y] in new_beams.keys():
            possible_directions = new_beams[L[x][y]]
            if not direction in possible_directions:
                direction_one = new_beams[L[x][y]][0]
                direction_two = new_beams[L[x][y]][1]

                beam(L, pos, direction_one)
                beam(L, pos, direction_two)
                return
        elif L[x][y] in new_directions[direction].keys():
            direction = new_directions[direction][L[x][y]]

        pos = (pos[0] + direction[0], pos[1] + direction[1])


MAX_TILES = 0
visited = {}

for i in range(m):
    visited = {}
    beam(L, pos=(i, 0), direction=(0, 1))
    MAX_TILES = len(visited) if len(visited) > MAX_TILES else MAX_TILES

    visited = {}
    beam(L, pos=(i, n), direction=(0, -1))
    MAX_TILES = len(visited) if len(visited) > MAX_TILES else MAX_TILES


for j in range(n):
    visited = {}
    beam(L, pos=(0, j), direction=(1, 0))
    MAX_TILES = len(visited) if len(visited) > MAX_TILES else MAX_TILES

    visited = {}
    beam(L, pos=(m, j), direction=(-1, 0))
    MAX_TILES = len(visited) if len(visited) > MAX_TILES else MAX_TILES


print(MAX_TILES)
