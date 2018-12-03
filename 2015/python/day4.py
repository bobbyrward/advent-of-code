from __future__ import print_function

import itertools
import os

from collections import namedtuple
from hashlib import md5


class Miner(object):
    def __init__(self, secret):
        self.secret = secret

    def hash_for_value(self, solution):
        return md5(self.secret + str(solution)).hexdigest()

    def _is_valid_hash(self, zero_length, solution):
        return self.hash_for_value(solution)[:zero_length] == '0' * zero_length

    def has_five_zeroes(self, solution):
        return self._is_valid_hash(5, solution)

    def has_six_zeroes(self, solution):
        return self._is_valid_hash(6, solution)


def get_data():
    with open(os.path.join(os.path.dirname(__file__), '..', 'data', 'day4.txt')) as data_fp:
        return data_fp.read().strip()


def part_one(data):
    miner = Miner(data)

    for possible_solution in itertools.count():
        if miner.has_five_zeroes(possible_solution):
            return possible_solution

    return Exception('No solution')


def part_two(data):
    miner = Miner(data)

    for possible_solution in itertools.count():
        if miner.has_six_zeroes(possible_solution):
            return possible_solution

    return Exception('No solution')


def main():
    data = get_data()

    print('Part one solution:', part_one(data))
    print('Part two solution:', part_two(data))


if __name__ == '__main__':
    main()


