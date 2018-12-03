from __future__ import print_function

import re
import os


VOWEL_RE = re.compile(r'[aeiou]')
REPEATED_RE = re.compile(r'(.)\1')
OFFENSIVE_RE = re.compile(r'(ab)|(cd)|(pq)|(xy)')


def is_nice(line):
    vowel_count = len(VOWEL_RE.findall(line))

    if vowel_count < 3:
        return False

    if not REPEATED_RE.search(line):
        return False

    if OFFENSIVE_RE.search(line):
        return False

    return True


PART2_REPEATED_RE = re.compile(r'(..).*\1')
PART2_SEPARATED_RE = re.compile(r'(.).\1')

def is_nice_part2(line):
    if not PART2_REPEATED_RE.search(line):
        return False

    if not PART2_SEPARATED_RE.search(line):
        return False

    return True


def get_data():
    with open(os.path.join(os.path.dirname(__file__), '..', 'data', 'day5.txt')) as data_fp:
        return data_fp.readlines()


def part_one(data):
    nice_strings = [line for line in data if is_nice(line)]
    return len(nice_strings)


def part_two(data):
    nice_strings = [line for line in data if is_nice_part2(line)]
    return len(nice_strings)


def main():
    data = get_data()

    print('Part one solution:', part_one(data))
    print('Part two solution:', part_two(data))


if __name__ == '__main__':
    main()



