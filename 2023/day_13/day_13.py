import argparse

from helpers import read_input, Grid


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--practice", action="store_true")
    args = parser.parse_args()
    filename = "data/practice_input.txt" if args.practice else "data/input.txt"
    puzzle_input = read_input(filename)
    grids = [Grid(grid_str) for grid_str in puzzle_input.split("\n\n")]
    g = grids[1]

    print("Part 1:")
    print(sum([g.score_p1() for g in grids]))

    print("Part 2:")
    print(sum([g.score_p2() for g in grids]))

    # print("Part 1:")
    # print(part_1(galaxies=galaxies, empty_rows=empty_rows, empty_cols=empty_cols))

    # print("Part 2:")
    # print(part_2(galaxies=galaxies, empty_rows=empty_rows, empty_cols=empty_cols))
