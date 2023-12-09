import argparse
from helpers import read_input, Node, node_cycle
from numba import njit


def part_1(instructions: str, node_map: dict) -> int:
    steps = 0
    n_instructions = len(instructions)
    node = node_map['AAA']
    while node.value != "ZZZ":
        instruction = instructions[steps % n_instructions]
        node = node_map[node.map(instruction)]
        steps += 1
    return steps


def part_2(instructions: str, node_map: dict) -> int:
    steps = 0
    n_instructions = len(instructions)
    nodes = [node for node in node_map.values() if node.value[-1] == "A"]
    while not all([node.value[-1] == "Z" for node in nodes]):
        instruction = instructions[steps % n_instructions]
        nodes = [node_map[node.map(instruction)] for node in nodes]
        steps += 1
    return steps


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--practice", action="store_true")
    args = parser.parse_args()
    filename = "data/practice_input.txt" if args.practice else "data/input.txt"
    puzzle_input = read_input(filename)
    instructions = puzzle_input[0]
    nodes = [Node(line) for line in puzzle_input[2:]]
    node_map = {node.value: node for node in nodes}

    print("Part 1:")
    print(part_1(instructions, node_map))

    print("Part 2:")
    print(part_2(instructions, node_map))


