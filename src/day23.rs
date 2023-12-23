use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use rayon::prelude::*;

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/23.txt").unwrap();

    let lines = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("PART 1: {}", part1(&lines));
    println!("PART 2: {}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> usize {
    let map = Map::new(lines);

    let mut max_path_length = 0;

    let mut queue = vec![(map.start_x, 0, HashSet::from([(map.start_x, 0)]))];

    while let Some((x, y, path)) = queue.pop() {
        if y == map.height - 1 && x == map.end_x {
            max_path_length = max(max_path_length, path.len() - 1);
            continue;
        }

        let mut new_paths =
            vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].iter()
                .filter(|(x, y)| map.is_in_bounds(*x, *y))
                .filter(|(next_x, next_y)| {
                    let tile = *map.tiles.get(*next_y as usize).unwrap().get(*next_x as usize).unwrap();

                    !(
                        tile == '#' ||
                            (*next_x == x + 1 && tile == '<') ||
                            (*next_x == x - 1 && tile == '>') ||
                            (*next_y == y + 1 && tile == '^') ||
                            (*next_y == y - 1 && tile == 'v')
                    )
                })
                .filter(|(x, y)| !path.contains(&(*x, *y)))
                .map(|(x, y)| {
                    let mut new_path = path.clone();
                    new_path.insert((*x, *y));

                    (*x, *y, new_path)
                })
                .collect::<Vec<_>>();

        queue.append(&mut new_paths);
    }

    max_path_length
}

fn part2(lines: &Vec<String>) -> usize {
    let map = Map::new(lines);
    let graph = map.compress_to_graph();

    graph.find_longest_path_distance((map.start_x, 0), (map.end_x, map.height - 1), HashSet::new()).unwrap()
}

struct Map {
    tiles: Vec<Vec<char>>,
    width: i32,
    height: i32,
    start_x: i32,
    end_x: i32,
}

impl Map {
    fn new(lines: &Vec<String>) -> Map {
        let tiles = lines.iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        Map {
            width: tiles.first().unwrap().len() as i32,
            height: tiles.len() as i32,
            start_x: tiles.first().unwrap().iter().position(|c| *c != '#').unwrap() as i32,
            end_x: tiles.last().unwrap().iter().position(|c| *c != '#').unwrap() as i32,
            tiles,
        }
    }

    fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.height
    }

    fn is_wall(&self, x: i32, y: i32) -> bool {
        *self.tiles.get(y as usize).unwrap().get(x as usize).unwrap() == '#'
    }

    fn compress_to_graph(&self) -> Graph {
        let mut graph = Graph { edges: HashMap::new() };

        let mut visited_junctions = HashSet::new();
        let mut junction_queue = vec![(self.start_x, 0)];

        while let Some((x, y)) = junction_queue.pop() {
            if visited_junctions.contains(&(x, y)) {
                continue;
            }

            visited_junctions.insert((x, y));

            let next_junctions = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                .into_iter()
                .filter(|(x, y)| self.is_in_bounds(*x, *y) && !self.is_wall(*x, *y))
                .filter_map(|(next_x, next_y)| self.find_next_junction(next_x, next_y, (x, y)))
                .collect::<Vec<_>>();

            for next_junction in next_junctions {
                let (next_junction_x, next_junction_y, distance) = next_junction;

                junction_queue.push((next_junction_x, next_junction_y));

                graph.update_distance((x, y), (next_junction_x, next_junction_y), distance);
                graph.update_distance((next_junction_x, next_junction_y), (x, y), distance);
            }
        }

        graph
    }

    fn find_next_junction(&self, mut x: i32, mut y: i32, current_position: (i32, i32)) -> Option<(i32, i32, usize)> {
        let mut path = HashSet::from([current_position]);

        loop {
            path.insert((x, y));

            if y == self.height - 1 && x == self.end_x {
                return Some((x, y, path.len() - 1));
            }

            let next_tiles =
                vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                    .into_iter()
                    .filter(|(x, y)| self.is_in_bounds(*x, *y) && !self.is_wall(*x, *y))
                    .filter(|(x, y)| !path.contains(&(*x, *y)))
                    .collect::<Vec<_>>();

            match next_tiles.len() {
                0 => {
                    return None;
                }
                1 => {
                    let (new_x, new_y) = next_tiles[0];
                    x = new_x;
                    y = new_y;
                }
                _ => {
                    return Some((x, y, path.len() - 1));
                }
            }
        }
    }
}

struct Graph {
    edges: HashMap<(i32, i32), HashMap<(i32, i32), usize>>,
}

impl Graph {
    fn update_distance(&mut self, from: (i32, i32), to: (i32, i32), distance: usize) {
        self.edges.entry(from)
            .and_modify(|connections| {
                connections.entry(to)
                    .and_modify(|curr_distance| *curr_distance = max(distance, *curr_distance))
                    .or_insert(distance);
            })
            .or_insert(HashMap::from([(to, distance)]));
    }

    fn find_longest_path_distance(&self, start: (i32, i32), end: (i32, i32), path: HashSet<(i32, i32)>) -> Option<usize> {
        if start == end {
            return Some(0);
        }

        self.edges.get(&start).unwrap().into_par_iter()
            .filter(|(next, _)| !path.contains(next))
            .filter_map(|(next, distance)| {
                let mut new_path = path.clone();
                new_path.insert(*next);

                match self.find_longest_path_distance(*next, end, new_path) {
                    None => None,
                    Some(remaining_distance) => Some(distance + remaining_distance),
                }
            })
            .max()
    }
}
