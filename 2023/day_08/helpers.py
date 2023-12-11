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


def find_first_z_node(node: Node, node_map: dict, instructions: str) -> int:
    steps = 0
    n_instructions = len(instructions)
    index_of_z = []
    while len(index_of_z) == 0:
        node = node_map[node.map(instructions[steps % n_instructions])]
        steps += 1
        if node.value[-1] == "Z":
            return steps
