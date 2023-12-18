use std::collections::{HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/18.txt").unwrap();

    let lines = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("PART 1: {}", part1(&lines));
}

fn part1(lines: &Vec<String>) -> usize {
    let instructions = lines.iter()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect::<Vec<Instruction>>();

    let mut dug_tiles = HashSet::new();

    let mut current_position = (0, 0);
    for instruction in instructions {
        let tiles = instruction.get_tiles(current_position);

        for tile in &tiles {
            dug_tiles.insert(*tile);
        }

        current_position = *(tiles.last().unwrap());
    }

    let mut queue = vec![(1, 1)];
    while let Some((x, y)) = queue.pop() {
        if dug_tiles.contains(&(x, y)) {
            continue;
        }

        dug_tiles.insert((x, y));

        queue.append(&mut vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]);
    }

    dug_tiles.len()
}

struct Instruction {
    direction: Direction,
    length: i64,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(" ").collect::<Vec<&str>>();

        Ok(Instruction {
            direction: Direction::from_str(split.get(0).unwrap()).unwrap(),
            length: split.get(1).unwrap().parse::<i64>().unwrap(),
        })
    }
}

impl Instruction {
    fn get_tiles(&self, (x, y): (i64, i64)) -> Vec<(i64, i64)> {
        (1..=self.length).map(|delta| {
            match self.direction {
                Direction::UP => (x, y - delta),
                Direction::DOWN => (x, y + delta),
                Direction::LEFT => (x - delta, y),
                Direction::RIGHT => (x + delta, y)
            }
        }).collect()
    }
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::UP),
            "D" => Ok(Direction::DOWN),
            "L" => Ok(Direction::LEFT),
            "R" => Ok(Direction::RIGHT),
            &_ => Err(())
        }
    }
}
