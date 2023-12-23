def is_valid_move(maze, x, y):
    rows, cols = len(maze), len(maze[0])
    return 0 <= x < rows and 0 <= y < cols and maze[x][y] != "#"


def dfs(maze, x, y, end_x, end_y, visited):
    if x == end_x and y == end_y:
        return 1

    directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]
    visited.add((x, y))
    longest_path = 0

    for dx, dy in directions:
        new_x, new_y = x + dx, y + dy
        if is_valid_move(maze, new_x, new_y) and (new_x, new_y) not in visited:
            path_length = dfs(maze, new_x, new_y, end_x, end_y, visited)
            longest_path = max(longest_path, path_length)

    visited.remove((x, y))
    return 1 + longest_path if longest_path > 0 else 0


def longest_path_in_maze(maze, start_x, start_y, end_x, end_y):
    visited = set()
    return dfs(maze, start_x, start_y, end_x, end_y, visited)


L = open(0).read().splitlines()
grid = []
for i, line in enumerate(L):
    row = []
    for j, c in enumerate(line):
        row.append(c)
    grid.append(row)
# Example maze (0 represents an open path, 1 represents a wall)

start_point = [(0, j) for j in range(len(grid[0])) if grid[0][j] == "."][
    0
]  # Start point coordinates
end_point = [
    (len(grid) - 1, j) for j in range(len(grid[0]) - 1) if grid[len(grid) - 1][j] == "."
][0]
result = longest_path_in_maze(
    grid, start_point[0], start_point[1], end_point[0], end_point[1]
)
print("Longest path length from", start_point, "to", end_point, "in the maze:", result)
