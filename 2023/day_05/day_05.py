import argparse
from helpers import read_input, parse_input, mapped_location, inverse_mapped_location
from Mapper import Mapper
from math import prod


def part_1(seeds: list[int], maps: list[Mapper]) -> int:
    seed_locations = [mapped_location(seed, maps) for seed in seeds]
    return min(seed_locations)


def part_2(seeds: list[int], maps: list[Mapper]) -> int:
    seed_ranges = []
    for i in range(len(seeds) // 2):
        seed_ranges += [(seeds[2 * i], seeds[2 * i] + seeds[2 * i + 1])]

    local_minima = []
    for map in maps[::-1]:
        local_minima = [map.inverse(x) for x in local_minima]
        local_minima += [map_range.source_start for map_range in map.ranges]
    local_minima += [seed_range[0] for seed_range in seed_ranges]
    valid_minima = [
        x
        for x in local_minima
        if any([start <= x <= end for start, end in seed_ranges])
    ]
    return min([mapped_location(x, maps) for x in valid_minima])


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--practice", action="store_true")
    args = parser.parse_args()
    filename = "data/practice_input.txt" if args.practice else "data/input.txt"
    puzzle_input = read_input(filename)
    seeds, maps = parse_input(puzzle_input)

    print("Part 1:")
    print(part_1(seeds, maps))

    print("Part 2:")
    print(part_2(seeds, maps))
