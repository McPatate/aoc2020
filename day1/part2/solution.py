import numpy as np

with open('input.txt', 'r') as f:
    i = [int(v) for v in f.read().rstrip().split('\n')]

i.sort()
np_in = np.asarray(i)

filtered_high = np_in[np_in < 2020 - np_in[0] - np_in[1]]

found = False

for v in filtered_high:
    if found:
        break
    for v2 in filtered_high[1:]:
        if not found:
            for v3 in filtered_high[2:]:
                s = v + v2 + v3
                if s == 2020:
                    print(f"pt2's solution is : {v} * {v2} * {v3} = {v*v2*v3}")
                    found = True
                    break
