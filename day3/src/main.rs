use std::collections::{HashMap, HashSet};
use std::fs;

fn get_symbol_and_number_locations(
    contents: &str,
) -> (HashSet<(i32, i32)>, HashSet<(i32, i32, String)>) {
    let mut symbol_locations = HashSet::new();
    let mut part_number_locations = HashSet::new();
    let mut line_index = 0;

    for line in contents.lines() {
        let mut char_index: i32 = 0;
        let mut part_number = String::new();

        for char in line.chars() {
            if char.is_ascii_digit() {
                part_number.push(char);
            } else {
                if !part_number.is_empty() {
                    part_number_locations.insert((line_index, char_index, part_number.clone()));
                    part_number.clear();
                }
                if char != '.' {
                    symbol_locations.insert((line_index, char_index));
                }
            }
            char_index += 1;
        }
        if !part_number.is_empty() {
            part_number_locations.insert((line_index, char_index, part_number.clone()));
            part_number.clear();
        }
        line_index += 1;
    }

    return (symbol_locations, part_number_locations);
}

fn get_symbol_and_number_locations_with_symbols(
    contents: &str,
) -> (HashSet<(i32, i32, char)>, HashSet<(i32, i32, String)>) {
    let mut symbol_locations = HashSet::new();
    let mut part_number_locations = HashSet::new();
    let mut line_index = 0;

    for line in contents.lines() {
        let mut char_index: i32 = 0;
        let mut part_number = String::new();

        for char in line.chars() {
            if char.is_ascii_digit() {
                part_number.push(char);
            } else {
                if !part_number.is_empty() {
                    part_number_locations.insert((line_index, char_index, part_number.clone()));
                    part_number.clear();
                }
                if char != '.' {
                    symbol_locations.insert((line_index, char_index, char));
                }
            }
            char_index += 1;
        }
        if !part_number.is_empty() {
            part_number_locations.insert((line_index, char_index, part_number.clone()));
            part_number.clear();
        }
        line_index += 1;
    }

    return (symbol_locations, part_number_locations);
}

fn main() {
    let file_path = "src/input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut sum = 0;
    let symbol_locations;
    let part_number_locations;
    (symbol_locations, part_number_locations) = get_symbol_and_number_locations(&contents);

    'outer: for (l_index, c_index, part_number_str) in &part_number_locations {
        for i in 0..(part_number_str.len() + 2) {
            if symbol_locations.contains(&(l_index - 1, c_index - i as i32))
                || symbol_locations.contains(&(*l_index, c_index - i as i32))
                || symbol_locations.contains(&(l_index + 1, c_index - i as i32))
            {
                sum += part_number_str.parse::<i32>().unwrap();
                continue 'outer;
            }
        }
    }

    println!("Part 1: {sum}");

    let symbol_locations2;
    let part_number_locations2;
    (symbol_locations2, part_number_locations2) =
        get_symbol_and_number_locations_with_symbols(&contents);
    let mut gears = HashMap::new();
    for (l_index, c_index, part_number_str) in &part_number_locations2 {
        for i in 0..(part_number_str.len() + 2) {
            for j in 0..3 {
                let possible_symbol = (l_index + (j - 1), c_index - i as i32, '*');
                if symbol_locations2.contains(&possible_symbol) {
                    gears
                        .entry(possible_symbol)
                        .and_modify(|nums: &mut HashSet<(&i32, &i32, &String)>| {
                            nums.insert((l_index, c_index, part_number_str));
                        })
                        .or_insert(HashSet::from([(l_index, c_index, part_number_str)]));
                }
            }
        }
    }

    let mut sum = 0;
    for (_gear, numbers) in gears {
        if numbers.len() != 2 {
            continue;
        }
        let mut product = 1;
        for (_, _, number) in numbers {
            let parsed_number: i32 = number.parse().unwrap();
            product *= parsed_number;
        }
        sum += product;
    }

    println!("Part 2: {sum}");
}
