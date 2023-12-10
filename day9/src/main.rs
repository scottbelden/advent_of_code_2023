use std::fs;

fn get_back_vector(input_line: Vec<isize>) -> Vec<isize> {
    let sub_vector: Vec<isize> = input_line
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect();
    if sub_vector.iter().all(|&item| item == 0) {
        return vec![input_line[input_line.len() - 1], 0];
    } else {
        let mut result = vec![input_line[input_line.len() - 1]];
        result.append(&mut get_back_vector(sub_vector));
        return result;
    }
}

fn get_front_vector(input_line: Vec<isize>) -> Vec<isize> {
    let sub_vector: Vec<isize> = input_line
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect();
    if sub_vector.iter().all(|&item| item == 0) {
        return vec![input_line[0], 0];
    } else {
        let mut result = vec![input_line[0]];
        result.append(&mut get_front_vector(sub_vector));
        return result;
    }
}

fn main() {
    let file_path = "src/input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut sum = 0;
    for line in contents.lines() {
        let output = get_back_vector(
            line.split_whitespace()
                .map(|value| value.parse::<isize>().unwrap())
                .collect(),
        );
        sum += output.iter().sum::<isize>();
    }

    println!("Part 1: {sum}");

    let mut sum = 0;
    for line in contents.lines() {
        let output = get_front_vector(
            line.split_whitespace()
                .map(|value| value.parse::<isize>().unwrap())
                .collect(),
        );
        sum += output.iter().rev().fold(0, |acc, &elem| elem - acc);
    }

    println!("Part 2: {sum}");
}
