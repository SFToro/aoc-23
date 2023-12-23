from typing import Tuple
from collections import deque
from heapq import heappop, heappush


class grapho:
    def __init__(self, text: str):
        L = text.splitlines()
        self.grid = {}
        for i, _ in enumerate(L):
            for j, char in enumerate(L[i]):
                self.grid[(i, j)] = int(char)
        self.visited = set()
        self.queue = []
        self.distances = {}
        self.parents = {}
        self.m = len(L)
        self.n = len(L[0])
        self.graph = {}
        self.target = None
        self.path = []

        for node in self.grid:
            neighbours = self.get_neighbours(node, self.grid)
            self.graph[node] = {}
            for neighbour in neighbours:
                self.graph[node][neighbour["position"]] = neighbour["distance"]

    def __str__(self):
        # self.parents[(0, 0)] = None
        # self.target = (self.m - 1, self.n - 1)
        # parent = self.target
        # parents = [parent]
        # while parent is not None:
        #     parent = self.parents[parent]
        #     parents.append(parent)
        result = ""
        for i in range(self.m):
            row = "".join(
                f"{self.grid[(i,j)]}" if (i, j) in self.path else "."
                for j in range(self.n)
            )
            result += row + f" {i}\n"
        return result

    def get_neighbours(self, node: Tuple[int, int], grid):
        positions = [(0, 1), (0, -1), (1, 0), (-1, 0)]
        # positions = [pos for pos in all_positions if pos != direction and count < 3]
        for position in positions:
            new_node = (node[0] + position[0], node[1] + position[1])
            if 0 <= new_node[0] < self.m and 0 <= new_node[1] < self.n:
                yield (
                    {
                        "position": new_node,
                        "distance": grid[new_node],
                        # "direction": (position[0], position[1]),
                        # "count": count + 1,
                    }
                )

    def shortest_distance_within_moves(self, start, end):
        distances = {node: float("inf") for node in self.graph}
        distances[start] = self.grid[start]
        visited = set()
        heap = [(self.grid[start], start, (0, 0), 1, (start,))]

        while heap:
            curr_distance, curr_node, (dx, dy), count, path = heappop(heap)
            if (curr_node, (dx, dy), count) in visited:
                continue
            visited.add((curr_node, (dx, dy), count))

            if curr_node == end:
                self.path = path
                print(self.path)

                print(self)

                print("total distance", curr_distance)

            for neighbour in self.graph[curr_node]:
                if neighbour in path:
                    continue
                cost = self.graph[curr_node][neighbour]
                new_dx = neighbour[0] - path[-1][0]
                new_dy = neighbour[1] - path[-1][1]

                if len(path) > 3:
                    dir_4 = (path[-3][0] - path[-4][0], path[-3][1] - path[-4][1])

                    dir_3 = (path[-2][0] - path[-3][0], path[-2][1] - path[-3][1])
                    dir_2 = (path[-1][0] - path[-2][0], path[-1][1] - path[-2][1])
                    dir_1 = (neighbour[0] - path[-1][0], neighbour[1] - path[-1][1])

                    if dir_3 == dir_2 and dir_2 == dir_1 and dir_1 == dir_4:
                        continue

                new_path = (*path, neighbour)

                heappush(
                    heap,
                    (
                        curr_distance + cost,
                        neighbour,
                        (new_dx, new_dy),
                        count,
                        new_path,
                    ),
                )

        return -1


text = open(0, encoding="utf-8").read()


g = grapho(text)
end = (g.m - 1, g.n - 1)
distance = g.shortest_distance_within_moves((0, 0), end)
# print(shortest_path)
print("distance", distance)
# print(g)
