import multiprocessing as mp
from typing import List, Tuple, Dict, Union, MutableSet

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


def beam(
    L: List[str],
    pos: Tuple[int, int],
    direction: Tuple[int, int],
    visited,
    lock,
    counter,
):
    while 0 <= pos[0] < m and 0 <= pos[1] < n:
        if visited.get(pos) and direction in visited.get(pos):
            with lock:
                counter.value -= 1
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
                p1 = mp.Process(
                    target=beam, args=(L, pos, direction_one, visited, lock, counter)
                )
                p2 = mp.Process(
                    target=beam, args=(L, pos, direction_two, visited, lock, counter)
                )
                with lock:
                    p1.start()
                    p2.start()
                    counter.value += 2

                with lock:
                    counter.value -= 1
                return
        elif L[x][y] in new_directions[direction].keys():
            direction = new_directions[direction][L[x][y]]

        pos = (pos[0] + direction[0], pos[1] + direction[1])
    with lock:
        counter.value -= 1


with mp.Manager() as manager:
    lock = manager.Lock()
    visited = manager.dict()
    counter = manager.Value("i", 1)
    beam(L, pos=(0, 0), direction=(0, 1), visited=visited, lock=lock, counter=counter)
    while True:
        with lock:
            if counter.value == 0:
                break

    grid = [["."] * n for _ in range(m)]
    for pos in visited.keys():
        x, y = pos
        grid[x][y] = "#"

    string_grid = "\n".join("".join(item for item in row) for row in grid)

    print(len(visited))
