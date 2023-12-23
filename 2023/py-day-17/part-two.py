import heapq

L = open(0, encoding="utf-8").read().splitlines()
start = (0, 0)
queue = []
pre_turn = 4
max_straight = 10
m = len(L)
n = len(L[0])
goal = (m - 1, n - 1)

grid = []
for i, _ in enumerate(L):
    row = []
    for j, char in enumerate(L[i]):
        row.append(int(char))
    grid.append(row)
cost = 0  # cumulative cost of all moves, based on value of each entered location
# heuristic:int = 0 + start.manhattan_distance_from(goal)
current_posn = start
dirn = None  # Current direction. (None when we start.)
straight_steps: int = 0  # number of steps taken in one direction without turning
heapq.heappush(
    queue, (cost, current_posn, dirn, straight_steps)
)  # cost must come first for our priority queue

seen = set()

CHANGES = {
    (0, 1): [(1, 0), (-1, 0)],
    (1, 0): [(0, 1), (0, -1)],
    (-1, 0): [(0, 1), (0, -1)],
    (0, -1): [(1, 0), (-1, 0)],
}

while queue:
    cost, current_posn, dirn, straight_steps = heapq.heappop(queue)
    # logger.debug(f"{cost=}, {current_posn=}, {dirn=}, {steps=}")

    if current_posn == goal and straight_steps >= pre_turn:
        print(cost)

    # check if we've been in this configuration before
    if (current_posn, dirn, straight_steps) in seen:
        continue

    seen.add((current_posn, dirn, straight_steps))  # explored

    next_states = []  # point, dirn, steps
    if dirn is None:  # we're at the start, so we can go in any direction
        for dir_x, dir_y in ((0, 1), (1, 0), (0, -1), (-1, 0)):
            dirn = (dir_x, dir_y)
            neighbour = (current_posn[0] + dirn[0], current_posn[1] + dirn[1])
            next_states.append((neighbour, dirn, 1))

    else:
        if straight_steps >= pre_turn:
            for new_dirn in CHANGES[dirn]:
                next_states.append(
                    (
                        (current_posn[0] + new_dirn[0], current_posn[1] + new_dirn[1]),
                        (new_dirn[0], new_dirn[1]),
                        1,
                    )
                )
        if straight_steps < max_straight:  # we can move straight ahead.
            next_states.append(
                (
                    (current_posn[0] + dirn[0], current_posn[1] + dirn[1]),
                    (dirn[0], dirn[1]),
                    straight_steps + 1,
                )
            )

    for neighbour, dirn, new_steps in next_states:
        if 0 <= neighbour[1] < n and 0 <= neighbour[0] < m:
            new_cost = cost + grid[neighbour[0]][neighbour[1]]
            # heuristic = new_cost + neighbour.manhattan_distance_from(goal)
            heapq.heappush(queue, (new_cost, neighbour, dirn, new_steps))
