use std::fs;
use itertools::Itertools;

fn is_within_cube_limit(cube_color: &str, num_cubes: i32) -> bool {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    match cube_color {
        "red" => num_cubes <= max_red,
        "green" => num_cubes <= max_green,
        "blue" => num_cubes <= max_blue,
        _ => panic!(),
    }
}

fn main() {
    let file_path = "src/input.txt";
    
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut sum = 0;
    'outer: for line in contents.lines() {
        let mut parts = line.split(": ").take(2);
        let game_part = parts.next().unwrap();
        let game_id = game_part[5..].parse::<i32>().unwrap();

        for cube_group in parts.next().unwrap().split("; ") {
            for cube_part in cube_group.split(", ") {
                let (num_cubes, cube_color) = cube_part.split(" ").collect_tuple().unwrap();
                if !is_within_cube_limit(cube_color, num_cubes.parse::<i32>().unwrap()) {
                    continue 'outer
                }
            }
        }
        sum += game_id;

    }
    println!("{sum}");


    let mut sum = 0;
    for line in contents.lines() {
        let parts = line.split(": ").take(2);
        let (_game_part, cube_part) = parts.collect_tuple().unwrap();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for cube_group in cube_part.split("; ") {
            for cube_part in cube_group.split(", ") {
                let (num_cubes, cube_color) = cube_part.split(" ").collect_tuple().unwrap();
                let n_cubes = num_cubes.parse::<i32>().unwrap();
                if cube_color == "red" && n_cubes > max_red {
                    max_red = n_cubes
                }
                if cube_color == "green" && n_cubes > max_green {
                    max_green = n_cubes
                }
                if cube_color == "blue" && n_cubes > max_blue {
                    max_blue = n_cubes
                }
            }
        }
        sum += max_red * max_green * max_blue;

    }
    println!("{sum}");


}
