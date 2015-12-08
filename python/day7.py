from __future__ import print_function

import re
import os


OPERATIONS = {

}


class Circuit(object):
    def __init__(self, wiring_lines):
        self.connections = {}
        self.value_cache = {}
        self.operators = {
            'NOT': self.perform_not,
            'AND': self.perform_and,
            'OR': self.perform_or,
            'LSHIFT': self.perform_lshift,
            'RSHIFT': self.perform_rshift,
            'DIRECT': self.perform_direct,
        }

        for line in wiring_lines:
            self.parse_line(line)

    def parse_line(self, line):
        tokens = line.strip().split(' ')

        _, output = tokens[-2:]
        operation = tokens[:-2]

        if len(operation) == 1:
            # direct connection
            self.add_connection(output, 'DIRECT', operation[0], None)
        elif len(operation) == 2:
            # NOT
            self.add_connection(output, operation[0], operation[1], None)
        else:
            self.add_connection(output, operation[1], operation[0], operation[2])

    def add_connection(self, output, operator, lhs, rhs):
        self.connections[output] = {
            'operator': operator,
            'lhs': lhs,
            'rhs': rhs,
        }

    def perform_not(self, lhs, rhs):
        return ~self.resolve_name(lhs)

    def perform_and(self, lhs, rhs):
        return self.resolve_name(lhs) & self.resolve_name(rhs)

    def perform_or(self, lhs, rhs):
        return self.resolve_name(lhs) | self.resolve_name(rhs)

    def perform_lshift(self, lhs, rhs):
        return self.resolve_name(lhs) << self.resolve_name(rhs)

    def perform_rshift(self, lhs, rhs):
        return self.resolve_name(lhs) >> self.resolve_name(rhs)

    def perform_direct(self, lhs, rhs):
        return self.resolve_name(lhs)

    def cache_value(self, wire_name, value):
        # print('Stored {} for {}'.format(value, wire_name))
        self.value_cache[wire_name] = value

    def resolve_name(self, wire_name):
        if wire_name is None:
            return None

        if wire_name in self.value_cache:
            return self.value_cache[wire_name]

        try:
            value = int(wire_name)
        except ValueError:
            pass
        else:
            self.cache_value(wire_name, value)
            return value

        connection = self.connections[wire_name]
        # print('{} = {}'.format(wire_name, connection))
        operator = self.operators[connection['operator']]

        value = operator(connection['lhs'], connection['rhs']) & 0xFFFF
        self.cache_value(wire_name, value)

        return value


def get_data():
    with open(os.path.join(os.path.dirname(__file__), '..', 'data', 'day7.txt')) as data_fp:
        return data_fp.readlines()


def part_one(data):
    circuit = Circuit(data)
    return circuit.resolve_name('a')


def part_two(data):
    circuit = Circuit(data)
    circuit.connections['b'] = {'operator': 'DIRECT', 'lhs': 3176, 'rhs': None}
    return circuit.resolve_name('a')


def test_one():
    lines = [
        '123 -> x',
        '456 -> y',
        'x AND y -> d',
        'x OR y -> e',
        'x LSHIFT 2 -> f',
        'y RSHIFT 2 -> g',
        'NOT x -> h',
        'NOT y -> i',
    ]

    outputs = {
        'd': 72,
        'e': 507,
        'f': 492,
        'g': 114,
        'h': 65412,
        'i': 65079,
        'x': 123,
        'y': 456,
    }

    circuit = Circuit(lines)

    for key, value in outputs.items():
        circuit_value = circuit.resolve_name(key)

        if circuit_value == value:
            print('{} resolved correctly.'.format(key))
        else:
            print('{} resolved incorrectly.'.format(key))
            print('\tExpected: {}'.format(value))
            print('\tActual: {}'.format(circuit_value))


def main():
    data = get_data()

    print('Part one solution:', part_one(data))
    print('Part two solution:', part_two(data))


if __name__ == '__main__':
    main()




