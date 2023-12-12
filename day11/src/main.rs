use itertools::Itertools;
use std::{collections::HashSet, fs};

fn get_distances(
    galaxy_locations: &HashSet<(usize, usize)>,
    x_values: &HashSet<usize>,
    y_values: &HashSet<usize>,
    spacing: usize,
) -> usize {
    let mut total = 0;
    for pairs in galaxy_locations.iter().combinations(2) {
        let (galaxy1_x, galaxy1_y) = *pairs[0];
        let (galaxy2_x, galaxy2_y) = *pairs[1];
        let mut x_space = 0;
        for x_range in galaxy1_x.min(galaxy2_x)..galaxy1_x.max(galaxy2_x) {
            if !x_values.contains(&x_range) {
                x_space += spacing;
            }
        }
        let mut y_space = 0;
        for y_range in galaxy1_y.min(galaxy2_y)..galaxy1_y.max(galaxy2_y) {
            if !y_values.contains(&y_range) {
                y_space += spacing;
            }
        }
        let total_distance =
            (galaxy2_y.abs_diff(galaxy1_y) + y_space) + (galaxy2_x.abs_diff(galaxy1_x) + x_space);
        total += total_distance;
    }
    return total;
}

fn main() {
    let file_path = "src/input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let galaxy_locations = contents
        .lines()
        .enumerate()
        .flat_map(|(y_index, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x_index, char)| match char {
                    '#' => Some((x_index, y_index)),
                    _ => None,
                })
        })
        .collect::<HashSet<(usize, usize)>>();

    let x_values = galaxy_locations
        .iter()
        .filter_map(|(x, _)| Some(*x))
        .collect::<HashSet<usize>>();

    let y_values = galaxy_locations
        .iter()
        .filter_map(|(_, y)| Some(*y))
        .collect::<HashSet<usize>>();

    let distances = get_distances(&galaxy_locations, &x_values, &y_values, 1);
    println!("Part 1: {distances}");

    let distances = get_distances(&galaxy_locations, &x_values, &y_values, 1000000 - 1);
    println!("Part 2: {distances}");
}
