input = list(map(lambda x: x, open("input_3.txt", "r").read().split("\n")))


def more(column):
    if column.count("1") > column.count("0"):
        return "1"
    elif column.count("1") < column.count("0"):
        return "0"
    elif column.count("1") == column.count("0"):
        return "1"


def less(column):
    if column.count("1") > column.count("0"):
        return "0"
    elif column.count("1") < column.count("0"):
        return "1"
    elif column.count("1") == column.count("0"):
        return "0"



def day_3_2(input):
    arr = input.copy()
    oxy = ""
    co2 = ""
    x = []
    for i in range(len(input[0])):
        if len(arr) == 1:
            oxy = arr[0]
            break
        for j in range(len(arr)):
            x.append(arr[j][i])
        often = more(x)
        to_remove = []
        for j in arr:
            if j[i] != often:
                to_remove.append(j)
        for j in to_remove:
            arr.remove(j)
        to_remove = []
        oxy += often
        x = []

    return oxy

def dupa(input):
    arr = input.copy()
    co2 = ""
    x = []
    for i in range(len(input[0])):
        if len(arr) == 1:
            co2 = arr[0]
            break
        for j in range(len(arr)):
            x.append(arr[j][i])
        less_often = less(x)
        to_remove = []
        for j in arr:
            if j[i] != less_often:
                to_remove.append(j)
        for j in to_remove:
            arr.remove(j)
        to_remove = []
        co2 += less_often
        x = []

    return co2


print(int(day_3_2(input), 2) * int(dupa(input), 2))
# print(day_3_2(input))
# print(dupa(input))