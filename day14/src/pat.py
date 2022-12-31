import json
AIR, SAND, ROCK, SOURCE = '.', 'o', '#', '+'
with open('../input.txt') as in_file:
    rock_paths = [json.loads(f"[[{row.replace('->', '],[')}]]")
                    for row in in_file]
# Normalize the data to simplify visualization
min_x, max_x, max_y = 1000, 0, 0
for rock_path in rock_paths:
    for x, y in rock_path:
        min_x, max_x, max_y = min(min_x, x), max(max_x, x), max(max_y, y)
for i, rock_path in enumerate(rock_paths):
    rock_paths[i] = [(x - min_x, y) for x, y in rock_path]
max_x -= min_x
grid = [[AIR] * (max_y + 1) for _ in range(max_x + 1)]
source = (500 - min_x, 0)
grid[source[0]][source[1]] = SOURCE
# Load grid with rocks
for rock_path in rock_paths:
    for (x1, y1), (x2, y2) in zip(rock_path[:-1], rock_path[1:]):
        if y1 == y2:
            start, end = min(x1, x2), max(x1, x2) + 1
            points = ((x, y1) for x in range(start, end))
        elif x1 == x2:
            start, end = min(y1, y2), max(y1, y2) + 1
            points = ((x1, y) for y in range(start, end))
        for x, y in points:
            grid[x][y] = ROCK
# Fill structure with sand until full
done = False
number_of_grains = 0
while not done:
    x, y = source
    while True:
        if y == max_y:
            done = True
            break
        elif x in [-1, max_x] or grid[x][y + 1] == AIR:
            y += 1
        elif x == 0 or grid[x - 1][y + 1] == AIR:
            x, y = x - 1, y + 1
        elif x == max_x or grid[x + 1][y + 1] == AIR:
            x, y = x + 1, y + 1
        else:
            break
    if not done:
        number_of_grains += 1
        grid[x][y] = SAND
print('\n'.join([''.join(row) for row in zip(*grid)]))
print(number_of_grains)