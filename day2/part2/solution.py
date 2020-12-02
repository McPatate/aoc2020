with open('input.txt', 'r') as f:
    i = f.read().rstrip().split('\n')

def check_rule(rule, pwd):
    positions, letter = rule.split(' ')
    first_pos, second_pos = positions.split('-')
    first_pos = int(first_pos) - 1
    second_pos = int(second_pos) - 1
    match = 0
    if pwd[first_pos] == letter:
        match += 1
    if pwd[second_pos] == letter:
        match += 1

    if match == 1:
        return True
    return False

count = 0
for v in i:
    rule, pwd = v.split(":")
    if check_rule(rule, list(pwd.lstrip())):
        count += 1

print(f"There are {count} valid pwds")