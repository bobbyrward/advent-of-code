from __future__ import print_function

import os


def get_data():
    with open(os.path.join(os.path.dirname(__file__), '..', 'data', 'day8.txt')) as data_fp:
        return data_fp.readlines()


def part_one(data):
    code_count = 0
    actual_count = 0

    for line in data:
        line = line.strip()
        code_count += len(line)
        actual_count += len(eval(line))

    return code_count - actual_count


def part_two(data):
    code_count = 0
    encoded_count = 0

    for line in data:
        line = line.strip()
        code_count += len(line)

        encoded = line.replace('\\', '\\\\')
        encoded = encoded.replace('"', '\\"')
        encoded = ''.join(['"', encoded, '"'])
        encoded_count += len(encoded)

    return encoded_count - code_count


def test_one():
    data = [
        r'""',
        r'"abc"',
        r'"aaa\"aaa"',
        r'"\x27"',
    ]

    return part_one(data)


def test_two():
    data = [
        r'""',
        r'"abc"',
        r'"aaa\"aaa"',
        r'"\x27"',
    ]

    return part_two(data)


def main():
    data = get_data()

    # print('Test one solution:', test_one())
    print('Part one solution:', part_one(data))
    # print('Test two solution:', test_two())
    print('Part two solution:', part_two(data))


if __name__ == '__main__':
    main()
