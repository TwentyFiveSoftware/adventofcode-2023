use std::collections::{HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/21.txt").unwrap();

    let lines = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("PART 1: {}", part1(&lines));
}

fn part1(lines: &Vec<String>) -> usize {
    let tiles = lines.iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let height = tiles.len() as i32;
    let width = tiles.get(0).unwrap().len() as i32;

    let (start_x, start_y) = tiles.iter().enumerate()
        .find_map(|(y, row)| row.iter().enumerate()
            .find_map(|(x, c)| {
                if *c == 'S' {
                    return Some((x, y));
                }
                None
            })
        ).unwrap();

    const MAX_STEP_COUNT: u32 = 64;

    let mut reachable_tiles = HashSet::new();

    let mut queue = vec![(start_x as i32, start_y as i32, 0)];
    let mut evaluated_combinations = HashSet::new();

    while let Some((x, y, steps)) = queue.pop() {
        let adjacent_tiles =
            vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].iter()
                .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < width && *y < height)
                .filter(|(x, y)| *tiles.get(*y as usize).unwrap().get(*x as usize).unwrap() != '#')
                .map(|(x, y)| (*x, *y, steps + 1))
                .collect::<Vec<_>>();

        if steps == MAX_STEP_COUNT - 1 {
            for (adjacent_tile_x, adjacent_tile_y, _) in adjacent_tiles {
                reachable_tiles.insert((adjacent_tile_x, adjacent_tile_y));
            }

            continue;
        }

        for adjacent_tile in adjacent_tiles {
            if evaluated_combinations.contains(&adjacent_tile) {
                continue;
            }

            queue.push(adjacent_tile);
            evaluated_combinations.insert(adjacent_tile);
        }
    }

    reachable_tiles.len()
}
