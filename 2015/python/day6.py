from __future__ import print_function

import itertools
import os
import re


LINE_RE = re.compile('^(toggle|turn on|turn off) (\d+),(\d+) through (\d+),(\d+)$')


class LightGrid(object):
    def __init__(self, width, height, initial_value):
        self.width = width
        self.height = height
        self.lights = [initial_value] * self.size

    @property
    def size(self):
        return self.width * self.height

    def get_sum(self):
        return sum(self.lights)

    def get_enabled_count(self):
        return sum(1 for _ in itertools.ifilter(lambda x: x, self.lights))

    def process_lights_in_row(self, row, start, stop, affect_function):
        light_start = row * self.width + start
        light_stop = row * self.width + stop

        self.lights[light_start:light_stop] = itertools.imap(
            affect_function, self.lights[light_start:light_stop]
        )

    def print_grid(self):
        for y in range(self.height):
            row = self.lights[y * self.width:(y + 1) *self.width]
            print(''.join('*' if light else '-' for light in row))

    def process_command(self, start_coord, stop_coord, affect_function):
        for row in range(start_coord[1], stop_coord[1] + 1):
            self.process_lights_in_row(row, start_coord[0], stop_coord[0]+1, affect_function)


def get_data():
    with open(os.path.join(os.path.dirname(__file__), '..', 'data', 'day6.txt')) as data_fp:
        return data_fp.readlines()


def parse_line(line, functions):
    match = LINE_RE.match(line)

    if not match:
        raise Exception('Line does not match: "{}"'.format(line))

    groups = match.groups()

    return {
        'affect_function': functions[groups[0]],
        'start_coord': (int(groups[1]), int(groups[2])),
        'stop_coord': (int(groups[3]), int(groups[4])),
    }


def part_one(data):
    functions = {
        'toggle': lambda x: not x,
        'turn on': lambda x: True,
        'turn off': lambda x: False,
    }

    grid = LightGrid(1000, 1000, False)

    for line in data:
        command = parse_line(line, functions)
        grid.process_command(**command)

    return grid.get_enabled_count()


def part_two(data):
    functions = {
        'toggle': lambda x: x+2,
        'turn on': lambda x: x+1,
        'turn off': lambda x: max(0, x-1),
    }

    grid = LightGrid(1000, 1000, 0)

    for line in data:
        command = parse_line(line, functions)
        grid.process_command(**command)

    return grid.get_sum()


def main():
    data = get_data()

    print('Part one solution:', part_one(data))
    print('Part two solution:', part_two(data))


if __name__ == '__main__':
    main()



