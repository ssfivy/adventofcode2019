#!/usr/bin/env python3

import math

def mass2fuel(mass):
    return math.floor(mass / 3) - 2

with open('01-1-input.txt', 'r') as fb:
    fuel = 0
    for row in fb:
        fuel += mass2fuel(int(row))

    print('part1:')
    print(fuel)

def mass2fuel_recursive(mass):
    fuel = math.floor(mass / 3) - 2
    if fuel <= 0:
        return 0
    else:
        return fuel + mass2fuel_recursive(fuel)

# test recursive
assert(mass2fuel_recursive(1969) == 966)
assert(mass2fuel_recursive(100756) == 50346)

with open('01-1-input.txt', 'r') as fb:
    fuel = 0
    for module in fb:
        fuel += mass2fuel_recursive(int(module))

    print('part2:')
    print(fuel)
