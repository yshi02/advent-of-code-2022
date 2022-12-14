def parse_input(input):
    return list(map(str.splitlines, open(input).read().strip().split("\n\n")))


def is_valid(left, right):
    # if one of the <left, right> pair is an int
    if type(left) == int:
        if type(right) == int:
            return left - right
        else:
            return is_valid([left], right)
    else:
        if type(right) == int:
            return is_valid(left, [right])

    # if both are lists, compare each element
    for l, r in zip(left, right):
        res = is_valid(l, r)
        if res != 0:
            return res

    # if still no decision can be made, compare the length
    return len(left) - len(right)


def part1(input):
    input = parse_input(input)

    sum_indices = 0
    for index, (left, right) in enumerate(input):
        if is_valid(eval(left), eval(right)) < 0:
            sum_indices += index + 1

    return sum_indices


def part2(input):
    input = parse_input(input)

    index_2 = 1
    index_6 = 2

    for _, value in enumerate(input):
        for signal in value:
            if is_valid(eval(signal), [[2]]) < 0:
                index_2 += 1
            if is_valid(eval(signal), [[6]]) < 0:
                index_6 += 1

    return index_2 * index_6


if __name__ == "__main__":
    input_filename = "input.txt"
    print("Part 1:", part1(input_filename))
    print("Part 2:", part2(input_filename))
