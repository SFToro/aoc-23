import sys
import math
from typing import Optional

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

accepting_rules = []
for name, ruleset in rules.items():
    for rule in ruleset:
        if ("A") in rule:
            accepting_rules.append(name)

rule_tree = {}
for acept_rule in accepting_rules:
    parent_rule = None
    rule_tree[acept_rule] = []
    while parent_rule != "in":
        for name, ruleset in rules.items():
            for rule in ruleset:
                if parent_rule is not None and parent_rule in rule:
                    rule_tree[acept_rule].append(name)
                    parent_rule = name
                elif parent_rule is None and acept_rule in rule:
                    rule_tree[acept_rule].append(name)
                    parent_rule = name


def negated(n: Optional[int]) -> Optional[int]:
    if n is None:
        return None
    return 4000 - n


# print(combinations_px)
def negated_norm(s: str):
    negated_rule = ""
    if "<" in s:
        negated_rule = s.replace("<", ">")
    else:
        negated_rule = s.replace(">", "<")
    return negated_rule


for rule_name, ruleset in rules.items():
    previous = None
    mapping = {}
    previous = ""
    for rule in ruleset:
        norm, dest = rule.split(":")
        if "|" in dest:
            a, b = dest.split("|")
            mapping[a] = previous + norm
            mapping[b] = previous + negated_norm(norm)
        else:
            mapping[dest] = previous + norm

        previous += negated_norm(norm) + ","
    rules[rule_name] = mapping


print(rules)
# if "<" in norm:
#     item, number = norm.split("<")
#     number = int(number)
#     items[item] = number
# if ">" in norm:
#     item, number = norm.split(">")
#     number = int(number)
#     items[item] = negated(number)

# chance = previous
# previous = negated(number)

# if "|" in dest:
#     a, b = dest.split("|")
