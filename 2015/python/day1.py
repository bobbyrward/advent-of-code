from __future__ import print_function

import re
import os


def get_data():
    with open(os.path.join(os.path.dirname(__file__), '..', 'data', 'day1.txt')) as data_fp:
        return data_fp.read()


def part_one(data):
    up_floors = len(re.findall(r'\(', data))
    down_floors = len(re.findall(r'\)', data))

    return up_floors - down_floors


def part_two(data):
    current_floor = 0
    direction_map = {'(': 1, ')': -1}

    for move_number, direction in enumerate(data):
        current_floor += direction_map[direction]

        if current_floor < 0:
            return move_number + 1

    raise Exception('Basement never reached')


def main():
    data = get_data()

    print('Part one solution:', part_one(data))
    print('Part two solution:', part_two(data))


if __name__ == '__main__':
    main()
