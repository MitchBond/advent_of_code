import re


def read_input(filename: str):
    with open(filename, "r") as f:
        input = f.read()
    return input.split("\n")


class Record:
    def __init__(self, record: str, groups: str) -> None:
        self.record = record
        self.groups = [int(x) for x in groups.split(",")]
        self.free_values = len(self.record) - (sum(self.groups) + len(self.groups) - 1)
        self.bins = len(self.groups) + 1
        self.dot_locations = [m.start() for m in re.finditer("\\.", self.record)]
        self.hash_locations = [m.start() for m in re.finditer("\\#", self.record)]
        self.hashes_needed = sum(self.groups) - len(self.hash_locations)

    def generate_candidates(self) -> list[str]:

        candidates = [[x] for x in range(self.free_values + 1)]

        for _ in range(self.bins-2):
            new_candidates = []
            for c in candidates:
                total = sum(c)
                for i in range(self.free_values - total + 1):
                    new_candidates += [c + [i]]
            candidates = new_candidates
        new_candidates = []
        for c in candidates:
            total = sum(c)
            new_candidates += [c + [self.free_values - total]]
        candidate_strs = []
        for c in new_candidates:
            string = "." * c[0]
            string += "".join(["#" * g + "." + "." * d for g, d in zip(self.groups[:-1], c[1:-1])])
            string += "#" * self.groups[-1] + "." * c[-1]
            candidate_strs += [string]

        return candidate_strs
    
    def valid_candidates(self) -> list[str]:
        candidates = self.generate_candidates()
        return [c for c in candidates if self.candidate_matches(c)]


    def candidate_matches(self, candidate: str) -> bool:
        candidate_dots = [m.start() for m in re.finditer("\\.", candidate)]
        candidate_hash = [m.start() for m in re.finditer("\\#", candidate)]

        matches_dots = all([x in candidate_dots for x in self.dot_locations])
        matches_hash = all([x in candidate_hash for x in self.hash_locations])
        return matches_dots & matches_hash
    