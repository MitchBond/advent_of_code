import argparse

from helpers import (
    read_input,
    Galaxy,
    get_galaxies,
    adjust_galaxies,
    get_empty_rows,
    get_empty_cols,
    sum_pairwise_distance,
)


def part_1(galaxies: list[Galaxy], empty_rows: list[int], empty_cols: list[int]) -> int:
    adjusted_galaxies = adjust_galaxies(galaxies, empty_rows, empty_cols, offset=1)
    return sum_pairwise_distance(adjusted_galaxies)


def part_2(galaxies: list[Galaxy], empty_rows: list[int], empty_cols: list[int]) -> int:
    adjusted_galaxies = adjust_galaxies(
        galaxies, empty_rows, empty_cols, offset=1000000 - 1
    )
    return sum_pairwise_distance(adjusted_galaxies)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--practice", action="store_true")
    args = parser.parse_args()
    filename = "data/practice_input.txt" if args.practice else "data/input.txt"
    puzzle_input = read_input(filename)
    galaxies = get_galaxies(puzzle_input)
    empty_rows = get_empty_rows(puzzle_input)
    empty_cols = get_empty_cols(puzzle_input)

    print("Part 1:")
    print(part_1(galaxies=galaxies, empty_rows=empty_rows, empty_cols=empty_cols))

    print("Part 2:")
    print(part_2(galaxies=galaxies, empty_rows=empty_rows, empty_cols=empty_cols))
