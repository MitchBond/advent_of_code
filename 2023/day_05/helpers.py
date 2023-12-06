import re
from Mapper import MapRange, Mapper


def read_input(filename: str):
    with open(filename, "r") as f:
        input = f.read()
    return input


def process_map(map: str) -> Mapper:
    return Mapper([MapRange(range_str) for range_str in map.split("\n")])


def extract_maps(map_input: str) -> list[str]:
    maps = map_input.split("\n\n")
    maps = [map.split(":\n")[1] for map in maps]
    return maps


def parse_input(input: str) -> (list[int], list[Mapper]):
    seed_input, map_input = input.split("\n", 1)
    seeds = [int(x) for x in re.findall("(\\d+)", seed_input)]
    maps = extract_maps(map_input)
    maps = [process_map(map) for map in maps]

    return seeds, maps


def mapped_location(x: int, maps: list[Mapper]) -> int:
    for map in maps:
        x = map.map(x)
    return x


def inverse_mapped_location(x: int, maps: list[Mapper]) -> int:
    for map in maps[::-1]:
        x = map.inverse(x)
    return x

