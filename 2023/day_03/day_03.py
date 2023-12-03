import argparse
from helpers import parse_input
from Engine import Engine


def part_1(engine: Engine) -> float:
    return engine.sum_valid_parts()


def part_2(engine: Engine) -> float:
    return engine.gear_ratio()


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--practice", action="store_true")
    args = parser.parse_args()
    filename = "data/practice_input.txt" if args.practice else "data/input.txt"
    parsed_input = parse_input(filename)
    engine = Engine(parsed_input)

    print("Part 1:")
    print(part_1(engine))

    print("Part 2:")
    print(part_2(engine))
