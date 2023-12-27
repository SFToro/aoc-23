import networkx as nx

L = open(0).read().splitlines()
G = nx.DiGraph()

for line in L:
    left, right = line.split(":")
    components = right.strip().split(" ")
    for component in components:
        G.add_edge(left, component, capacity=1)
        G.add_edge(component, left, capacity=1)

for x in G:
    for y in G:
        if x == y:
            continue
        cut, partition = nx.minimum_cut(G, x, y)
        if cut == 3:
            print(len(partition[0]) * len(partition[1]))
            exit(0)
