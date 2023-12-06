use std::fs;

fn main() {
    let file_path = "src/input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut lines = contents.lines();

    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|item| item.parse::<u32>().unwrap_or(0))
        .collect::<Vec<u32>>();

    let distances = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|item| item.parse::<u32>().unwrap_or(0))
        .collect::<Vec<u32>>();

    let mut product = 1;
    for (index, time) in times.into_iter().enumerate() {
        let mut count = 0;
        if time == 0 {
            continue;
        }
        for i in 0..time {
            if i * (time - i) > distances[index] {
                count += 1;
            }
        }
        if count > 0 {
            product *= count;
        }
    }

    println!("Part 1: {product}");
}
