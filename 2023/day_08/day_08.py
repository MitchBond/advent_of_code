import argparse
from helpers import read_input, Node, node_cycle, execute_instructions


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
    instructions = 50 * instructions
    n_instructions = len(instructions)
    nodes = [node.value for node in node_map.values() if node.value[-1] == "A"]
    node_summary = {}
    for node in node_map.values():
        path = execute_instructions(node, node_map, instructions)
        node_summary[node.value] = (path[-1], [i + 1 for i, node in enumerate(path) if node[-1] == "Z"])


    end = False
    while not end:
        vals = [node_summary[node] for node in nodes]
        nodes = [v[0] for v in vals]
        matches = [v[1] for v in vals]
        if all([len(m) > 0 for m in matches]):
            common_vals = set.intersection(*map(set, matches))

            if len(common_vals) > 0:
                return steps * n_instructions + min(common_vals)
        if steps % 100000 == 0:
            print(steps * n_instructions)
        steps += 1
    return None


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--practice", action="store_true")
    args = parser.parse_args()
    filename = "data/practice_input.txt" if args.practice else "data/input.txt"
    puzzle_input = read_input(filename)
    instructions = puzzle_input[0]
    nodes = [Node(line) for line in puzzle_input[2:]]
    node_map = {node.value: node for node in nodes}
    #
    # print("Part 1:")
    # print(part_1(instructions, node_map))

    print("Part 2:")
    print(part_2(instructions, node_map))


