L=open(0).readlines()

total = 0
for line in L:
    (a,b,c) = (int(x) for x in line.split('x'))
    (a,b,c) = sorted([a,b,c])
    
    total+=2*a+2*b+a*b*c
print(total)    
