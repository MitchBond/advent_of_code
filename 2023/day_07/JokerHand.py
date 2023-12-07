from collections import Counter
from Hand import Hand

card_index = {
    x: i
    for i, x in enumerate(
        ["A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2", "1", "J"]
    )
}


class JokerHand:
    def __init__(self, line: str):
        split_line = line.split(" ")
        self.cards = [char for char in split_line[0]]
        self.non_joker_cards = [char for char in self.cards if char != "J"]
        self.bid = int(split_line[1])
        self.char_counts = list(Counter(self.non_joker_cards).values())
        self.char_indexes = [card_index[x] for x in self.cards]
        self.original_rank = Hand(line).hand_rank()

    def hand_rank(self) -> int:
        if len(self.non_joker_cards) == 5:
            return self.original_rank
        else:
            if len(self.char_counts) <= 1:
                return 1
            elif len(self.char_counts) == 2 and any([c == 1 for c in self.char_counts]):
                return 2
            elif len(self.char_counts) == 2:
                return 3
            elif len(self.char_counts) == 3:
                return 4
            elif len(self.char_counts) == 4:
                return 6
            else:
                return self.original_rank

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
