import re
from dataclasses import dataclass
from math import prod


@dataclass
class Part:
    number: int
    x_start: int
    x_end: int
    y: int


@dataclass
class SymbolPosition:
    symbol: str
    x: int
    y: int

    def is_adjacent_to(self, part: Part) -> bool:
        in_x_range = (self.x >= part.x_start - 1) and (self.x <= part.x_end + 1)
        in_y_range = abs(self.y - part.y) <= 1
        return in_x_range and in_y_range

    def gear_ratio(self, parts: list[Part]) -> float:
        adjacent_parts = [part for part in parts if self.is_adjacent_to(part)]
        return (
            prod([part.number for part in adjacent_parts])
            if len(adjacent_parts) == 2
            else 0
        )


class Engine:
    def __init__(self, parsed_input: list[str]):
        self.parts = self.__get_parts(parsed_input)
        self.symbols = self.__get_symbols(parsed_input)
        self.gears = [symbol for symbol in self.symbols if symbol.symbol == "*"]

    @staticmethod
    def __get_parts(parsed_input: list[str]) -> list[Part]:
        part_list = []
        for y, line in enumerate(parsed_input):
            match_iter = re.finditer("(\\d+)", line)
            part_list += [
                Part(
                    number=int(match.group()),
                    x_start=match.span()[0],
                    x_end=match.span()[1] - 1,
                    y=y,
                )
                for match in match_iter
            ]
        return part_list

    @staticmethod
    def __get_symbols(parsed_input: list[str]) -> list[SymbolPosition]:
        symbol_list = []
        for y, line in enumerate(parsed_input):
            match_iter = re.finditer("[\\#\\$\\%\\&\\*\\+\\-\\/\\=\\@]", line)
            symbol_list += [
                SymbolPosition(symbol=match.group(), x=match.span()[0], y=y)
                for match in match_iter
            ]
        return symbol_list

    def sum_valid_parts(self) -> int:
        return sum(
            [
                part.number
                for part in self.parts
                if any([symbol.is_adjacent_to(part) for symbol in self.symbols])
            ]
        )

    def gear_ratio(self) -> int:
        return sum([gear.gear_ratio(self.parts) for gear in self.gears])
