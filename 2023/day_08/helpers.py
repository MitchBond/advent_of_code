import re


def read_input(filename: str):
    with open(filename, "r") as f:
        input = f.read()
    return input.split("\n")


class Node:
    def __init__(self, line: str):
        self.value = re.findall("(\\S+) = ", line)[0]
        self.left = re.findall("\\((\\S+),", line)[0]
        self.right = re.findall(", (\\S+)\\)", line)[0]

    def map(self, instruction: str) -> str:
        return self.left if instruction == "L" else self.right

    def __eq__(self, other):
        return self.value == other.value

    def __hash__(self):
        return hash(self.value)


def node_cycle(node_map: dict, instructions: str, start_node: Node) -> list[str]:
    i = 0
    n_instructions = len(instructions)
    nodes = [start_node.value]
    next_node = node_map[start_node.map(instructions[i % n_instructions])]
    while next_node != start_node or not next_node.z_steady_state():
        i += 1
        nodes += [next_node.value]
        next_node = node_map[start_node.map(instructions[i % n_instructions])]
        if next_node.value[-1] == "Z":
            print(f"{i}: {next_node.value}")
    return nodes


def execute_instructions(node: Node, node_map: dict, instructions: str) -> list[str]:
    nodes = []
    for i in instructions:
        node = node_map[node.map(i)]
        nodes += [node.value]
    return nodes
