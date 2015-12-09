from __future__ import print_function

import os
from functools import partial


def total_distance(location_mapping, route):
    distance = 0

    for idx in range(len(route)-1):
        distance += location_mapping[route[idx]][route[idx+1]]

    return distance



def brute_force_inner(location_mapping, route, remaining_locations, selection_func):
    # print("brute_force_inner({}, {})".format(route, remaining_locations))

    current_location = route[-1]

    if len(remaining_locations) == 1:
        destination = tuple(remaining_locations)[0]
        new_route = route + [destination]
        distance = total_distance(location_mapping, new_route)
        # print('Only {} is left, making total {}'.format(destination, distance))
        return route + [destination], distance

    sub_routes = []

    for location in remaining_locations:
        sub_route, distance = brute_force_inner(
            location_mapping,
            route + [location],
            remaining_locations.difference(set([location])),
            selection_func,
        )

        sub_routes.append((sub_route, distance))

    preferred_route = selection_func(sub_routes)

    # print('{} is shortest. making total {}'.format(preferred_route[0], preferred_route[1]))

    return preferred_route[0], preferred_route[1]


def brute_force(location_mapping, selection_func):
    all_locations = set(location_mapping.keys())
    all_routes = []

    for origin in all_locations:
        all_routes.append(brute_force_inner(
            location_mapping,
            [origin],
            all_locations.difference(set([origin])),
            selection_func,
        ))

    # for route in all_routes:
    #     print(route)

    return selection_func(all_routes)


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

