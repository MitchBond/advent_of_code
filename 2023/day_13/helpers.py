from collections import namedtuple
from dataclasses import dataclass
from collections import Counter


def read_input(filename: str):
    with open(filename, "r") as f:
        input = f.read()
    return input


Point = namedtuple("Point", ["x", "y"])


class Grid:
    def __init__(self, grid_str: str) -> None:
        lines = grid_str.split("\n")
        points = []
        for y, line in enumerate(lines):
            for x, char in enumerate(line):
                if char == ".":
                    points += [Point(x, y)]
        self.points = points
        self.max_y = len(lines) - 1
        self.max_x = len(lines[0]) - 1

    def non_symmetric_points_x(self, line_of_symmetry: float) -> list[int]:
        reflected_points = [
            Point(abs(line_of_symmetry - p.x), p.y)
            for p in self.points
            if 0 <= line_of_symmetry + (line_of_symmetry - p.x) <= self.max_x
        ]
        counts = Counter(reflected_points)
        return [point for point, count in dict(counts).items() if count == 1]

    def non_symmetric_points_y(self, line_of_symmetry: float) -> list[int]:
        reflected_points = [
            Point(p.x, abs(line_of_symmetry - p.y))
            for p in self.points
            if 0 <= line_of_symmetry + (line_of_symmetry - p.y) <= self.max_y
        ]
        counts = Counter(reflected_points)
        return [point for point, count in dict(counts).items() if count == 1]

    def score_p1(self) -> int:
        score = 0
        for x in range(self.max_x):
            if len(self.non_symmetric_points_x(x + 0.5)) == 0:
                score += x + 1
        for y in range(self.max_y):
            if len(self.non_symmetric_points_y(y + 0.5)) == 0:
                score += 100 * (y + 1)
        return score

    def score_p2(self) -> int:
        score = 0
        for x in range(self.max_x):
            if len(self.non_symmetric_points_x(x + 0.5)) == 1:
                score += x + 1
        for y in range(self.max_y):
            if len(self.non_symmetric_points_y(y + 0.5)) == 1:
                score += 100 * (y + 1)
        return score
