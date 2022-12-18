with open('Day 1\data', 'r') as datafile:
    datafile.readline()
    prev = 0
    count = 0
    while (line := datafile.readline()):
        if int(line) > prev:
            count += 1
        prev = int(line)
print(count)