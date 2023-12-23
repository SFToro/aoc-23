import sys

L = open(0).read().splitlines()

rules = {}

for i, line in enumerate(L):
    if line == "":
        parts = L[i + 1 :]
        break
    rule_name, rest = line.split("{")
    rule_set = rest.strip("}")
    rules[rule_name] = []
    for rule in rule_set.split(","):
        if ":" in rule:
            rules[rule_name].append(rule.strip())
        else:
            rules[rule_name][-1] += "|" + rule.strip()

rules["A"] = "accepted"
rules["R"] = "rejected"


devices = []
for part in parts:
    xyz = part.strip("{}").split(",")
    devices.append({device.split("=")[0]: int(device.split("=")[1]) for device in xyz})

accepted = 0
rejected = 0
while devices:
    dev = devices.pop()
    ruleset = rules["in"]
    TEST_ONGOING = True
    while TEST_ONGOING:
        rule = ruleset[0]
        while rule:
            if ruleset == "accepted":
                TEST_ONGOING = False
                accepted += sum(value for (k, value) in dev.items())
                break
            if ruleset == "rejected":
                TEST_ONGOING = False
                rejected += 1
                break
            else:
                norm, destination = rule.split(":")

                TEST = None
                if ">" in norm:
                    device_type, norm_value = norm.split(">")
                    TEST = dev[device_type] > int(norm_value)
                if "<" in norm:
                    device_type, norm_value = norm.split("<")
                    TEST = dev[device_type] < int(norm_value)

                if "|" in destination:
                    if TEST:
                        ruleset = rules[destination.split("|")[0]]
                        rule = rules[destination.split("|")[0]][0]
                    else:
                        ruleset = rules[destination.split("|")[1]]
                        rule = rules[destination.split("|")[1]][0]
                else:
                    if TEST:
                        ruleset = rules[destination]
                        rule = rules[destination][0]
                    else:
                        current_idx = ruleset.index(rule)
                        if current_idx + 1 < len(ruleset):
                            rule = ruleset[current_idx + 1]
                        else:
                            raise Exception(
                                "No more rules to test and no pipe destination"
                            )


print(accepted)
