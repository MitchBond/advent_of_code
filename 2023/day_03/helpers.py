def parse_input(filename: str):
    with open(filename, "r") as f:
        input = f.read()
    return input.split("\n")
