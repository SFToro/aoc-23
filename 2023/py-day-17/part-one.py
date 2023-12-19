from typing import Tuple
from collections import deque


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

        for node in self.grid:
            neighbours = self.get_neighbours(node, self.grid)
            self.graph[node] = {}
            for neighbour in neighbours:
                self.graph[node][neighbour["position"]] = neighbour["distance"]

    


    def __str__(self):
        m = max(node[0] for node in self.graph)
        n = max(node[1] for node in self.graph)
        parent = self.target
        parents = [parent]
        while parent is not None:
            parent = self.parents[parent][0]
            parents.append(parent)
        result = ""
        for i in range(m+1):
            row ="".join(f"{self.grid[(i,j)]}" if (i,j) in parents  else "." for j in range(n+1))
            result += row +f" {i}\n"
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

    def find_shortest_path(self, start_node, end_node):
        self.target = end_node
        self.queue.append((start_node, None,None))
        self.visited.add(start_node)
        self.distances[start_node] = 0
        self.parents[start_node] = None, None

        i =0
        while self.queue:
            i+=1
            if i > 10000:
                break
            current_node,in_dir,parent_in_dir = self.queue.pop(0)

            # if current_node == (0,5):
            
            last_two_directions = deque(maxlen=3)
            if in_dir and parent_in_dir:
                last_two_directions.append(self.parents[current_node][1])
                last_two_directions.append(self.parents[self.parents[current_node][0]][1])
                last_two_directions.append(self.parents[self.parents[self.parents[current_node][0]][0]][1])


            # print(f"Checking {current_node}, {last_two_directions}")
            for neighbour in self.graph[current_node]:
                if neighbour in self.visited:
                    continue

                direction = (
                    -current_node[0] + neighbour[0],
                    -current_node[1] + neighbour[1],
                )
                flag = True
                for dir in last_two_directions:
                    if dir != direction:
                        flag = False
                if flag and len(last_two_directions) == 3:
                    print(f"Skipping {neighbour} of {current_node}")
                    continue

                if (
                    self.distances.get(neighbour) is None
                    or self.distances[neighbour]
                    > (self.distances[current_node]
                    + self.graph[current_node][neighbour])
                ):
                    self.distances[neighbour] = (
                        self.distances[current_node]
                        + self.graph[current_node][neighbour]
                    )
                    self.parents[neighbour] = current_node, direction
                    # if current_node == (0,3):
                        # print(f"neighbour of {current_node} is ",neighbour)

                    if current_node == (0,1):
                        self.queue.append((neighbour,direction,(0,1)))
                    elif current_node == (1,0):
                        self.queue.append((neighbour,direction,(1,0)))
                    else:
                        self.queue.append((neighbour,direction,in_dir))
                        print(f"Adding {neighbour} from {current_node} with {direction} and {in_dir}")

                        
            self.visited.add(current_node)

        return self.distances.get(end_node)
    


text = open(0, encoding="utf-8").read()


g = grapho(text)
print(g.grid[(4,10)])
end=(3,11)
print(g.find_shortest_path((0, 0), end))
parent = end
# while parent is not None:
    # parent = g.parents[parent][0]
    # print(parent)    
# print(g.parents[(2,0)][0])
print(g)
