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

    # def add_edge(self, node, neighbour, distance):
    #     if node not in self.graph:
    #         self.graph[node] = {}
    #     self.graph[node][neighbour] = distance
    def __str__(self):
        self.parents[(0, 0)] = None
        self.target = (self.m - 1, self.n - 1)
        parent = self.target
        parents = [parent]
        while parent is not None:
            parent = self.parents[parent]
            parents.append(parent)
        result = ""
        for i in range(self.m):
            row = "".join(
                f"{self.grid[(i,j)]}" if (i, j) in parents else "."
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
        distances[start] = 0
        visited = set()
        heap = [(5, start, (0, 0), 0)]
        while heap:
            print("infinite")
            curr_distance, curr_node, (dx, dy), count = heappop(heap)
            if curr_node == (0, 3):
                print(curr_node, count, curr_distance)
            if (curr_node, (dx, dy), count) in visited:
                continue
            if curr_node == end:
                return curr_distance

            for neighbour in self.graph[curr_node]:
                cost = self.graph[curr_node][neighbour]
                # print(cost)
                if curr_node == (0, 3):
                    print(curr_node, count, curr_distance, neighbour, cost)
                # cost = -cost
                new_dx = neighbour[0] - curr_node[0]
                new_dy = neighbour[1] - curr_node[1]
                if (new_dx, new_dy) == (dx, dy):
                    count += 1
                else:
                    count = 1

                # Check movement constraints
                if neighbour[0] + neighbour[1] <= 4:
                    if count < 10:
                        new_distance = curr_distance + cost

                        heappush(
                            heap, (new_distance, neighbour, (new_dx, new_dy), count)
                        )
                else:
                    if count < 4 and count < 10:
                        new_distance = curr_distance + cost

                        heappush(
                            heap, (new_distance, neighbour, (new_dx, new_dy), count)
                        )

            visited.add((curr_node, (dx, dy), count))

        return -1


text = open(0, encoding="utf-8").read()


g = grapho(text)
end = (g.m - 1, g.n - 1)
distance = g.shortest_distance_within_moves((0, 0), end)
# print(shortest_path)
print("distance", distance)
# print(g)
