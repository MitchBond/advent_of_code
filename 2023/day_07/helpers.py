import re
import math


def read_input(filename: str):
    with open(filename, "r") as f:
        input = f.read()
    return input.split("\n")


def extract_ints_from_str(string: str) -> list[int]:
    return [int(x) for x in re.findall("(\\d+)", string)]
