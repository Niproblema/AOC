use std::fs::read_to_string;

fn main() {
    let input_path = "./inputs/input.txt";
    let input_data = read_lines(input_path);

    let result1 = task_1(&input_data);
    let result2 = task_2(&input_data);

    println!("{:?}", result1);
    println!("{:?}", result2);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn task_1(input: &Vec<String>) -> i32 {
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    let mut sum = 0;

    let mut i = 0;
    while i < input.len() {
        let splits: Vec<&str> = input[i].split(' ').collect();

        let mut k = 0;
        let mut is_ok = true;
        while k < splits.len() {
            let potential_count = splits[k];
            let potential_color = splits[k + 1];

            if potential_color.contains("red") && potential_count.parse::<i32>().unwrap() > MAX_RED
            {
                is_ok = false;
                break;
            }

            if potential_color.contains("green")
                && potential_count.parse::<i32>().unwrap() > MAX_GREEN
            {
                is_ok = false;
                break;
            }

            if potential_color.contains("blue")
                && potential_count.parse::<i32>().unwrap() > MAX_BLUE
            {
                is_ok = false;
                break;
            }

            k += 2;
        }

        if is_ok {
            sum += i + 1;
        }

        i += 1;
    }

    sum as i32
}

fn task_2(input: &Vec<String>) -> u64 {
    let mut sum: u64 = 0;

    let mut i = 0;
    while i < input.len() {
        let splits: Vec<&str> = input[i].split(' ').collect();

        let (mut red, mut green, mut blue): (u64, u64, u64) = (0, 0, 0);

        let mut k = 0;
        while k < splits.len() {
            let potential_count = splits[k];
            let potential_color = splits[k + 1];

            if potential_color.contains("red") && potential_count.parse::<u64>().unwrap() > red {
                red = potential_count.parse::<u64>().unwrap();
            }

            if potential_color.contains("green") && potential_count.parse::<u64>().unwrap() > green
            {
                green = potential_count.parse::<u64>().unwrap();
            }

            if potential_color.contains("blue") && potential_count.parse::<u64>().unwrap() > blue {
                blue = potential_count.parse::<u64>().unwrap();
            }

            k += 2;
        }

        sum += red*green*blue;

        i += 1;
    }

    sum
}
