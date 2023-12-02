import argparse
from helpers import parse_input, Game


def part_1(games: list[Game]) -> float:
    return sum(
        [game.id for game in games if game.possible_with(red=12, green=13, blue=14)]
    )


def part_2(games: list[Game]) -> float:
    return sum([game.power() for game in games])


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--practice", action="store_true")
    args = parser.parse_args()
    filename = "data/practice_input.txt" if args.practice else "data/input.txt"
    parsed_input = parse_input(filename)
    games = [Game(game) for game in parsed_input]

    print("Part 1:")
    print(part_1(games))

    print("Part 2:")
    print(part_2(games))
