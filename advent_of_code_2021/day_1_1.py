input = list(map(lambda x: int(x) ,open("input_1.txt", "r").read().split("\n")))
count = 0

for i in range(1, len(input)):
    if input[i] > input[i-1]:
        count += 1

print(count)