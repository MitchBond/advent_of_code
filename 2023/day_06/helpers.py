import re
import math


def read_input(filename: str):
    with open(filename, "r") as f:
        input = f.read()
    return input.split("\n")


def extract_ints_from_str(string: str) -> list[int]:
    return [int(x) for x in re.findall("(\\d+)", string)]


def min_time_held(time: int, distance: int) -> int:
    a = 1
    b = -time
    c = distance
    return (-b - (b**2 - 4 * a * c) ** 0.5) / 2 * a


def max_time_held(time: int, distance: int) -> int:
    a = 1
    b = -time
    c = distance
    return (-b + (b**2 - 4 * a * c) ** 0.5) / 2 * a


def number_of_winning_options(time: int, distance: int) -> int:
    return (
        math.ceil(max_time_held(time, distance) - 1)
        - math.floor(min_time_held(time, distance) + 1)
        + 1
    )
