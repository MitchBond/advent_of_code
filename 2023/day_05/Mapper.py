import re


class MapRange:
    def __init__(self, range_str: str):
        destination, source, length = self.__extract_ints_from_str(range_str)
        self.destination_start = destination
        self.source_start = source
        self.length = length

    @staticmethod
    def __extract_ints_from_str(string: str) -> list[int]:
        return [int(x) for x in re.findall("(\\d+)", string)]

    def map(self, x: int) -> int:
        if (x < self.source_start) or (x > self.source_start + self.length):
            return x
        else:
            return self.destination_start + (x - self.source_start)

    def inverse(self, x: int) -> int:
        if (x < self.destination_start) or (x > self.destination_start + self.length):
            return x
        else:
            return self.source_start + (x - self.destination_start)


class Mapper:
    def __init__(self, ranges: list[MapRange]):
        self.ranges = ranges

    def map(self, x: int) -> int:
        for map_range in self.ranges:
            new_x = map_range.map(x)
            if new_x != x:
                return new_x
        return x

    def inverse(self, x: int) -> int:
        for map_range in self.ranges:
            new_x = map_range.inverse(x)
            if new_x != x:
                return new_x
        return x
