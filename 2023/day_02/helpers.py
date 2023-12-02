import re


class Game:
    def __init__(self, game_str: str):
        game = game_str.split(": ")[-1]
        observations = game.split(";")
        self.id = self.__get_id(game_str)
        self.observations = [
            Observation(observation_str) for observation_str in observations
        ]
        self.max_red = max([obs.red for obs in self.observations])
        self.max_green = max([obs.green for obs in self.observations])
        self.max_blue = max([obs.blue for obs in self.observations])

    @staticmethod
    def __get_id(game_str) -> int:
        pattern = "Game (\\d+):"
        return int(re.findall(pattern, game_str)[0])

    def possible_with(self, red: int, green: int, blue: int) -> bool:
        reds_below_max = self.max_red <= red
        greens_below_max = self.max_green <= green
        blues_below_max = self.max_blue <= blue
        return reds_below_max and greens_below_max and blues_below_max

    def power(self) -> int:
        return self.max_red * self.max_blue * self.max_green


class Observation:
    def __init__(self, observation_str: str):
        self.red: int = self.__colour_number(observation_str, colour="red")
        self.green: int = self.__colour_number(observation_str, colour="green")
        self.blue: int = self.__colour_number(observation_str, colour="blue")

    @staticmethod
    def __colour_number(observation_str: str, colour: str) -> int:
        pattern = f"(\\d+) {colour}"
        matches = re.findall(pattern, observation_str)
        return int(matches[0]) if len(matches) > 0 else 0


def parse_input(filename: str):
    with open(filename, "r") as f:
        input = f.read()
    return input.split("\n")
