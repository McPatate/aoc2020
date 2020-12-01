with open('input.txt', 'r') as f:
    input = [int(v) for v in f.read().rstrip().split('\n')]

input.sort()
low = []
high = []

for v in input:
    if v <= 1010:
        low.append(v)
    else:
        high.append(v)

looking = True
for l in low:
    if looking:
        for h in high:
            if l + h == 2020:
                print(f"Today's solution is : {l} * {h} = {l*h}")
                looking = False
                break
    else:
        break

