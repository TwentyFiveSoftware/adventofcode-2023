use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/24.txt").unwrap();

    let lines = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("PART 1: {}", part1(&lines));
}

fn part1(lines: &Vec<String>) -> usize {
    let hailstones = lines.iter()
        .map(|line| Hailstone::from_str(line).unwrap())
        .collect::<Vec<_>>();

    const TEST_AREA_MIN: f64 = 200000000000000.0;
    const TEST_AREA_MAX: f64 = 400000000000000.0;

    let mut count = 0;

    for i in 0..hailstones.len() {
        for j in (i + 1)..hailstones.len() {
            let (s, t, (x, y, _)) = hailstones.get(i).unwrap().calculate_intersection(hailstones.get(j).unwrap());

            if s < 0.0 || t < 0.0 {
                continue;
            }

            if x < TEST_AREA_MIN || TEST_AREA_MAX < x || y < TEST_AREA_MIN || TEST_AREA_MAX < y {
                continue;
            }

            count += 1;
        }
    }

    count
}

struct Hailstone {
    position: (f64, f64, f64),
    velocity: (f64, f64, f64),
}

impl Hailstone {
    fn calculate_intersection(&self, other: &Hailstone) -> (f64, f64, (f64, f64, f64)) {
        let (x, y, _) = self.position;
        let (dx, dy, _) = self.velocity;

        let (a, b, _) = other.position;
        let (da, db, _) = other.velocity;

        let t = (b - y - (dy / dx) * (a - x)) / (dy * da / dx - db);
        let s = (a - x + da * t) / dx;

        (s, t, other.at(t))
    }

    fn at(&self, t: f64) -> (f64, f64, f64) {
        let (x, y, z) = self.position;
        let (dx, dy, dz) = self.velocity;

        (x + dx * t, y + dy * t, z + dz * t)
    }
}

impl FromStr for Hailstone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" @ ").into_iter();
        let position = split.next().unwrap().split(", ")
            .map(|n| n.trim().parse::<i64>().unwrap()).collect::<Vec<_>>();
        let velocity = split.last().unwrap().split(", ")
            .map(|n| n.trim().parse::<i64>().unwrap()).collect::<Vec<_>>();

        Ok(Hailstone {
            position: (
                *position.get(0).unwrap() as f64,
                *position.get(1).unwrap() as f64,
                *position.get(2).unwrap() as f64,
            ),
            velocity: (
                *velocity.get(0).unwrap() as f64,
                *velocity.get(1).unwrap() as f64,
                *velocity.get(2).unwrap() as f64,
            ),
        })
    }
}
