import re


class Scratchcard:
    def __init__(self, input_line: str):
        self.card_number = int(re.findall(" (\\d+):", input_line)[0])
        card_str = input_line.split(":")[1]
        winning_numbers_str, numbers_str = card_str.split("|")
        self.winning_numbers = self.__extract_ints_from_str(winning_numbers_str)
        self.numbers = self.__extract_ints_from_str(numbers_str)
        self.matching_numbers = set(self.winning_numbers).intersection(self.numbers)
        self.total_matching_numbers = len(self.matching_numbers)

    @staticmethod
    def __extract_ints_from_str(string: str) -> list[int]:
        return [int(x) for x in re.findall("(\\d+)", string)]

    def points(self) -> int:
        return (
            2 ** (self.total_matching_numbers - 1)
            if self.total_matching_numbers > 0
            else 0
        )

    def copies_won(self, max_cards: int) -> list[int]:
        return list(
            range(
                self.card_number + 1,
                min(self.card_number + self.total_matching_numbers + 1, max_cards + 1),
            )
        )
