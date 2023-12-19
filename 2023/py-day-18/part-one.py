
L = open(0).read().splitlines()

position = (0,0)
visited = []
visited.append(position)
DIRECTIONS = {"R":[0,1],"L":[0,-1],"U":[-1,0],"D":[1,0]}

for line in L:
    direction,count, color = line.split()
    count = int(count)  
    color = color.replace("(","").replace(")","")
    for _ in range(count):
        x =position[0] + DIRECTIONS[direction][0]
        y =position[1] + DIRECTIONS[direction][1]
        position = (x,y)
        visited.append(position)


    
    

AG=0

AG = abs(sum(visited[i][0]*visited[i+1][1] - visited[i+1][0]*visited[i][1] for i in range(len(visited)-1)))/2

P = len(visited) -1

I =AG-(P/2)+1
A=I+P
print("Area:",A)
print("perimeter:",P)
print("inner:",I)


m = max(vis[0] for vis in visited)
n = max(vis[1] for vis in visited)
# for i in range(m+1):
#     row ="".join(f"#" if (i,j) in visited else "." for j in range(n+1))
#     print(row)



    