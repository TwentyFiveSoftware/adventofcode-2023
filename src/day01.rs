use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/01.txt").unwrap();
    let input = BufReader::new(input_file);

    let lines = input
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("PART 1: {}", part1(&lines));
    println!("PART 2: {}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> i32 {
    lines
        .iter()
        .map(|line| {
            let numbers = line
                .chars()
                .filter(|char| char.is_ascii_digit())
                .collect::<Vec<char>>();

            return format!("{}{}", numbers[0], numbers[numbers.len() - 1]);
        })
        .map(|number_string| number_string.parse::<i32>().unwrap())
        .sum()
}

fn part2(lines: &Vec<String>) -> i32 {
    lines
        .iter()
        .map(|whole_line| {
            let mut numbers: Vec<u32> = Vec::new();
            let mut line = &whole_line[..];

            while line.len() > 0 {
                if line.starts_with(|c: char| c.is_ascii_digit()) {
                    numbers.push(line.chars().next().unwrap().to_digit(10).unwrap());
                    line = &line[1..];
                    continue;
                }

                let number = match () {
                    _ if line.starts_with("one") => 1,
                    _ if line.starts_with("two") => 2,
                    _ if line.starts_with("three") => 3,
                    _ if line.starts_with("four") => 4,
                    _ if line.starts_with("five") => 5,
                    _ if line.starts_with("six") => 6,
                    _ if line.starts_with("seven") => 7,
                    _ if line.starts_with("eight") => 8,
                    _ if line.starts_with("nine") => 9,
                    _ => 0
                };

                if number > 0 {
                    numbers.push(number);
                }

                line = &line[1..];
            }

            return format!("{}{}", numbers[0], numbers[numbers.len() - 1]);
        })
        .map(|number_string| number_string.parse::<i32>().unwrap())
        .sum()
}
