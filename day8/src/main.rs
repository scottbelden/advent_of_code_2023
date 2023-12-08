use std::{collections::HashMap, fs};

fn main() {
    let file_path = "src/input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut path_map = HashMap::new();
    let mut lines = contents.lines();
    let path = lines.next().unwrap();

    lines.next(); // Skip blank line
    for line in lines {
        let (current_position, destinations) = line.split_once(" = ").unwrap();
        let (left_destination, right_destination) = destinations[1..(destinations.len() - 1)]
            .split_once(", ")
            .unwrap();
        path_map.insert(
            current_position,
            HashMap::from([('L', left_destination), ('R', right_destination)]),
        );
    }

    let mut steps = 0;
    let mut current_location = "AAA";
    for char in path.chars().cycle() {
        steps += 1;
        match char {
            'L' => current_location = path_map.get(current_location).unwrap().get(&'L').unwrap(),
            _ => current_location = path_map.get(current_location).unwrap().get(&'R').unwrap(),
        }
        if current_location == "ZZZ" {
            break;
        }
    }

    println!("Part 1: {steps}");

    // println!("Part 2: {product}");
}
