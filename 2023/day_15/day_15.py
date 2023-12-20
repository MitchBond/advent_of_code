import argparse
from helpers import read_input, Box


def hash_algo(string: str) -> int:
    ans = 0
    for c in string:
        ans += ord(c)
        ans *= 17
        ans = ans % 256
    return ans


def part_2(puzzle_input: str) -> int:
    boxes = [Box([]) for i in range(256)]
    for instruction in puzzle_input:
        if "-" in instruction:
            label = instruction.split("-")[0]
            box_number = hash_algo(label)
            boxes[box_number].minus(label)
        else:
            label, focal_length = instruction.split("=")
            box_number = hash_algo(label)
            boxes[box_number].add(label, int(focal_length))

    return sum([(i + 1) * box.score() for i, box in enumerate(boxes)])


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--practice", action="store_true")
    args = parser.parse_args()
    filename = "data/practice_input.txt" if args.practice else "data/input.txt"
    puzzle_input = read_input(filename).split(",")
    print("Part 1:")
    print(sum([hash_algo(s) for s in puzzle_input]))

    print("Part 2:")
    print(part_2(puzzle_input))


if __name__ == "__main__":
    main()
