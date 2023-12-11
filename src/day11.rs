use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/11.txt").unwrap();

    let lines = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("PART 1: {}", part1(&lines));
    println!("PART 2: {}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> i64 {
    parse_image(lines, 2).calculate_distance_sum()
}

fn part2(lines: &Vec<String>) -> i64 {
    parse_image(lines, 1000000).calculate_distance_sum()
}

fn parse_image(lines: &Vec<String>, expansion_factor: usize) -> Image {
    let image = lines.iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let rows_without_galaxies = image.iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|c| *c != '#'))
        .map(|(y, _)| y)
        .collect::<Vec<usize>>();

    let columns_without_galaxies = (0..image.get(0).unwrap().len())
        .filter(|x| {
            image.iter().all(|row| *(row.get(*x).unwrap()) != '#')
        })
        .collect::<Vec<usize>>();

    let galaxies = image
        .iter()
        .enumerate()
        .map(|(y, row)| row
            .iter()
            .enumerate()
            .filter_map(move |(x, c)| {
                if *c == '#' {
                    return Some((x, y));
                }
                return None;
            })
        )
        .flatten()
        .filter_map(|(x, y)| {
            let rows_without_galaxy_before = rows_without_galaxies.iter()
                .filter(|row_y| **row_y < y).count();

            let columns_without_galaxy_before = columns_without_galaxies.iter()
                .filter(|column_x| **column_x < x).count();

            return Some((
                x + columns_without_galaxy_before * (expansion_factor - 1),
                y + rows_without_galaxy_before * (expansion_factor - 1),
            ));
        })
        .collect::<Vec<(usize, usize)>>();

    Image { galaxies }
}

struct Image {
    galaxies: Vec<(usize, usize)>,
}

impl Image {
    fn calculate_distance_sum(&self) -> i64 {
        let mut sum = 0;

        for (i, (x_a, y_a)) in self.galaxies.iter().enumerate() {
            for (x_b, y_b) in &self.galaxies[i..] {
                sum += (*x_b as i64 - *x_a as i64).abs() + (*y_b as i64 - *y_a as i64).abs();
            }
        }

        sum
    }
}
