use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/15.txt").unwrap();

    let lines = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("PART 1: {}", part1(&lines));
    println!("PART 2: {}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> u64 {
    lines.first().unwrap().split(",").map(hash).sum::<u64>()
}

fn part2(lines: &Vec<String>) -> u64 {
    lines.first().unwrap().split(",")
        .map(|s| Instruction::from_str(s).unwrap())
        .fold(HashMap::new(), |mut lens_boxes, instruction| {
            let lens_box = lens_boxes.entry(instruction.lens_box_number()).or_insert(Vec::new());

            match instruction {
                Instruction::ADD(lens) => {
                    match lens_box.iter().position(|l: &Lens| l.label == lens.label) {
                        None => {
                            lens_box.push(lens);
                        }
                        Some(index) => {
                            lens_box.remove(index);
                            lens_box.insert(index, lens);
                        }
                    }
                }
                Instruction::REMOVE(label) => {
                    lens_box.retain(|lens| lens.label != label)
                }
            }

            lens_boxes
        })
        .iter()
        .map(|(lens_box_number, lenses)| {
            lenses.iter()
                .enumerate()
                .map(|(slot, lens)| (lens_box_number + 1) * (slot as u64 + 1) * lens.focal_length)
                .sum::<u64>()
        })
        .sum()
}

struct Lens {
    label: String,
    focal_length: u64,
}

enum Instruction {
    ADD(Lens),
    REMOVE(String),
}

impl Instruction {
    fn lens_box_number(&self) -> u64 {
        match self {
            Instruction::ADD(lens) => hash(&lens.label),
            Instruction::REMOVE(label) => hash(label)
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with('-') {
            return Ok(Instruction::REMOVE(String::from(&s[..s.len() - 1])));
        }

        return Ok(Instruction::ADD(Lens {
            label: String::from(s.split("=").next().unwrap()),
            focal_length: s.split("=").last().unwrap().parse::<u64>().unwrap(),
        }));
    }
}

fn hash(text: &str) -> u64 {
    text.chars().fold(0, |value, c| ((value + c as u64) * 17) % 256)
}
