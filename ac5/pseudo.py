from pprint import pprint
with open("input") as file:
    input = file.read().split('\n')

# create init state
initiallines = []
while True: 
    if not input[0]:
        break
    else:
        initiallines.append(input.pop(0))
piles = {}
pilekeys = {}
kee = 1
for i, number in enumerate(initiallines[-1]):
    if number != ' ':
        piles[kee] = []
        pilekeys[kee] = i
        kee += 1
initiallines.pop(-1)
for j, line in enumerate(initiallines):
    for cargo, vals in pilekeys.items():
        if line[vals] != ' ':  
            piles[cargo].append(line[vals]) 

for i,pile in piles.items():
    print(f'{i}: {pile}')

print("    ")

for line in input:
    if line:
        nums = line.split(' ') 
        howmany = int(nums[1])
        cfrom = int(nums[3])
        cto = int(nums[5])
        movedcrates = piles[cfrom][0:howmany]
        piles[cto] = movedcrates + piles[cto]

        for moveone in range(howmany):
            piles[cfrom].pop(0)

for i,pile in piles.items():
    print(f'{i}: {pile}')
        


finalstring = ''.join([x[0] for x in piles.values()])
print(finalstring)