#!/usr/bin/env python3

# I am not familiar enough with doing geometrical stuff in code
# so using python for now, dont want to do two things at the same time

# ok I'm allowing google and external libraries too
from shapely.geometry import LineString


def calculate_shortest_distance(lines):
    line = 0
    points = [[(0,0)], [(0,0)]]

    for l in lines:
        x1 = 0
        y1 = 0
        directions = l.strip().split(',')
        for d in directions:
            # too many repetitions, not beautiful :(
            if   d[0] == 'R':
                x2 = x1 + int(d[1:])
                points[line].append((x2,y1))
                x1 = x2
            elif d[0] == 'L':
                x2 = x1 - int(d[1:])
                points[line].append((x2,y1))
                x1 = x2
            elif d[0] == 'U':
                y2 = y1 + int(d[1:])
                points[line].append((x1,y2))
                y1 = y2
            elif d[0] == 'D':
                y2 = y1 - int(d[1:])
                points[line].append((x1,y2))
                y1 = y2
        line += 1

    #print(points)

    minimum = None

    line0 = LineString(points[0])
    line1 = LineString(points[1])
    crossings = line0.intersection(line1)
    for cross in crossings:
        #print(cross)
        distance = abs(cross.x) + abs(cross.y)
        if distance == 0:
            continue
        if minimum is None:
            minimum = distance
        elif distance < minimum:
            minimum = distance

    print(minimum)
    return minimum


# test 1
test1 = ['R75,D30,R83,U83,L12,D49,R71,U7,L72',
        'U62,R66,U55,R34,D71,R55,D58,R83']
result = calculate_shortest_distance(test1)
assert(result == 159)

# test 2
test2 = ['R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51',
        'U98,R91,D20,R16,D67,R40,U7,R15,U6,R7']
result = calculate_shortest_distance(test2)
assert(result == 135)

# real thing
with open('03-1-input.txt', 'r') as fb:
    inputs = []
    for l in fb:
        inputs.append(l)
    result = calculate_shortest_distance(inputs)
    print('Part 1 result: {}'.format(result))




