from collections import deque


def is_valid(grid, position):
    return position in grid and grid[position] != "#"


def get_n_neighbours(grid, position):
    count = 0
    for direction in directions:
        next_position = (position[0] + direction[0], position[1] + direction[1])
        if is_valid(grid, next_position):
            count += 1
    return -count


def print_grid(counted):
    for i, line in enumerate(L):
        for j, c in enumerate(line):
            if (i, j) in counted:
                print("O", end="")
            else:
                print(c, end="")
        print()


L = open(0).read().splitlines()

grid = {}
for i, line in enumerate(L):
    for j, c in enumerate(line):
        grid[(i, j)] = c
        if c == "S":
            start = (i, j)


directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]
visited = set(start)
queue = deque([(0, start)])
sum = 0
ans = set()

while queue:
    steps, position = queue.popleft()
    if position in visited:
        continue
    if steps % 2 == 0:
        ans.add(position)
    if steps > 64:
        break
    if steps == 64:
        continue

    visited.add(position)

    for direction in directions:
        next_position = (position[0] + direction[0], position[1] + direction[1])
        if is_valid(grid, next_position):
            if next_position not in visited:
                queue.append((steps + 1, next_position))

# # print(total)
# # print(len(total))
# for position in total:
#     for direction in directions:
#         next_position = (position[0] + direction[0], position[1] + direction[1])
#         if is_valid(grid, next_position):
#             ans.add(next_position)
# print_grid(ans)
print(len(ans))
