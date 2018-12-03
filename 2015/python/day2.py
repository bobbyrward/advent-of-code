from __future__ import print_function

import os
from collections import namedtuple


class Box(namedtuple('Box', ['l', 'w', 'h'])):
    __slots__ = ()

    @property
    def surface_area(self):
        return (
            2 * self.l * self.w +
            2 * self.w * self.h +
            2 * self.h * self.l
        )

    @property
    def smallest_side(self):
        return list(sorted(self))[:2]

    @property
    def slack_needed(self):
        smallest_side = self.smallest_side
        return smallest_side[0] * smallest_side[1]

    @property
    def shortest_distance(self):
        smallest_side = self.smallest_side
        return sum(smallest_side * 2)

    @property
    def volume(self):
        return self.l * self.w * self.h


def get_data():
    with open(os.path.join(os.path.dirname(__file__), '..', 'data', 'day2.txt')) as data_fp:
        for line in data_fp:
            yield Box(*map(int, line.split('x')))


def required_paper(box):
    return box.surface_area + box.slack_needed


def part_one(data):
    return sum(map(required_paper, data))


def required_ribbon(box):
    return box.shortest_distance + box.volume


def part_two(data):
    return sum(map(required_ribbon, data))


def main():
    data = list(get_data())

    print('Part one solution:', part_one(data))
    print('Part two solution:', part_two(data))


if __name__ == '__main__':
    main()

