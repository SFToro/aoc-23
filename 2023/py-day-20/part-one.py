from collections import deque, Counter


class Module:
    def __init__(self, name, module_type, destinations):
        self.name = name
        self.type = module_type
        if module_type == "%":
            self.inputs = "off"
        else:
            self.inputs = {}
        self.destinations = destinations


L = open(0).read().splitlines()

pool = {}
for line in L:
    module, destinations = line.split(" -> ")
    destinations = destinations.split(", ")

    if module == "broadcaster":
        broadcaster = Module("broadcaster", "broadcast", destinations)

    else:
        name = module[1:]
        module_type = module[0]

        pool[name] = Module(name, module_type, destinations)


for name, module in pool.items():
    for destination in module.destinations:
        if destination in pool and pool[destination].type == "&":
            pool[destination].inputs[name] = 0

high = low = 0
for _ in range(1000):
    low += 1
    q = deque([("broadcaster", dest, 0) for dest in broadcaster.destinations])
    # print(f"button -low-> broadcaster")
    # for dest in start_destinations:
    # print(f"broadcaster -low-> {dest}")

    while q:
        source, destination, pulse = q.popleft()
        if pulse == 0:
            low += 1
        else:
            high += 1

        if destination not in pool:
            continue

        module = pool[destination]
        if module.type == "%":
            if pulse == 0:
                module.inputs = "on" if module.inputs == "off" else "off"
                signal = 1 if module.inputs == "on" else 0
                for dest in module.destinations:
                    q.append((module.name, dest, signal))
                    # print(f"{module.name} -{'low' if signal == 0 else 'high'}-> {dest}")
        else:
            module.inputs[source] = pulse
            signal = 0 if all(input == 1 for input in module.inputs.values()) else 1
            for dest in module.destinations:
                q.append((module.name, dest, signal))
                # print(f"{module.name} -{'low' if signal == 0 else 'high'}-> {dest}")


print(low * high)
