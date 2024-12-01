from pathlib import Path

def transpose(l):
    return [list(row) for row in zip(*l)]

input = transpose(
    (int(n) for n in line.split())
    for line in Path("inputs/day01.txt").read_text().strip().split("\n")
)

input[0].sort()
input[1].sort()

part1 = sum(
    abs(a - b)
    for a, b in zip(input[0], input[1])
)

part2 = sum(
    n * input[1].count(n)
    for n in input[0]
)

print(f"Part 1: {part1}")
print(f"Part 2: {part2}")
