import numpy as np


def parse_input(input):
    raw_input = open(input, "r").read().strip()
    separate_by_rows = raw_input.splitlines()
    separate_by_signs = list(map(lambda x: x.split(": "), separate_by_rows))
    parsed_input = []
    for [sensor, beacon] in separate_by_signs:
        sensor_coord = list(
            map(
                int,
                sensor.replace("Sensor at ", "")
                .replace("x=", "")
                .replace("y=", "")
                .split(", "),
            )
        )
        beacon_coord = list(
            map(
                int,
                beacon.replace("closest beacon is at ", "")
                .replace("x=", "")
                .replace("y=", "")
                .split(", "),
            )
        )
        parsed_input.append([sensor_coord, beacon_coord])
    return parsed_input


def merge_ranges(ranges):
    """
    Merges a /sorted/ list of ranges into a list of non-overlapping ranges.
    """
    merged_ranges = [ranges[0]]
    for i in range(1, len(ranges)):
        if ranges[i][0] <= merged_ranges[-1][1]:
            merged_ranges[-1][1] = max(merged_ranges[-1][1], ranges[i][1])
        else:
            merged_ranges.append(ranges[i])
    return merged_ranges


def part1(input):
    """
    For the given y coordinate, check how many positions are invalid given the sensor and beacon positions.
    """
    y = 2000000
    coords = parse_input(input)
    invalid_positions = set()

    # for each (sensor, beacon) pair, check how many beacon positions are invalid given the y coordinate
    for [sensor, beacon] in coords:
        # manhattan distance
        radius = abs(sensor[0] - beacon[0]) + abs(sensor[1] - beacon[1])

        # check if y is in range
        if abs(y - sensor[1]) <= radius:
            x_radius = abs(radius - abs(y - sensor[1]))
            for x in range(sensor[0] - x_radius, sensor[0] + x_radius + 1):
                invalid_positions.add((x, y))

    # remove existing sensors/beacons from the invalid positions
    for [sensor, beacon] in coords:
        invalid_positions.discard(tuple(sensor))
        invalid_positions.discard(tuple(beacon))

    return len(invalid_positions)


def part2(input):
    """
    For the given search space size, find the only valid position that is not covered by any sensor.
    """
    search_size = 4000000
    coords = parse_input(input)

    # iterate over the search space
    for y in range(search_size + 1):
        covered_ranges = []

        # for the current y, check which x ranges are covered by the sensors
        for [sensor, beacon] in coords:
            radius = abs(sensor[0] - beacon[0]) + abs(sensor[1] - beacon[1])
            if abs(y - sensor[1]) <= radius:
                x_radius = abs(radius - abs(y - sensor[1]))
                covered_ranges.append([sensor[0] - x_radius, sensor[0] + x_radius])
        covered_ranges.sort(key=lambda x: x[0])

        # merge the ranges
        merged_ranges = merge_ranges(covered_ranges)

        if len(merged_ranges) != 1:
            # find the gap, there should only be one
            print(covered_ranges, merged_ranges)
            x = merged_ranges[0][1] + 1
            return (x * 4000000) + y

    return -1


if __name__ == "__main__":
    input_filename = "input.txt"
    print("Part 1:", part1(input_filename))
    print("Part 2:", part2(input_filename))
