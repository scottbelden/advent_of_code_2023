use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn get_matches(line: &str) -> usize {
    let (_, all_numbers) = line.split(": ").collect_tuple().unwrap();
    let (winning_group, my_group) = all_numbers.split(" | ").collect_tuple().unwrap();
    let winning_numbers: HashSet<&str> = winning_group.split_whitespace().collect();
    let my_numbers: HashSet<&str> = my_group.split(" ").collect();
    let intersection = winning_numbers.intersection(&my_numbers);
    let matches = intersection.count();
    return matches;
}

fn main() {
    let file_path = "src/input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut sum = 0;
    for line in contents.lines() {
        let matches = get_matches(line);
        if matches > 0 {
            sum += i32::pow(2, matches as u32 - 1);
        }
    }

    println!("Part 1: {sum}");

    let mut game_to_cards = HashMap::new();
    for (index, line) in contents.lines().enumerate() {
        let matches = get_matches(line);
        game_to_cards.insert(index, game_to_cards.get(&index).unwrap_or(&0) + 1);
        for i in 0..matches {
            game_to_cards.insert(
                index + i + 1,
                game_to_cards.get(&(index + i + 1)).unwrap_or(&0)
                    + game_to_cards.get(&index).unwrap(),
            );
        }
    }

    let total: i32 = game_to_cards.values().sum();
    println!("Part 2: {total}");
}
