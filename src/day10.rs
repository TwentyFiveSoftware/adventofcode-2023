use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/10.txt").unwrap();

    let lines = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("PART 1: {}", part1(&lines));
}

fn part1(lines: &Vec<String>) -> usize {
    let graph = parse_graph(lines);

    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State {
        distance: usize,
        position: (usize, usize),
    }

    impl PartialOrd<Self> for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.distance.cmp(&self.distance).then_with(|| self.position.cmp(&other.position))
        }
    }

    let mut heap = BinaryHeap::new();
    heap.push(State { distance: 0, position: graph.start_node });

    let mut distances = HashMap::new();
    distances.insert(graph.start_node, 0);

    while let Some(state) = heap.pop() {
        if distances.contains_key(&state.position) && *distances.get(&state.position).unwrap() < state.distance {
            continue;
        }

        for connection in &graph.nodes.get(&state.position).unwrap().connections {
            let connection_state = State { position: *connection, distance: state.distance + 1 };

            if !distances.contains_key(connection) || *distances.get(connection).unwrap() > connection_state.distance {
                heap.push(connection_state);
                distances.entry(*connection)
                    .and_modify(|distance| *distance = connection_state.distance)
                    .or_insert(connection_state.distance);
            }
        }
    }

    *distances.values().max().unwrap()
}

fn parse_graph(lines: &Vec<String>) -> Graph {
    let map = lines.iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let (start_x, start_y) = map.iter().enumerate()
        .find_map(|(y, row)|
            row.iter().enumerate().find_map(|(x, c)| {
                if *c == 'S' {
                    return Some((x, y));
                }
                None
            })
        ).unwrap();

    let start_node = Node {
        connections: vec![
            (start_x - 1, start_y),
            (start_x + 1, start_y),
            (start_x, start_y - 1),
            (start_x, start_y + 1),
        ].iter()
            .filter(|(adjacent_x, adjacent_y)|
                get_node_connections(&map, *adjacent_x, *adjacent_y)
                    .iter()
                    .any(|(x, y)| *x == start_x && *y == start_y)
            )
            .map(|(x, y)| (*x, *y))
            .collect(),
    };

    let mut queue = VecDeque::from(start_node.connections.clone());

    let mut nodes = HashMap::new();
    nodes.insert((start_x, start_y), start_node);

    while let Some((x, y)) = queue.pop_front() {
        if nodes.contains_key(&(x, y)) {
            continue;
        }

        let connections = get_node_connections(&map, x, y);
        for connection in &connections {
            queue.push_back(*connection)
        }

        nodes.insert((x, y), Node { connections });
    }

    Graph { nodes, start_node: (start_x, start_y) }
}

fn get_node_connections(map: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    match map[y][x] {
        '|' => vec![(x, y - 1), (x, y + 1)],
        '-' => vec![(x - 1, y), (x + 1, y)],
        'L' => vec![(x, y - 1), (x + 1, y)],
        'J' => vec![(x, y - 1), (x - 1, y)],
        '7' => vec![(x, y + 1), (x - 1, y)],
        'F' => vec![(x, y + 1), (x + 1, y)],
        _ => vec![],
    }
}

struct Graph {
    nodes: HashMap<(usize, usize), Node>,
    start_node: (usize, usize),
}

struct Node {
    connections: Vec<(usize, usize)>,
}
