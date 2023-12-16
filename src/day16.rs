use std::collections::{HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/16.txt").unwrap();

    let lines = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("PART 1: {}", part1(&lines));
    println!("PART 2: {}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> usize {
    parse_contraption(lines)
        .calculate_energized_tile_count(Ray { direction: RayDirection::RIGHT, x: 0, y: 0 })
}

fn part2(lines: &Vec<String>) -> usize {
    let contraption = parse_contraption(lines);

    vec![
        (0..contraption.width).map(|x| vec![
            Ray { direction: RayDirection::DOWN, x: x as i32, y: 0 },
            Ray { direction: RayDirection::UP, x: x as i32, y: contraption.height as i32 - 1 },
        ]).flatten().collect::<Vec<Ray>>(),
        (0..contraption.height).map(|y| vec![
            Ray { direction: RayDirection::RIGHT, x: 0, y: y as i32 },
            Ray { direction: RayDirection::LEFT, x: contraption.width as i32 - 1, y: y as i32 },
        ]).flatten().collect::<Vec<Ray>>(),
    ]
        .iter()
        .flatten()
        .map(|start_ray| contraption.calculate_energized_tile_count(*start_ray))
        .max()
        .unwrap()
}

struct Contraption {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Contraption {
    fn calculate_energized_tile_count(&self, start_ray: Ray) -> usize {
        let mut energized_tiles = HashSet::new();

        let mut rays_cache = HashSet::new();
        let mut rays = vec![start_ray];
        while !rays.is_empty() {
            let ray = rays.pop().unwrap();

            if rays_cache.contains(&ray) {
                continue;
            }

            energized_tiles.insert((ray.x, ray.y));
            rays_cache.insert(ray);

            let mut new_rays = self.simulate_ray(ray);
            rays.append(&mut new_rays);
        }

        energized_tiles.len()
    }

    fn simulate_ray(&self, ray: Ray) -> Vec<Ray> {
        let tile = self.tiles.get(ray.y as usize).unwrap().get(ray.x as usize).unwrap();
        let new_rays = tile.simulate_ray(ray);

        new_rays.iter()
            .filter(|new_ray| new_ray.x >= 0 && new_ray.y >= 0 &&
                new_ray.x < self.width as i32 && new_ray.y < self.height as i32)
            .map(|ray| *ray)
            .collect()
    }
}

enum Tile {
    Empty,
    LeftUpMirror,
    LeftDownMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

impl Tile {
    fn simulate_ray(&self, ray: Ray) -> Vec<Ray> {
        match self {
            Tile::Empty => {
                match ray.direction {
                    RayDirection::UP => vec![Ray { direction: ray.direction, x: ray.x, y: ray.y - 1 }],
                    RayDirection::DOWN => vec![Ray { direction: ray.direction, x: ray.x, y: ray.y + 1 }],
                    RayDirection::LEFT => vec![Ray { direction: ray.direction, x: ray.x - 1, y: ray.y }],
                    RayDirection::RIGHT => vec![Ray { direction: ray.direction, x: ray.x + 1, y: ray.y }],
                }
            }
            Tile::LeftUpMirror => {
                match ray.direction {
                    RayDirection::UP => vec![Ray { direction: RayDirection::RIGHT, x: ray.x + 1, y: ray.y }],
                    RayDirection::DOWN => vec![Ray { direction: RayDirection::LEFT, x: ray.x - 1, y: ray.y }],
                    RayDirection::LEFT => vec![Ray { direction: RayDirection::DOWN, x: ray.x, y: ray.y + 1 }],
                    RayDirection::RIGHT => vec![Ray { direction: RayDirection::UP, x: ray.x, y: ray.y - 1 }],
                }
            }
            Tile::LeftDownMirror => {
                match ray.direction {
                    RayDirection::UP => vec![Ray { direction: RayDirection::LEFT, x: ray.x - 1, y: ray.y }],
                    RayDirection::DOWN => vec![Ray { direction: RayDirection::RIGHT, x: ray.x + 1, y: ray.y }],
                    RayDirection::LEFT => vec![Ray { direction: RayDirection::UP, x: ray.x, y: ray.y - 1 }],
                    RayDirection::RIGHT => vec![Ray { direction: RayDirection::DOWN, x: ray.x, y: ray.y + 1 }],
                }
            }
            Tile::HorizontalSplitter => {
                match ray.direction {
                    RayDirection::LEFT | RayDirection::RIGHT => Tile::Empty.simulate_ray(ray),
                    RayDirection::UP | RayDirection::DOWN => vec![
                        Ray { direction: RayDirection::LEFT, x: ray.x - 1, y: ray.y },
                        Ray { direction: RayDirection::RIGHT, x: ray.x + 1, y: ray.y },
                    ],
                }
            }
            Tile::VerticalSplitter => {
                match ray.direction {
                    RayDirection::UP | RayDirection::DOWN => Tile::Empty.simulate_ray(ray),
                    RayDirection::LEFT | RayDirection::RIGHT => vec![
                        Ray { direction: RayDirection::UP, x: ray.x, y: ray.y - 1 },
                        Ray { direction: RayDirection::DOWN, x: ray.x, y: ray.y + 1 },
                    ],
                }
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Ray {
    direction: RayDirection,
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum RayDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn parse_contraption(lines: &Vec<String>) -> Contraption {
    let tiles = lines.iter()
        .map(|line| line.chars()
            .map(|tile| match tile {
                '.' => Tile::Empty,
                '/' => Tile::LeftUpMirror,
                '\\' => Tile::LeftDownMirror,
                '-' => Tile::HorizontalSplitter,
                '|' => Tile::VerticalSplitter,
                _ => Tile::Empty,
            })
            .collect::<Vec<Tile>>())
        .collect::<Vec<Vec<Tile>>>();

    Contraption {
        width: tiles.get(0).unwrap().len(),
        height: tiles.len(),
        tiles,
    }
}
