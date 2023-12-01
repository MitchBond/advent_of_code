def sort_matches(matches: list[tuple[str, int]]) -> list[tuple[str, int]]:
    return sorted([match for match in matches if match[1] > -1], key=lambda x: x[1])


def get_left_most_match(string: str, substr_list: list) -> str:
    matches = [(substr, string.find(substr)) for substr in substr_list]
    sorted_matches = sort_matches(matches)
    return sorted_matches[0][0]


def get_right_most_match(string: str, substr_list: list) -> str:
    matches = [(substr, string.rfind(substr)) for substr in substr_list]
    sorted_matches = sort_matches(matches)
    return sorted_matches[-1][0]


def get_first_and_last_match(string: str, substr_list: list[str]) -> tuple[str, str]:
    first_match = get_left_most_match(string, substr_list)
    last_match = get_right_most_match(string, substr_list)

    return first_match, last_match
