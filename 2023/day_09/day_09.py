import argparse

from helpers import read_input, extract_ints_from_str


def extrapolate_next_value(seq: list[int]) -> int:
    final_diff = [seq[-1]]
    diff_all_zero = False
    while not diff_all_zero:
        seq = [a - b for a, b in zip(seq[1:], seq[:-1])]
        final_diff += [seq[-1]]
        diff_all_zero = all([x == 0 for x in seq])
    return sum(final_diff)


def part_1(sequences: list[int]) -> int:
    next_values = [extrapolate_next_value(seq) for seq in sequences]
    return sum(next_values)


def part_2(sequences: list[int]) -> int:
    next_values = [extrapolate_next_value(seq[::-1]) for seq in sequences]
    return sum(next_values)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--practice", action="store_true")
    args = parser.parse_args()
    filename = "data/practice_input.txt" if args.practice else "data/input.txt"
    puzzle_input = read_input(filename)
    sequences = [extract_ints_from_str(line) for line in puzzle_input]
    print("Part 1:")
    print(part_1(sequences))

    print("Part 2:")
    print(part_2(sequences))
