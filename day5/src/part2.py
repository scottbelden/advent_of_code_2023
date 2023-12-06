from itertools import batched
import math
from pprint import pprint

chunks: list[list[str]] = []
chunk: list[str] = []
with open("input.txt") as fp:
    for line in fp:
        line = line.strip()
        if line == "":
            chunks.append([c for c in chunk])
            chunk = []
        else:
            chunk.append(line)

    chunks.append([c for c in chunk])

seeds: list[int] = []
transformer_groups: list[list[tuple[int, int, int]]] = []

for chunk in chunks:
    transformer_group: list[tuple[int, int, int]] = []
    add_group = True
    for line in chunk:
        if line.startswith("seeds"):
            seeds = [int(seed) for seed in line.split(": ")[1].split()]
            add_group = False
            continue
        elif ":" in line:
            continue

        dest_start, source_start, range_num = (int(value) for value in line.split())
        transformer_group.append((dest_start, source_start, range_num))

    if add_group:
        transformer_groups.append(transformer_group.copy())


def get_location(seed: int, transformer_groups: list[list[tuple[int, int, int]]]) -> int:
    current_value = seed

    for tranformer_group in transformer_groups:
        for dest_start, source_start, range_num in tranformer_group:
            if source_start <= current_value < source_start + range_num:
                current_value = dest_start + (current_value - source_start)
                break

    return current_value


locations: list[int] = []
for seed in seeds:
    locations.append(get_location(seed, transformer_groups))

print(f"part1: {min(locations)}")


def get_min_location(seed_start: int, seed_range: int) -> int:
    start_location = get_location(seed_start, transformer_groups)
    end_location = get_location(seed_start + seed_range - 1, transformer_groups)
    if end_location - start_location + 1 == seed_range:
        return start_location
    elif seed_range == 2:
        return min(start_location, end_location)
    elif seed_range % 2 == 0:
        return min(
            get_min_location(seed_start, int(seed_range / 2)),
            get_min_location(seed_start + int(seed_range / 2), int(seed_range / 2)),
        )
    else:
        return min(
            get_min_location(seed_start, math.floor(seed_range / 2)),
            get_min_location(seed_start + math.floor(seed_range / 2), math.ceil(seed_range / 2)),
        )


locations: list[int] = []
for seed_start, seed_range in batched(seeds, 2):
    min_location = get_min_location(seed_start, seed_range)
    locations.append(min_location)

print(f"part2: {min(locations)}")
