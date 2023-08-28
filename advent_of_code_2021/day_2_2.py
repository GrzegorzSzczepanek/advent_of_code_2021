input = list(map(lambda x: x.split(" "), open("input_2.txt").read().split("\n")))

def day2(input):
    horizontal = 0
    depth = 0
    aim = 0
    for i in input:
        if i[0] == "down":
            aim += int(i[1])
        elif i[0] == "up":
            if aim > 0:
                aim -= int(i[1])
        elif i[0] == "forward":
            horizontal += int(i[1])
            depth += int(i[1]) * aim

    return horizontal * depth

print(day2(input))