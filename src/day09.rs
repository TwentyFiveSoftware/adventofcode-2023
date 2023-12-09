use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/09.txt").unwrap();

    let lines = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("PART 1: {}", part1(&lines));
    println!("PART 2: {}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> i64 {
    lines
        .iter()
        .map(|line| Sequence::from_str(line).unwrap())
        .map(|sequence| sequence.extrapolate_next_value())
        .sum()
}

fn part2(lines: &Vec<String>) -> i64 {
    lines
        .iter()
        .map(|line| Sequence::from_str(line).unwrap())
        .map(|sequence| sequence.extrapolate_previous_value())
        .sum()
}

struct Sequence {
    values: Vec<i64>,
}

impl Sequence {
    fn extrapolate_next_value(&self) -> i64 {
        if self.is_all_zero() {
            return 0;
        }

        return self.values.last().unwrap() + self.derivative().extrapolate_next_value();
    }

    fn extrapolate_previous_value(&self) -> i64 {
        if self.is_all_zero() {
            return 0;
        }

        return self.values.first().unwrap() - self.derivative().extrapolate_previous_value();
    }

    fn derivative(&self) -> Sequence {
        let values = (0..self.values.len() - 1)
            .map(|i| self.values.get(i + 1).unwrap() - self.values.get(i).unwrap())
            .collect::<Vec<i64>>();

        Sequence { values }
    }

    fn is_all_zero(&self) -> bool {
        self.values.iter().all(|value| *value == 0)
    }
}

impl FromStr for Sequence {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.split(" ")
            .map(|number| number.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        Ok(Sequence { values })
    }
}
