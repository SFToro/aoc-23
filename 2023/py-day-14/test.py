def print_grid(L):
    L2 = ["".join(line) for line in L]
    for i, line in enumerate(L2):
        print(f"{line} {len(L2)-i}")
    print()


f = open(0)

L = [line.rstrip("\n") for line in f.readlines()]
# l = L[0]
# print(L[0])
# print(l.split("#"))

# print(["".join(sorted(fragment)) for fragment in l.split("#")])

# print("#".join(["".join(sorted(fragment)) for fragment in l.split("#")]))
print("\n".join(l for l in L))
print("")
L = ["".join(line) for line in list(zip(*(L[::-1])))]
print("\n".join(l for l in L))
for idx, line in enumerate(L):
    line = "#".join(["".join(sorted(fragment)) for fragment in line.split("#")])
    L[idx] = line
print("")

print("\n".join(l for l in L))
# print_grid([sorted(l) for l in L])
