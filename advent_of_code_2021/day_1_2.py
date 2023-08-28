input = list(map(lambda x: int(x) ,open("input_1.txt", "r").read().split("\n")))
count = 0

for i in range(1, len(input)):
    if sum(input[i: i + 3]) > sum(input[i - 1: i + 2]):
        count += 1


print(count)