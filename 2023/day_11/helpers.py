from dataclasses import dataclass


def read_input(filename: str):
    with open(filename, "r") as f:
        input = f.read()
    return input.split("\n")


@dataclass
class Galaxy:
    x: int
    y: int

    def distance(self, other) -> int:
        return abs(self.x - other.x) + abs(self.y - other.y)


def get_galaxies(puzzle_input: list[str]) -> list[Galaxy]:
    galaxies = []
    for y, line in enumerate(puzzle_input):
        for x, char in enumerate(line):
            if char == "#":
                galaxies += [Galaxy(x=x, y=y)]
    return galaxies


def get_empty_rows(puzzle_input: list[str]) -> list[int]:
    empties = []
    for y, line in enumerate(puzzle_input):
        if all([char == "." for char in line]):
            empties += [y]
    return empties


def get_empty_cols(puzzle_input: list[str]) -> list[int]:
    empties = []
    for x in range(len(puzzle_input[0])):
        if all([puzzle_input[y][x] == "." for y in range(len(puzzle_input))]):
            empties += [x]
    return empties


def adjust_galaxies(
    galaxies: list[Galaxy],
    empty_rows: list[int],
    empty_cols: list[int],
    offset: int = 1,
) -> list[Galaxy]:
    adjusted_galaxies = []
    for galaxy in galaxies:
        x_delta = offset * sum([galaxy.x > col for col in empty_cols])
        y_delta = offset * sum([galaxy.y > row for row in empty_rows])
        adjusted_galaxies += [Galaxy(galaxy.x + x_delta, galaxy.y + y_delta)]

    return adjusted_galaxies


def sum_pairwise_distance(galaxies: list[Galaxy]) -> int:
    ans = 0
    for i in range(len(galaxies)):
        for j in range(i, len(galaxies)):
            ans += galaxies[i].distance(galaxies[j])

    return ans
