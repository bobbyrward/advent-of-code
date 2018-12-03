from __future__ import print_function

import itertools
import os
from collections import namedtuple


DIRECTION_MAP = {
    '<': (-1, 0),
    '>': (1, 0),
    '^': (0, -1),
    'v': (0, 1),
}


class Position(namedtuple('Position', ['x', 'y'])):
    __slots__ = ()

    def move_direction(self, direction_character):
        movement = DIRECTION_MAP[direction_character]

        return Position(
            self.x + movement[0],
            self.y + movement[1],
        )


def get_data():
    with open(os.path.join(os.path.dirname(__file__), '..', 'data', 'day3.txt')) as data_fp:
        return data_fp.read()


def next_move(visited, current_pos, direction):
    new_pos = current_pos.move_direction(direction)
    visited.add(new_pos)
    return new_pos


def part_one(data):
    visited = set()

    current_pos = Position(0,0)
    visited.add(current_pos)

    for move in data:
        current_pos = next_move(visited, current_pos, move)

    return len(visited)


def part_two(data):
    visited = set()

    santa_pos = Position(0,0)
    robo_pos = Position(0,0)
    visited.add(santa_pos)

    def robo_iterator(moves):
        santa = itertools.islice(iter(moves), 0, None, 2)
        robo = itertools.islice(iter(moves), 1, None, 2)

        return itertools.izip(santa, robo)

    for santa_move, robo_move in robo_iterator(data):
        santa_pos = next_move(visited, santa_pos, santa_move)
        robo_pos = next_move(visited, robo_pos, robo_move)

    return len(visited)


def main():
    data = get_data()

    print('Part one solution:', part_one(data))
    print('Part two solution:', part_two(data))


if __name__ == '__main__':
    main()


