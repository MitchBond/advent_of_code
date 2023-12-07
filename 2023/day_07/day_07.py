import argparse

from Hand import Hand
from JokerHand import JokerHand
from helpers import read_input


def part_1(hands: list[Hand] | list[JokerHand]) -> int:
    sorted_hands = sorted(hands)
    return sum([(i + 1) * hand.bid for i, hand in enumerate(sorted_hands)])


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--practice", action="store_true")
    args = parser.parse_args()
    filename = "data/practice_input.txt" if args.practice else "data/input.txt"
    puzzle_input = read_input(filename)
    hands = [Hand(line) for line in puzzle_input]
    joker_hands = [JokerHand(line) for line in puzzle_input]
    print("Part 1:")
    print(part_1(hands))
    #
    print("Part 2:")
    print(part_1(joker_hands))
