from dataclasses import dataclass


def read_input(filename: str):
    with open(filename, "r") as f:
        input = f.read()
    return input


@dataclass
class Lens:
    label: str
    focal_length: int


@dataclass
class Box:
    lenses: list[Lens]

    def add(self, label: str, focal_length: int) -> None:
        for lens in self.lenses:
            if lens.label == label:
                lens.focal_length = focal_length
                return None
        self.lenses += [Lens(label, focal_length)]

    def minus(self, label: str) -> None:
        self.lenses = [lens for lens in self.lenses if lens.label != label]

    def score(self) -> int:
        return sum([(i + 1) * lens.focal_length for i, lens in enumerate(self.lenses)])
