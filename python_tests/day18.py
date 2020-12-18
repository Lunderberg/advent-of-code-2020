#!/usr/bin/env python3

import re
import sys

def eval_line_p1(line):
    class Num:
        def __init__(self, x):
            self.x = x

        def __repr__(self):
            return f'Num({self.x})'

        def __add__(self, other):
            return Num(self.x + other.x)

        def __sub__(self, other):
            return Num(self.x * other.x)

    line = line.replace('*', '-')
    line = re.sub(r"(\d+)", r"Num(\1)", line)
    return eval(line).x


def eval_line_p2(line):
    class Num:
        def __init__(self, x):
            self.x = x

        def __repr__(self):
            return f'Num({self.x})'

        def __mul__(self, other):
            return Num(self.x + other.x)

        def __add__(self, other):
            return Num(self.x * other.x)

    line = line.replace('*', 't').replace('+','*').replace('t','+')
    line = re.sub(r"(\d+)", r"Num(\1)", line)
    return eval(line).x


def main():
    lines = [line.strip() for line in open(sys.argv[1])]
    print(sum(eval_line_p1(line) for line in lines))
    print(sum(eval_line_p2(line) for line in lines))

if __name__ == '__main__':
    main()
