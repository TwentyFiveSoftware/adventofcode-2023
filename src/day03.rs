use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/03.txt").unwrap();

    let lines = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("PART 1: {}", part1(&lines));
    println!("PART 2: {}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> i32 {
    parse_numbers(lines)
        .iter()
        .map(|number| (number.value, get_chars_around_number(lines, number)))
        .filter_map(|(number, surrounding_chars)| {
            if surrounding_chars.iter().any(|(c, _)| !c.is_ascii_digit() && *c != '.') {
                return Some(number);
            }
            return None;
        })
        .sum()
}

fn part2(lines: &Vec<String>) -> i32 {
    parse_numbers(lines)
        .iter()
        .fold(HashMap::new(), |mut numbers_per_gear, number| {
            get_chars_around_number(lines, &number)
                .iter()
                .filter_map(|(c, position)| {
                    if *c == '*' {
                        return Some(position.clone());
                    }
                    return None;
                })
                .for_each(|gear_position| {
                    numbers_per_gear.entry(gear_position).or_insert(Vec::new()).push(number.value);
                });

            numbers_per_gear
        })
        .values()
        .filter(|adjacent_numbers| adjacent_numbers.len() == 2)
        .map(|adjacent_numbers| adjacent_numbers.iter().fold(1, |acc, n| acc * n))
        .sum()
}

fn parse_numbers(lines: &Vec<String>) -> Vec<NumberInfo> {
    let number_regex = Regex::new(r"^([0-9]+)(?:[^0-9]|$)").unwrap();

    lines
        .iter()
        .enumerate()
        .map(|(line_index, line)| {
            let mut numbers: Vec<NumberInfo> = Vec::new();

            let mut i = 0;
            while i < line.len() {
                match number_regex.captures(&line[i..]) {
                    None => {
                        i += 1;
                    }
                    Some(groups) => {
                        let number = groups.get(1).unwrap().as_str();

                        numbers.push(NumberInfo {
                            value: number.parse::<i32>().unwrap(),
                            line_index,
                            start_index: i,
                            length: number.len(),
                        });

                        i += number.len();
                    }
                }
            }

            numbers
        })
        .flatten()
        .collect()
}

struct NumberInfo {
    value: i32,
    line_index: usize,
    start_index: usize,
    length: usize,
}

fn get_chars_around_number(
    lines: &Vec<String>,
    number: &NumberInfo,
) -> Vec<(char, (usize, usize))> {
    let inclusive_line_start_index = max(0, number.line_index as i32 - 1) as usize;
    let exclusive_line_end_index = min(lines.len(), number.line_index + 2);

    let inclusive_char_start_index = max(0, number.start_index as i32 - 1) as usize;
    let exclusive_char_end_index =
        min(lines[number.line_index].len(), number.start_index + number.length + 1);

    let mut chars: Vec<(char, (usize, usize))> = vec![];

    for i in inclusive_line_start_index..exclusive_line_end_index {
        for j in inclusive_char_start_index..exclusive_char_end_index {
            chars.push((lines[i].chars().nth(j).unwrap(), (i, j)))
        }
    }

    return chars;
}
