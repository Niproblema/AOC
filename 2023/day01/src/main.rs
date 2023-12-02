use std::fs::read_to_string;

fn main() {
    let input_path = "./inputs/input1.txt";
    let input_data = read_lines(input_path);

    part1(&input_data);
    part2(&input_data);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn part1(input: &Vec<String>) {
    let mut sum = 0;
    for line in input {
        let mut first_idx = line.len();
        let mut last_idx = 0;
        let mut first_char = 'a';
        let mut last_char = 'a';

        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if i <= first_idx {
                    first_idx = i;
                    first_char = c;
                }
                if i >= last_idx {
                    last_idx = i;
                    last_char = c;
                }
            }
        }
        let to_add = format!("{}{}", first_char, last_char);
        let to_add: i32 = to_add.parse().unwrap();
        sum += to_add;
    }
    println!("{}", sum);
}

fn part2(input: &Vec<String>) {
    let replacements = [
        ["one", "one1one"],
        ["two", "two2two"],
        ["three", "three3three"],
        ["four", "four4four"],
        ["five", "five5five"],
        ["six", "six6six"],
        ["seven", "seven7seven"],
        ["eight", "eight8eight"],
        ["nine", "nine9nine"],
    ];

    let mut new_lines = Vec::new();
    for line in input {
        let mut temp_str = String::from(line);
        for rep in replacements {
            temp_str = temp_str.replace(rep[0], rep[1]);
        }
        new_lines.push(temp_str);
    }

    part1(&new_lines);
}
