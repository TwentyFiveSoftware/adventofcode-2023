use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/13.txt").unwrap();

    let lines = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("PART 1: {}", part1(&lines));
}

fn part1(lines: &Vec<String>) -> u64 {
    lines
        .split(|line| line.is_empty())
        .map(|pattern| {
            let vertical_line = (1..pattern[0].len()).find_map(|line_index| {
                if is_pattern_symmetrical_to_vertical_line(pattern, line_index) {
                    return Some(line_index);
                }
                return None;
            });

            if let Some(line_index) = vertical_line {
                return line_index as u64;
            }

            let horizontal_line = (1..pattern.len()).find_map(|line_index| {
                if is_pattern_symmetrical_to_horizontal_line(pattern, line_index) {
                    return Some(line_index);
                }
                return None;
            });

            if let Some(line_index) = horizontal_line {
                return (line_index as u64) * 100;
            }

            println!("not symmetrical!");
            return 0;
        })
        .sum()
}

fn is_pattern_symmetrical_to_vertical_line(pattern: &[String], line_index: usize) -> bool {
    let min_side_width = min(line_index, pattern[0].len() - line_index);

    let left = pattern.iter()
        .map(|line| (&line[line_index - min_side_width..line_index]).chars().rev().collect::<String>());
    let right = pattern.iter()
        .map(|line| String::from(&line[line_index..line_index + min_side_width]));

    zip(left, right).all(|(a, b)| a == b)
}

fn is_pattern_symmetrical_to_horizontal_line(pattern: &[String], line_index: usize) -> bool {
    let min_side_width = min(line_index, pattern.len() - line_index);

    let top = (&pattern[line_index - min_side_width..line_index]).iter().rev();
    let bottom = (&pattern[line_index..line_index + min_side_width]).iter();

    zip(top, bottom).all(|(a, b)| a == b)
}
