use std::fs;

fn _get_first_last_value(line: &str) -> u32 {
    let mut first: u32 = 0;
    let mut last: u32 = 0;
    for char in line.chars() {
        if char.is_ascii_digit() {
            first = char.to_digit(10).unwrap();
            break
        }
    }
    for char in line.chars().rev() {
        if char.is_ascii_digit() {
            last = char.to_digit(10).unwrap();
            break
        }
    }

    return (first * 10) + last;
}

fn main() {
    let file_path2 = "src/day1_input";
    
    let contents = fs::read_to_string(file_path2)
        .expect("Should have been able to read the file");

    let mut sum = 0;
    for line in contents.lines() {
        sum += _get_first_last_value(line);
    }

    println!("{sum}");

    sum = 0;
    for line in contents.lines() {
        let fixed_line = line
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");
        sum += _get_first_last_value(&fixed_line);
    }

    println!("{sum}");

}
