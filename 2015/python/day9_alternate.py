from __future__ import print_function

import os
import itertools
from functools import partial


def total_distance(location_mapping, route):
    return sum(map(lambda xy: location_mapping[xy[0]][xy[1]], zip(route, route[1:])))


def brute_force(location_mapping, selection_func):
    return selection_func(map(lambda route: (route, total_distance(location_mapping, route)), itertools.permutations(location_mapping.keys())))


def get_data():
    with open(os.path.join(os.path.dirname(__file__), '..', 'data', 'day9.txt')) as data_fp:
        return data_fp.readlines()


def parse_line(line):
    location_from, _, location_to, _, distance = line.split(' ')
    return location_from, location_to, distance


def load_data(data):
    location_mapping = {}

    for line in data:
        distance = parse_line(line)
        location_mapping.setdefault(distance[0], {})[distance[1]] = int(distance[2])
        location_mapping.setdefault(distance[1], {})[distance[0]] = int(distance[2])

    return location_mapping


def part_one(data):
    location_mapping = load_data(data)
    return brute_force(location_mapping, partial(min, key=lambda x: x[1]))


def part_two(data):
    location_mapping = load_data(data)
    return brute_force(location_mapping, partial(max, key=lambda x: x[1]))


def test_one():
    data = [
        'London to Dublin = 464',
        'London to Belfast = 518',
        'Dublin to Belfast = 141',
    ]

    return part_one(data)


def test_two():
    data = [
        'London to Dublin = 464',
        'London to Belfast = 518',
        'Dublin to Belfast = 141',
    ]

    return part_two(data)


def main():
    data = get_data()

    print('Test one solution:', test_one())
    print('Part one solution:', part_one(data))
    print('Test two solution:', test_two())
    print('Part two solution:', part_two(data))


if __name__ == '__main__':
    main()


