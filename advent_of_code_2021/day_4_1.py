input = open("input_4.txt", "r").read().split("\n")
arr = []
for i in input:
    arr.append(i)


# for i in range(len(input)):
#     for j in range(len(input[i])):
#         input[i][j] = input[i][j].replace("  ",",")
#         input[i][j] = input[i][j].replace(" ", ",")
#     input[i] = "".join(input[i]).split(",")



print(input)