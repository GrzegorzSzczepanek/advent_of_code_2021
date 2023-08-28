import numpy as np
input = np.array(list(map(lambda x: list(x), open("input_3.txt", "r").read().split("\n")))).T.tolist()

def day_3_1(input):
    gamma = ""
    epsilon = ""

    for i in input:
        gamma += max(set(i), key=i.count)
        epsilon += min(set(i), key=i.count)

    return int(gamma, 2) * int(epsilon, 2)

print(day_3_1(input))

i = [1,1,0,0]
