import numpy as np


def parse_input(input):
    raw_input = open(input, "r").read().strip()
    separate_by_rows = raw_input.splitlines()
    separate_by_signs = list(map(lambda x: x.split(" -> "), separate_by_rows))
    parsed_input = []
    for row in separate_by_signs:
        current_row = []
        for pair in row:
            x, y = pair.split(",")
            current_row.append([int(x), int(y)])
        parsed_input.append(current_row)
    return parsed_input


def place_rocks(rocks):
    """
    place the rocks on the map and return the map
    """
    edge_length = 1000

    # create a map
    map = [["." for _ in range(edge_length)] for _ in range(edge_length)]

    # place the sand source
    map[0][500] = "+"

    # place the rocks
    for row in rocks:
        for i in range(len(row) - 1):
            curr = row[i]
            next = row[i + 1]

            # find the direction and normalize it
            dx = (
                0
                if next[0] - curr[0] == 0
                else (next[0] - curr[0]) // abs(next[0] - curr[0])
            )
            dy = (
                0
                if next[1] - curr[1] == 0
                else (next[1] - curr[1]) // abs(next[1] - curr[1])
            )
            direction = [dx, dy]

            # place the rocks between and including the current and next rock
            while curr != next:
                map[curr[1]][curr[0]] = "#"
                curr[0] += direction[0]
                curr[1] += direction[1]

            map[curr[1]][curr[0]] = "#"

    # np.savetxt("map.txt", map, fmt="%s", delimiter="")

    return map


def drop_sand(map):
    """
    drop a sand drop and return the map and if the sand drop leaked
    """

    sand_source = [500, 0]
    leaked = False

    # generate a sand drop
    sand = [sand_source[0], sand_source[1]]

    # let the sand drop fall
    sand_stopped = False
    while not sand_stopped:
        # check if the sand drop could fall further
        if map[sand[1] + 1][sand[0]] == ".":
            sand[1] += 1

        # check if the sand drop could fall to the sides if it can't fall further
        elif map[sand[1] + 1][sand[0] - 1] == ".":
            sand[0] -= 1
            sand[1] += 1

        elif map[sand[1] + 1][sand[0] + 1] == ".":
            sand[0] += 1
            sand[1] += 1

        # check if the sand drop could stop
        else:
            sand_stopped = True
            break

        # check if the sand drop leaked
        if all(map[i][sand[0]] == "." for i in range(sand[1] + 1, len(map))):
            leaked = True
            break

    if not leaked:
        map[sand[1]][sand[0]] = "o"

    return map, leaked


def part1(input):
    """
    return the max number of sand drops that the rocks can hold
    """
    rocks = parse_input(input)
    map = place_rocks(rocks)

    sand_count = 0
    while True:
        map, leaked = drop_sand(map)
        if leaked:
            break
        sand_count += 1

    np.savetxt("map_part1.txt", map, fmt="%s", delimiter="")
    return sand_count


def part2(input):
    """
    return the number of sand drops needed to make the sand pile reach the sand source
    """
    rocks = parse_input(input)
    map = place_rocks(rocks)

    # add the floor to the map
    highest_y = 0
    for i in range(len(map) - 1, 0, -1):
        if "#" in map[i]:
            highest_y = i
            break
    for i in range(len(map[highest_y + 2])):
        map[highest_y + 2][i] = "="

    sand_count = 0
    while True:
        map, _ = drop_sand(map)
        sand_count += 1
        if map[0][500] == "o":
            break

    np.savetxt("map_part2.txt", map, fmt="%s", delimiter="")
    return sand_count


if __name__ == "__main__":
    input_filename = "input.txt"
    print("Part 1:", part1(input_filename))
    print("Part 2:", part2(input_filename))
