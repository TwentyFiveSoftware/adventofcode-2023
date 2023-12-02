use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/02.txt").unwrap();
    let input = BufReader::new(input_file);

    let lines = input
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("PART 1: {}", part1(&lines));
    println!("PART 2: {}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> i32 {
    lines
        .iter()
        .map(|line| get_max_cube_amount_per_color(line))
        .filter_map(|(game_id, (max_r, max_g, max_b))| {
            let is_game_possible = max_r <= 12 && max_g <= 13 && max_b <= 14;
            return if is_game_possible { Some(game_id) } else { None };
        })
        .sum()
}

fn part2(lines: &Vec<String>) -> i32 {
    lines
        .iter()
        .map(|line| get_max_cube_amount_per_color(line))
        .map(|(_, (max_r, max_g, max_b))| {
            max_r * max_g * max_b
        })
        .sum()
}

fn get_max_cube_amount_per_color(line: &str) -> (i32, (i32, i32, i32)) {
    let line_regex = Regex::new(r"^Game ([0-9]+):(.*)$").unwrap();

    let groups = line_regex.captures(line.as_ref()).unwrap();
    let game_id = groups.get(1).unwrap().as_str().parse::<i32>().unwrap();

    let max_cube_count_per_color = groups.get(2).unwrap().as_str()
        .split(";")
        .map(|cube_set| {
            cube_set
                .split(",")
                .map(|cubes| cubes.trim())
                .fold((0, 0, 0), |(r, g, b), cubes| {
                    let split = cubes.split(" ").collect::<Vec<&str>>();
                    let cube_amount = split[0].parse::<i32>().unwrap();
                    let cube_color = split[1];

                    match cube_color {
                        "red" => (cube_amount, g, b),
                        "green" => (r, cube_amount, b),
                        "blue" => (r, g, cube_amount),
                        _ => (r, g, b)
                    }
                })
        })
        .fold((0, 0, 0), |(max_r, max_g, max_b), (r, g, b)| {
            (max(max_r, r), max(max_g, g), max(max_b, b))
        });

    (game_id, max_cube_count_per_color)
}
