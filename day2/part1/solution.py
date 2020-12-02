with open('input.txt', 'r') as f:
    i = f.read().rstrip().split('\n')

def check_rule(rule, pwd):
    occ_interval, letter = rule.split(' ')
    mini, maxi = occ_interval.split('-')
    mini = int(mini)
    maxi = int(maxi)
    occ_nb = pwd.count(letter)
    if occ_nb <= maxi and occ_nb >= mini:
        return True
    return False

count = 0
for v in i:
    rule, pwd = v.split(":")
    if check_rule(rule, list(pwd.lstrip())):
        count += 1

print(f"There are {count} valid pwds")