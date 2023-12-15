L = open(0).read()

instructions = L.split(",")
total = 0


def HASH(inst):
    hash = 0
    for char in inst:
        hash = (hash + ord(char)) * 17 % 256
    return hash


hashmap = {}
for i in range(0, 255 + 1):
    hashmap[i] = {}

for inst in instructions:
    if "=" in inst:
        label, number = inst.split("=")

    elif "-" in inst:
        label = inst.split("-")[0]
    else:
        raise ValueError("No '-' or '=' found")

    box = HASH(label)
    if number:
        hashmap[box][label] = number
    else:
        if hashmap[box].get(label):
            del hashmap[box][label]
    label, number = None, None

total = 0
for k, v in hashmap.items():
    # number = len(v)
    for idx, (kk, vv) in enumerate(v.items()):
        total += int(vv) * (int(k) + 1) * (idx + 1)

print(total)
