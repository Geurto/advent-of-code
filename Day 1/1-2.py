with open('Day 1\data', 'r') as datafile:
    count = 0
    curr = []
    prev = []
    for i in range(3):
        line = datafile.readline()
        curr.append(int(line))
        if i > 0:
            prev.append(int(line))
    while (line := datafile.readline()):
        curr.append(int(line))
        curr.pop(0)
        prev.append(curr[1])
        if len(prev) > 3:
            prev.pop(0)
        
        print(sum(curr), sum(prev))
        if sum(curr) > sum(prev):
            count += 1
    
print(count)