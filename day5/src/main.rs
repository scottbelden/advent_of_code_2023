use itertools::Itertools;
use std::{fs, str::Lines};

fn generate_vec(lines: &mut Lines<'_>) -> Vec<(u64, u64, u64)> {
    let mut output = Vec::new();
    for line in lines {
        let (destination_start, source_start, range) = line
            .split_whitespace()
            .map(|item| item.parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();
        output.push((destination_start, source_start, range));
    }
    return output;
}

fn get_next_value(current_value: &u64, transform_vector: &Vec<(u64, u64, u64)>) -> u64 {
    for (destination_start, source_start, range) in transform_vector {
        let end = source_start + range;
        if source_start <= current_value && current_value < &end {
            return destination_start + (current_value - source_start);
        }
    }
    return current_value.clone();
}

fn get_location(
    seed: &u64,
    seed_to_soil_vec: &Vec<(u64, u64, u64)>,
    soil_to_fertilizer_vec: &Vec<(u64, u64, u64)>,
    fertilizer_to_water_vec: &Vec<(u64, u64, u64)>,
    water_to_light_vec: &Vec<(u64, u64, u64)>,
    light_to_temperature_vec: &Vec<(u64, u64, u64)>,
    temperature_to_humidity_vec: &Vec<(u64, u64, u64)>,
    humidity_to_location_vec: &Vec<(u64, u64, u64)>,
) -> u64 {
    let soil = get_next_value(&seed, &seed_to_soil_vec);
    let fertilizer = get_next_value(&soil, &soil_to_fertilizer_vec);
    let water = get_next_value(&fertilizer, &fertilizer_to_water_vec);
    let light = get_next_value(&water, &water_to_light_vec);
    let temperature = get_next_value(&light, &light_to_temperature_vec);
    let humidity = get_next_value(&temperature, &temperature_to_humidity_vec);
    let location = get_next_value(&humidity, &humidity_to_location_vec);
    return location.clone();
}

fn main() {
    let file_path = "src/input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut seeds = Vec::new();
    let mut seed_to_soil_vec = Vec::new();
    let mut soil_to_fertilizer_vec = Vec::new();
    let mut fertilizer_to_water_vec = Vec::new();
    let mut water_to_light_vec = Vec::new();
    let mut light_to_temperature_vec = Vec::new();
    let mut temperature_to_humidity_vec = Vec::new();
    let mut humidity_to_location_vec = Vec::new();
    for chunk in contents.split("\n\n") {
        let mut lines = chunk.lines();
        let first_line = lines.next().unwrap();
        if first_line.starts_with("seeds:") {
            seeds = first_line[7..]
                .split_whitespace()
                .map(|item| item.parse::<u64>().unwrap())
                .collect();
        } else {
            if first_line == "seed-to-soil map:" {
                seed_to_soil_vec = generate_vec(&mut lines);
            } else if first_line == "soil-to-fertilizer map:" {
                soil_to_fertilizer_vec = generate_vec(&mut lines);
            } else if first_line == "fertilizer-to-water map:" {
                fertilizer_to_water_vec = generate_vec(&mut lines);
            } else if first_line == "water-to-light map:" {
                water_to_light_vec = generate_vec(&mut lines);
            } else if first_line == "light-to-temperature map:" {
                light_to_temperature_vec = generate_vec(&mut lines);
            } else if first_line == "temperature-to-humidity map:" {
                temperature_to_humidity_vec = generate_vec(&mut lines);
            } else if first_line == "humidity-to-location map:" {
                humidity_to_location_vec = generate_vec(&mut lines);
            }
        }
    }

    let mut locations = Vec::new();
    for seed in seeds {
        locations.push(get_location(
            &seed,
            &seed_to_soil_vec,
            &soil_to_fertilizer_vec,
            &fertilizer_to_water_vec,
            &water_to_light_vec,
            &light_to_temperature_vec,
            &temperature_to_humidity_vec,
            &humidity_to_location_vec,
        ));
    }
    let lowest = locations.iter().min().unwrap();

    println!("Part 1: {lowest}");
}
