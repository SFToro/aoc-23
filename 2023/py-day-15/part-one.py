L = open(0).read()

instructions = L.split(",")
total = 0
for inst in instructions:
    sum = 0
    for char in inst:
        print(ord(char), char)
        sum = (sum + ord(char)) * 17 % 256
    total += sum
print(total)
