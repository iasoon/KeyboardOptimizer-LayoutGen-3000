import json

SUDOKU = [
    [   0, 2, 0,    0, 0, 0,    0, 0, 0 ],
    [   0, 0, 0,    6, 0, 0,    0, 0, 3 ],
    [   0, 7, 4,    0, 8, 0,    0, 0, 0 ],

    [   0, 0, 0,    0, 0, 3,    0, 0, 2 ],
    [   0, 8, 0,    0, 4, 0,    0, 1, 0 ],
    [   6, 0, 0,    5, 0, 0,    0, 0, 0 ],

    [   0, 0, 0,    0, 1, 0,    7, 8, 0 ],
    [   5, 0, 0,    0, 0, 9,    0, 0, 0 ],
    [   0, 0, 0,    0, 0, 0,    0, 4, 0 ],
]


def not_restriction(*values):
    return { "not": list(values) }

def neq_restrictor(values):
    return { v: not_restriction(v) for v in values }

def neq_constraint(origin, target, values):
    return {
        "origin": origin,
        "target": target,
        "restrictor": neq_restrictor(values),
    }

values = ['1', '2', '3', '4', '5', '6', '7', '8', '9']
keys = []
key_names = []


for i in range(9):
    for j in range(9):
        keys.append((i, j))
        key_names.append("({}, {})".format(i+1, j+1))


constraints = []
for (i, k1) in enumerate(keys):
    for (j, k2) in enumerate(keys):
        if i == j:
            continue
        
        same_row = (k1[0] == k2[0])
        same_col = (k1[1] == k2[1])
        same_square = (k1[0] // 3 == k2[0] // 3) and (k1[1] // 3 == k2[1] // 3)

        if same_row or same_col or same_square:
            c = neq_constraint(key_names[i], key_names[j], values)
            constraints.append(c)

config = {
    "keys": key_names,
    "values": values,
    "constraints": constraints,
}

with open('sudoku.json', 'w') as f:
    json.dump(config, f)