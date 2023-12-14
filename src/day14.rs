use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/14.txt").unwrap();

    let lines = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("PART 1: {}", part1(&lines));
    println!("PART 2: {}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> u64 {
    platform_from_lines(lines)
        .tilt_vertical(true)
        .calculate_total_load()
}

fn part2(lines: &Vec<String>) -> u64 {
    let mut platform = platform_from_lines(lines);

    let mut cache = HashMap::new();

    let mut cycles_remaining = 1000000000;
    while cycles_remaining > 0 {
        platform = platform.cycle();
        cycles_remaining -= 1;

        let fingerprint = platform.fingerprint();
        if !cache.contains_key(&fingerprint) {
            cache.insert(fingerprint, cycles_remaining);
            continue;
        }

        let loop_size = cache.get(&fingerprint).unwrap() - cycles_remaining;
        cycles_remaining %= loop_size;
    }

    platform.calculate_total_load()
}

struct Platform {
    rows: Vec<Vec<char>>,
}

impl Platform {
    fn columns(&self) -> Vec<Vec<char>> {
        (0..self.rows.first().unwrap().len())
            .map(|x| self.rows
                .iter()
                .map(|line| *(line.iter().nth(x).unwrap()))
                .collect::<Vec<char>>()
            )
            .collect()
    }

    fn rows(&self) -> Vec<Vec<char>> {
        self.rows.clone()
    }

    fn cycle(&self) -> Platform {
        self
            .tilt_vertical(true)
            .tilt_horizontal(true)
            .tilt_vertical(false)
            .tilt_horizontal(false)
    }

    fn tilt_vertical(&self, north: bool) -> Platform {
        platform_from_columns(self
            .columns()
            .iter()
            .map(|column| tilt(column, north))
            .collect()
        )
    }

    fn tilt_horizontal(&self, west: bool) -> Platform {
        platform_from_rows(self
            .rows()
            .iter()
            .map(|row| tilt(row, west))
            .collect()
        )
    }

    fn calculate_total_load(&self) -> u64 {
        self.columns()
            .iter()
            .map(|column| column.iter()
                .rev()
                .enumerate()
                .filter(|(_, c)| **c == 'O')
                .map(|(n, _)| n as u64 + 1)
                .sum::<u64>()
            )
            .sum()
    }

    fn fingerprint(&self) -> String {
        self.rows.iter().flatten().collect::<String>()
    }
}

fn platform_from_lines(lines: &Vec<String>) -> Platform {
    platform_from_rows(lines.iter().map(|line| line.chars().collect::<Vec<char>>()).collect())
}

fn platform_from_rows(rows: Vec<Vec<char>>) -> Platform {
    Platform { rows }
}

fn platform_from_columns(columns: Vec<Vec<char>>) -> Platform {
    platform_from_rows(
        (0..columns.first().unwrap().len())
            .map(|y| columns
                .iter()
                .map(|column| *(column.iter().nth(y).unwrap()))
                .collect::<Vec<char>>()
            )
            .collect()
    )
}

fn tilt(line: &Vec<char>, reverse: bool) -> Vec<char> {
    let mut parts = line
        .split(|c| *c == '#')
        .map(|part| part.iter().map(|c| *c).collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    parts.iter_mut().for_each(|part| {
        part.sort_unstable();

        if reverse {
            part.reverse();
        }
    });

    parts.join(&'#')
}
