with open("input") as file:
    data = file.read().split('\n')

maxes_cals = []
total = 0
for line in data:
    if line:
        line = int(line)
        total += line
    else:
        maxes_cals.append(total)
        total = 0
maxes_cals.sort(reverse=True)
print(sum(maxes_cals[:3]))