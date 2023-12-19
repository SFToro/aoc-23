
L = open(0).read().splitlines()

position = (0,0)
visited = []
visited.append(position)
DIRECTIONS = {"0":[0,1],"2":[0,-1],"3":[-1,0],"1":[1,0]}
P=1
for line in L:
    direction,_, hexa = line.split()
    hexa = hexa.replace("(","").replace(")","")
    hexa, direction = hexa[1:-1], hexa[-1]
    direction = DIRECTIONS[direction]
    delta = int(hexa,16)
    position = (position[0]+delta*direction[0],position[1]+delta*direction[1])
    visited.append(position)
    P+=delta

AG=0

AG = abs(sum(visited[i][0]*visited[i+1][1] - visited[i+1][0]*visited[i][1] for i in range(len(visited)-1)))/2


I =AG-(P/2)+1
A=I+P
print("Area:",A)
print("perimeter:",P)
print("inner:",I)


# m = max(vis[0] for vis in visited)
# n = max(vis[1] for vis in visited)
# for i in range(m+1):
#     row ="".join(f"#" if (i,j) in visited else "." for j in range(n+1))
#     print(row)



    