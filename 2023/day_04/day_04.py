import argparse
from helpers import read_input
from Scratchcard import Scratchcard


def part_1(scratchcards: list[Scratchcard]) -> int:
    return sum([scratchcard.points() for scratchcard in scratchcards])


def part_2(scratchcards: list[Scratchcard]) -> int:
    total_cards = {card.card_number: 1 for card in scratchcards}
    max_cards = len(scratchcards)
    for card in scratchcards:
        copies_of_card = total_cards[card.card_number]
        for copy_number in card.copies_won(max_cards):
            total_cards[copy_number] += copies_of_card
    return sum(total_cards.values())


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--practice", action="store_true")
    args = parser.parse_args()
    filename = "data/practice_input.txt" if args.practice else "data/input.txt"
    puzzle_input = read_input(filename)
    scratchcards = [Scratchcard(line) for line in puzzle_input]

    print("Part 1:")
    print(part_1(scratchcards))

    print("Part 2:")
    print(part_2(scratchcards))
