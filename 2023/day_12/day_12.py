import argparse

from helpers import Record

from helpers import read_input


def part_1(puzzle_input: list[str]) -> int:
    records = []
    for line in puzzle_input:
        record, groups = line.split(" ")
        records += [Record(record, groups)]
    return sum([len(r.valid_candidates()) for r in records])
        
    
def part_2(puzzle_input: list[str]) -> int:
    records = []
    for line in puzzle_input:
        record, groups = line.split(" ")
        records += [Record("?".join(5 * [record]), ",".join(5 * [groups]))]
    return sum([len(r.valid_candidates()) for r in records])
    


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--practice", action="store_true")
    args = parser.parse_args()
    filename = "data/practice_input.txt" if args.practice else "data/input.txt"
    puzzle_input = read_input(filename)

    print("Part 1")
    print(part_1(puzzle_input))

    print("Part 2")
    print(part_2(puzzle_input))
    
