use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/12.txt").unwrap();

    let lines = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("PART 1: {}", part1(&lines));
}

fn part1(lines: &Vec<String>) -> u64 {
    lines
        .iter()
        .map(|line| {
            let split = line.split(" ").collect::<Vec<&str>>();
            let row = split.first().unwrap();
            let group_sizes = split.last().unwrap().split(",")
                .map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();

            calculate_possibilities(row, &group_sizes)
        })
        .sum()
}

fn calculate_possibilities(row: &str, groups: &Vec<usize>) -> u64 {
    let first_unknown = row.find('?');

    if first_unknown == None {
        if is_row_valid(row, groups) {
            return 1;
        }

        return 0;
    }

    let first_unknown_index = first_unknown.unwrap();
    let first_possibility = row[0..first_unknown_index].to_string() + "." + &row[first_unknown_index + 1..].to_string();
    let second_possibility = row[0..first_unknown_index].to_string() + "#" + &row[first_unknown_index + 1..].to_string();

    return calculate_possibilities(&first_possibility, groups) + calculate_possibilities(&second_possibility, groups);
}

fn is_row_valid(row: &str, groups: &Vec<usize>) -> bool {
    let present_groups = row.split(".").filter(|group| !group.is_empty())
        .collect::<Vec<&str>>();

    if present_groups.len() != groups.len() {
        return false;
    }

    zip(present_groups, groups).all(|(group, size)| group.len() == *size)
}
