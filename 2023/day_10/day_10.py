import argparse

from helpers import read_input


def get_neighbours(x: int, y: int, char: str) -> list[tuple]:
    match char:
        case "|":
            return [(x, y + 1), (x, y - 1)]
        case "-":
            return [(x + 1, y), (x - 1, y)]
        case "L":
            return [(x + 1, y), (x, y + 1)]
        case "J":
            return [(x - 1, y), (x, y + 1)]
        case "7":
            return [(x - 1, y), (x, y - 1)]
        case "F":
            return [(x + 1, y), (x, y - 1)]
        case ".":
            return []


def find_S_location(input: list[str]) -> (int, int):
    y = [y for y, line in enumerate(input) if "S" in line][0]
    x = [x for x, char in enumerate(input[y]) if char == "S"][0]
    return x, y


def map_S_position(input: list[str], s_location: (int , int)) -> list[str]:
    x = s_location[0]
    y = s_location[1]
    from_north = input[y-1][x] in ["|", "7", "F"]
    from_east = input[y][x+1] in ["-", "J", "7"]
    from_south = input[y+1][x] in ["|", "J", "L"]
    from_west = input[y][x-1] in ["-", "L", "F"]
    print([from_north, from_east, from_south, from_west])
    if from_north:
        if from_east:
            s = "L"
        elif from_south:
            s = "|"
        elif from_west:
            s = "J"
    elif from_east:
        if from_south:
            s = "F"
        if from_west:
            s = "-"
    elif from_south & from_west:
        s = "7"
    input[x][y] = s
    return input


def process_input(input: list[str]) -> dict:
    location_map = {}
    for x, line in enumerate(input):
        for y, char in enumerate(line):
            location_map[(x, y)] = get_neighbours(x, y, char)
    return location_map


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--practice", action="store_true")
    args = parser.parse_args()
    filename = "data/practice_input.txt" if args.practice else "data/input.txt"
    puzzle_input = [[char for char in line] for line in read_input(filename)]
    print(puzzle_input)
    s_location = find_S_location(puzzle_input)
    puzzle_input = map_S_position(puzzle_input, s_location)
    neighbour_map = process_input(puzzle_input)
    print(puzzle_input)
    print(s_location)
    print(neighbour_map)
