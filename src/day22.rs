use std::collections::{HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use rayon::prelude::*;

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/22.txt").unwrap();

    let lines = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("PART 1: {}", part1(&lines));
    println!("PART 2: {}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> usize {
    let mut brick_pile = BrickPile::new(lines);
    brick_pile.simulate();

    let not_safe_to_disintegrate = brick_pile.bricks
        .iter()
        .filter_map(|brick| {
            let supporting_bricks = brick.get_supporting_bricks(&brick_pile.bricks);
            if supporting_bricks.len() == 1 {
                return Some(supporting_bricks.first().unwrap().id);
            }

            None
        })
        .collect::<HashSet<_>>()
        .len();

    brick_pile.bricks.len() - not_safe_to_disintegrate
}

fn part2(lines: &Vec<String>) -> usize {
    let mut brick_pile = BrickPile::new(lines);
    brick_pile.simulate();

    (0..brick_pile.bricks.len())
        .into_par_iter()
        .map(|i| {
            let mut modified_pile = brick_pile.clone();
            modified_pile.bricks.remove(i);

            modified_pile.simulate();

            let moved_brick_count = modified_pile.bricks.iter()
                .filter(|brick| {
                    let original_brick = brick_pile.bricks.iter().find(|b| b.id == brick.id).unwrap();
                    original_brick.cubes != brick.cubes
                })
                .count();

            moved_brick_count
        })
        .sum()
}

#[derive(Clone)]
struct BrickPile {
    bricks: Vec<Brick>,
}

impl BrickPile {
    fn new(lines: &Vec<String>) -> BrickPile {
        let bricks = lines
            .iter()
            .map(|line| Brick::from_str(line).unwrap())
            .enumerate()
            .map(|(i, brick)| Brick { cubes: brick.cubes, id: i })
            .collect::<Vec<Brick>>();

        BrickPile { bricks }
    }

    fn simulate(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;

            for i in 0..self.bricks.len() {
                while !self.bricks[i].is_on_ground() && !self.bricks[i].is_supported(&self.bricks) {
                    self.bricks[i].move_one_down();
                    changed = true;
                }
            }
        }
    }
}

#[derive(Clone)]
struct Brick {
    id: usize,
    cubes: HashSet<(i32, i32, i32)>,
}

impl Brick {
    fn is_on_ground(&self) -> bool {
        self.cubes.iter().any(|(_, _, z)| *z == 1)
    }

    fn is_supported(&self, bricks: &Vec<Brick>) -> bool {
        self.get_supporting_bricks(bricks).len() > 0
    }

    fn get_supporting_bricks<'a>(&self, bricks: &'a Vec<Brick>) -> Vec<&'a Brick> {
        let min_z = self.cubes.iter().map(|(_, _, z)| z).min().unwrap();

        let possible_supports = self.cubes.iter()
            .filter(|(_, _, z)| z == min_z)
            .map(|(x, y, z)| (*x, *y, *z - 1))
            .collect::<HashSet<_>>();

        let other_bricks = bricks.iter()
            .filter(|brick| brick.cubes.intersection(&self.cubes).count() == 0);

        other_bricks
            .filter(|other_brick| other_brick.cubes.intersection(&possible_supports).count() > 0)
            .collect()
    }

    fn move_one_down(&mut self) {
        self.cubes = self.cubes.iter().map(|(x, y, z)| (*x, *y, *z - 1)).collect();
    }
}

impl FromStr for Brick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("~");
        let from = split.next().unwrap();
        let to = split.last().unwrap();

        let from_coordinates = from.split(",")
            .map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let to_coordinates = to.split(",")
            .map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let from_x = *from_coordinates.get(0).unwrap();
        let from_y = *from_coordinates.get(1).unwrap();
        let from_z = *from_coordinates.get(2).unwrap();

        let to_x = *to_coordinates.get(0).unwrap();
        let to_y = *to_coordinates.get(1).unwrap();
        let to_z = *to_coordinates.get(2).unwrap();

        let mut cubes = HashSet::new();
        for x in from_x..=to_x {
            for y in from_y..=to_y {
                for z in from_z..=to_z {
                    cubes.insert((x, y, z));
                }
            }
        }

        Ok(Brick { cubes, id: 0 })
    }
}
