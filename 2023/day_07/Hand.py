from collections import Counter

card_index = {
    x: i
    for i, x in enumerate(
        ["A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2", "1"]
    )
}


class Hand:
    def __init__(self, line: str):
        split_line = line.split(" ")
        self.cards = [char for char in split_line[0]]
        self.bid = int(split_line[1])
        self.char_counts = list(Counter(self.cards).values())
        self.char_indexes = [card_index[x] for x in self.cards]

    def hand_rank(self) -> int:
        if len(self.char_counts) == 1:
            return 1
        elif 4 in self.char_counts:
            return 2
        elif len(self.char_counts) == 2:
            return 3
        elif 3 in self.char_counts:
            return 4
        elif sum([x == 2 for x in self.char_counts]) == 2:
            return 5
        elif len(self.char_counts) == 4:
            return 6
        else:
            return 7

    def tie_break(self, other) -> bool:
        first_different_card = [
            x - y for x, y in zip(self.char_indexes, other.char_indexes) if x - y != 0
        ][0]
        return first_different_card > 0

    def __lt__(self, other) -> bool:
        return (
            self.hand_rank() > other.hand_rank()
            if self.hand_rank() != other.hand_rank()
            else self.tie_break(other)
        )
