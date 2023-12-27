from typing import Optional
import numpy as np
from collections import defaultdict


L = open(0).read().splitlines()

rules = {}


def negated_norm(s: str):
    negated_rule = ""
    if "<" in s:
        negated_rule = s.replace("<", ">=")
    else:
        negated_rule = s.replace(">", "<=")
    return negated_rule


for i, line in enumerate(L):
    if line == "":
        break
    rule_name, rest = line.split("{")
    rule_set = rest.strip("}")
    rules[rule_name] = []
    for rule in rule_set.split(","):
        if ":" in rule:
            rules[rule_name].append(rule.strip())
        else:
            rules[rule_name][-1] += "|" + rule.strip()

    mapping = defaultdict(list)
    previous = ""
    for rule in rules[rule_name]:
        norm, dest = rule.split(":")
        if "|" in dest:
            a, b = dest.split("|")
            mapping[a].append(previous + norm)
            mapping[b].append(previous + negated_norm(norm))
        else:
            mapping[dest].append(previous + norm)

        previous += negated_norm(norm) + ","
    rules[rule_name] = mapping

path = tuple(("in",))
queue = [("in", path, "")]
paths = []
while queue:
    rule, path, ruleset = queue.pop()
    if rule == "A":
        paths.append((tuple(path), ruleset))
        continue
    if rule == "R":
        continue
    for dest, norms in rules[rule].items():
        for norm in norms:
            new_norm = ruleset + "," + norm
            if dest not in path:
                new_path = tuple(path)
                new_path += (dest,)
                queue.append((dest, new_path, new_norm))


combinations = []
for path, ruleset in paths:
    items = {"x": [1, 4000], "m": [1, 4000], "a": [1, 4000], "s": [1, 4000]}
    for rule in ruleset.split(","):
        if rule == "":
            continue
        item_type = rule[0]

        if "<" in rule:
            if "=" in rule:
                _, num = rule.split("<=")
                if items[item_type][1] > int(num):
                    items[item_type][1] = int(num)
            else:
                _, num = rule.split("<")
                if items[item_type][1] > int(num):
                    items[item_type][1] = int(num) - 1
        elif ">" in rule:
            if "=" in rule:
                _, num = rule.split(">=")
                if items[item_type][0] < int(num):
                    items[item_type][0] = int(num)
            else:
                _, num = rule.split(">")
                if items[item_type][0] < int(num):
                    items[item_type][0] = int(num) + 1
    combinations.append(items)

total = 0

for combination in combinations:
    total += np.prod([combination[x][1] - combination[x][0] + 1 for x in combination])

print(total)
