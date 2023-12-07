import argparse
from helpers import read_input, extract_ints_from_str, number_of_winning_options
import math


def part_1(times: list, distances: list) -> int:
    x = [number_of_winning_options(t, d) for t, d in zip(times, distances)]
    return math.prod(x)


def part_2(times: list, distances: list) -> int:
    x = [number_of_winning_options(t, d) for t, d in zip(times, distances)]
    return math.prod(x)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--practice", action="store_true")
    args = parser.parse_args()
    filename = "data/practice_input.txt" if args.practice else "data/input.txt"
    puzzle_input = read_input(filename)
    times = extract_ints_from_str(puzzle_input[0])
    distances = extract_ints_from_str(puzzle_input[1])
    times_part2 = extract_ints_from_str(puzzle_input[0].replace(" ", ""))
    distances_part_2 = extract_ints_from_str(puzzle_input[1].replace(" ", ""))

    print("Part 1:")
    print(part_1(times, distances))

    print("Part 2:")
    print(part_2(times_part2, distances_part_2))
