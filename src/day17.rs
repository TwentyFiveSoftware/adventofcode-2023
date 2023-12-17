use std::cmp::{Ordering};
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/17.txt").unwrap();

    let lines = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("PART 1: {}", part1(&lines));
    println!("PART 2: {}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> u32 {
    Grid::new(lines).find_path_with_min_heat_loss(0, 3)
}

fn part2(lines: &Vec<String>) -> u32 {
    Grid::new(lines).find_path_with_min_heat_loss(3, 10)
}

struct Grid {
    grid: Vec<Vec<u8>>,
    width: i32,
    height: i32,
}

impl Grid {
    fn new(lines: &Vec<String>) -> Grid {
        let grid = lines.iter()
            .map(|row| row
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
            )
            .collect::<Vec<Vec<u8>>>();

        let height = grid.len() as i32;
        let width = grid.first().unwrap().len() as i32;

        Grid { grid, width, height }
    }

    fn find_path_with_min_heat_loss(&self, allow_turning_after: usize, allow_straight_until: usize) -> u32 {
        let nodes = self.build_path_graph(allow_turning_after, allow_straight_until);
        let min_heat_losses = self.find_min_heat_losses(&nodes, vec![
            NodeID { x: 0, y: 0, direction: Direction::RIGHT, straight_blocks_count: 1 },
            NodeID { x: 0, y: 0, direction: Direction::DOWN, straight_blocks_count: 1 },
        ]);

        nodes.keys()
            .filter(|node| node.x == self.width - 1 && node.y == self.height - 1)
            .map(|end_node| *min_heat_losses.get(end_node).unwrap_or(&u32::MAX))
            .min()
            .unwrap()
    }

    fn get_heat_loss(&self, x: i32, y: i32) -> u8 {
        *self.grid.get(y as usize).unwrap().get(x as usize).unwrap()
    }

    fn build_path_graph(&self, allow_turning_after: usize, allow_straight_until: usize) -> HashMap<NodeID, Vec<NodeID>> {
        let mut nodes = HashMap::new();

        let mut queue = vec![
            NodeID { x: 0, y: 0, direction: Direction::RIGHT, straight_blocks_count: 1 },
            NodeID { x: 0, y: 0, direction: Direction::DOWN, straight_blocks_count: 1 },
        ];

        while !queue.is_empty() {
            let node = queue.pop().unwrap();
            if nodes.contains_key(&node) {
                continue;
            }

            let mut possible_directions = vec![];

            if node.straight_blocks_count < allow_straight_until {
                possible_directions.push(node.direction);
            }

            if node.straight_blocks_count > allow_turning_after {
                possible_directions.append(&mut node.direction.get_turn_directions());
            }

            let mut connected_nodes = possible_directions.iter()
                .filter_map(|direction| {
                    let (x, y) = direction.get_position_in_direction(node.x, node.y);
                    if x < 0 || y < 0 || x >= self.width || y >= self.height {
                        return None;
                    }

                    let straight_blocks_count = if *direction == node.direction {
                        node.straight_blocks_count + 1
                    } else { 1 };

                    Some(NodeID { x, y, direction: *direction, straight_blocks_count })
                })
                .collect::<Vec<NodeID>>();

            nodes.insert(node, connected_nodes.clone());
            queue.append(&mut connected_nodes);
        }

        nodes
    }

    fn find_min_heat_losses(&self, nodes: &HashMap<NodeID, Vec<NodeID>>, start_nodes: Vec<NodeID>) -> HashMap<NodeID, u32> {
        #[derive(Copy, Clone, Eq, PartialEq)]
        struct State {
            heat_loss: u32,
            node: NodeID,
        }

        impl PartialOrd<Self> for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                other.heat_loss.cmp(&self.heat_loss).then_with(|| self.node.cmp(&other.node))
            }
        }

        let mut heap = BinaryHeap::new();
        let mut min_heat_losses = HashMap::new();

        for start_node in &start_nodes {
            heap.push(State { heat_loss: 0, node: *start_node });
            min_heat_losses.insert(*start_node, 0);
        }

        while let Some(state) = heap.pop() {
            if min_heat_losses.contains_key(&state.node) && *min_heat_losses.get(&state.node).unwrap() < state.heat_loss {
                continue;
            }

            for connected_node in nodes.get(&state.node).unwrap() {
                let delta = self.get_heat_loss(connected_node.x, connected_node.y) as u32;
                let connection_state = State { node: *connected_node, heat_loss: state.heat_loss + delta };

                if !min_heat_losses.contains_key(connected_node) || *min_heat_losses.get(connected_node).unwrap() > connection_state.heat_loss {
                    heap.push(connection_state);

                    min_heat_losses.entry(*connected_node)
                        .and_modify(|heat_loss| *heat_loss = connection_state.heat_loss)
                        .or_insert(connection_state.heat_loss);
                }
            }
        }

        min_heat_losses
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct NodeID {
    x: i32,
    y: i32,
    direction: Direction,
    straight_blocks_count: usize,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn get_turn_directions(&self) -> Vec<Direction> {
        match self {
            Direction::UP | Direction::DOWN => vec![Direction::LEFT, Direction::RIGHT],
            Direction::LEFT | Direction::RIGHT => vec![Direction::UP, Direction::DOWN],
        }
    }

    fn get_position_in_direction(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Direction::UP => (x, y - 1),
            Direction::DOWN => (x, y + 1),
            Direction::LEFT => (x - 1, y),
            Direction::RIGHT => (x + 1, y),
        }
    }
}
