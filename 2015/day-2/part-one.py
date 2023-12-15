L=open(0).readlines()

total = 0
for line in L:
    (a,b,c) = (int(x) for x in line.split('x'))
    a1 = 2*a*b
    a2= 2*b*c
    a3= 2*a*c
    total+=(a1+a2+a3+min(a1,a2,a3)/2)

print(min(1,2,3))
