import argparse

from helpers import get_first_and_last_match


def parse_input(filename: str):
    with open(filename, "r") as f:
        input = f.read()
    return input.split("\n")


def part_1(parsed_input: list[str]) -> float:
    substring_list = [str(x) for x in range(1, 10)]
    matches = [get_first_and_last_match(line, substring_list) for line in parsed_input]
    return sum([10 * int(match[0]) + int(match[1]) for match in matches])


def part_2(parsed_input: list[str]) -> float:
    substring_to_int_map = {
        "one": 1,
        "two": 2,
        "three": 3,
        "four": 4,
        "five": 5,
        "six": 6,
        "seven": 7,
        "eight": 8,
        "nine": 9,
        **{str(x): x for x in range(1, 10)},
    }
    matches = [
        get_first_and_last_match(line, list(substring_to_int_map.keys()))
        for line in parsed_input
    ]
    return sum(
        [
            10 * substring_to_int_map[match[0]] + substring_to_int_map[match[1]]
            for match in matches
        ]
    )


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--practice", action="store_true")
    args = parser.parse_args()
    filename = "data/practice_input.txt" if args.practice else "data/input.txt"
    parsed_input = parse_input(filename)
    print("Part 1:")
    print(part_1(parsed_input))

    print("Part 2:")
    print(part_2(parsed_input))
