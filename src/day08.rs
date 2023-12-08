use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::str::FromStr;
use regex::Regex;

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/08.txt").unwrap();

    let mut input = String::new();
    BufReader::new(input_file).read_to_string(&mut input).unwrap();

    let map = Map::from_str(&input).unwrap();

    println!("PART 1: {}", part1(&map));
    println!("PART 2: {}", part2(&map));
}

fn part1(map: &Map) -> u64 {
    map.calculate_path_length("AAA", &vec!["ZZZ"])
}

fn part2(map: &Map) -> u64 {
    let end_nodes = map.graph.keys().filter(|node| node.ends_with("Z"))
        .map(|node| &node[..]).collect::<Vec<&str>>();

    map.graph.keys()
        .filter(|node| node.ends_with("A"))
        .fold(1, |lcm_path_length, start_node| {
            lcm(map.calculate_path_length(start_node, &end_nodes), lcm_path_length)
        })
}

struct Map {
    graph: HashMap<String, (String, String)>,
    instructions: Vec<char>,
}

impl Map {
    fn calculate_path_length(&self, start_node: &str, end_nodes: &Vec<&str>) -> u64 {
        let mut current_node = start_node;
        let mut current_instruction_index = 0;
        let mut path_length = 0;

        while !end_nodes.contains(&current_node) {
            let (left_node, right_node) = self.graph.get(current_node).unwrap();

            current_node = match self.instructions.get(current_instruction_index).unwrap() {
                'L' => left_node,
                'R' => right_node,
                _ => current_node
            };

            path_length += 1;
            current_instruction_index = (current_instruction_index + 1) % self.instructions.len();
        }

        path_length
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let graph_line_regex = Regex::new(r"^([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)$").unwrap();

        let instructions = s.lines().next().unwrap().chars().collect::<Vec<char>>();

        let graph = s.lines().skip(2)
            .fold(HashMap::new(), |mut graph, line| {
                let groups = graph_line_regex.captures(line).unwrap();

                let node_id = String::from_str(groups.get(1).unwrap().as_str()).unwrap();
                let left_child = String::from_str(groups.get(2).unwrap().as_str()).unwrap();
                let right_child = String::from_str(groups.get(3).unwrap().as_str()).unwrap();

                graph.insert(node_id, (left_child, right_child));
                graph
            });

        Ok(Map { graph, instructions })
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    return gcd(b, a % b);
}
