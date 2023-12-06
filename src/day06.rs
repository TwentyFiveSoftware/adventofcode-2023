use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/06.txt").unwrap();

    let lines = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("PART 1: {}", part1(&lines));
    println!("PART 2: {}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> i64 {
    let times = lines.get(0).unwrap()[10..].split(" ")
        .map(|number| number.trim()).filter(|number| !number.is_empty())
        .map(|number| number.parse::<i64>().unwrap());

    let distances = lines.get(1).unwrap()[10..].split(" ")
        .map(|number| number.trim()).filter(|number| !number.is_empty())
        .map(|number| number.parse::<i64>().unwrap());

    zip(times, distances)
        .map(|(time, distance)| Race { time, distance })
        .map(|race| race.calculate_number_of_ways_to_win())
        .fold(1, |acc, n| acc * n)
}

fn part2(lines: &Vec<String>) -> i64 {
    let time = lines.get(0).unwrap()[10..].replace(" ", "").parse::<i64>().unwrap();
    let distance = lines.get(1).unwrap()[10..].replace(" ", "").parse::<i64>().unwrap();
    Race { time, distance }.calculate_number_of_ways_to_win()
}

struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn calculate_number_of_ways_to_win(&self) -> i64 {
        // graphs: f(x) = (time - x) * x, g(x) = distance
        // => equation to solve: (time - x) * x > distance <=> -x^2 + time * x - distance > 0
        // using quadratic formula: x_1,x_2 = (-time +/- sqrt(time^2 - 4 * distance)) / -2

        let time = self.time as f64;
        let distance = self.distance as f64;

        let d = (time * time - 4.0 * distance).sqrt();

        let x_1: f64 = (-time + d) / -2.0;
        let x_2: f64 = (-time - d) / -2.0;

        let min_x = (x_1 + 1.0).floor();
        let max_x = (x_2 - 1.0).ceil();

        max_x as i64 - min_x as i64 + 1
    }
}
