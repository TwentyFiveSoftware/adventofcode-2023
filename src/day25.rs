use std::cmp::{min, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use rayon::prelude::*;

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/25.txt").unwrap();

    let lines = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("PART 1: {}", part1(&lines));
}

fn part1(lines: &Vec<String>) -> usize {
    let graph = Graph::from(lines);

    let local_edge_usages = (0..graph.nodes.len())
        .into_par_iter()
        .map(|i| {
            let mut local_edge_usage = HashMap::new();

            let start_node = graph.nodes.get(i).unwrap();
            let shortest_paths = graph.find_shortest_paths_from_node(start_node);

            for j in i + 1..graph.nodes.len() {
                let mut current_node = graph.nodes.get(j).unwrap().as_str();

                while current_node != start_node {
                    let predecessor = shortest_paths.get(current_node).unwrap();

                    let (from, to) = (predecessor.to_string(), current_node.to_string());

                    local_edge_usage.entry(get_unique_edge(from, to))
                        .and_modify(|count| *count += 1)
                        .or_insert(1);

                    current_node = predecessor;
                }
            }

            local_edge_usage
        })
        .collect::<Vec<_>>();

    let mut edge_usage = HashMap::new();

    for local_edge_usage in local_edge_usages {
        for (edge, local_count) in local_edge_usage {
            edge_usage.entry(edge)
                .and_modify(|count| *count += local_count)
                .or_insert(local_count);
        }
    }

    let mut entries = edge_usage.iter().collect::<Vec<_>>();
    entries.sort_unstable_by_key(|(_edge, count)| *count);
    entries.reverse();

    let edges_by_priority = entries.iter()
        .map(|((from, to), _)| (from.to_string(), to.to_string()))
        .collect::<Vec<_>>();
    let node_set = graph.nodes.clone().into_iter().collect::<HashSet<String>>();

    for i in 0..min(edges_by_priority.len(), 10) {
        for j in i + 1..min(edges_by_priority.len(), 10) {
            for k in j + 1..min(edges_by_priority.len(), 10) {
                let modified_graph = graph
                    .without_edge(edges_by_priority.get(i).unwrap())
                    .without_edge(edges_by_priority.get(j).unwrap())
                    .without_edge(edges_by_priority.get(k).unwrap());

                let connected_nodes = modified_graph.find_connected_nodes(graph.nodes.first().unwrap());
                let remaining_nodes = node_set.difference(&connected_nodes)
                    .map(|node| node.to_string()).collect::<HashSet<String>>();

                if remaining_nodes.is_empty() {
                    continue;
                }

                let other_connected_nodes = modified_graph.find_connected_nodes(remaining_nodes.iter().next().unwrap());
                if remaining_nodes.difference(&other_connected_nodes).count() == 0 {
                    return connected_nodes.len() * other_connected_nodes.len();
                }
            }
        }
    }

    0
}

fn get_unique_edge(from: String, to: String) -> (String, String) {
    if from > to {
        (to, from)
    } else {
        (from, to)
    }
}

struct Graph {
    nodes: Vec<String>,
    edges: HashSet<(String, String)>,
}

impl From<&Vec<String>> for Graph {
    fn from(lines: &Vec<String>) -> Self {
        let mut nodes = HashSet::new();
        let mut edges = HashSet::new();

        for line in lines {
            let mut split = line.split(": ").into_iter();
            let from = split.next().unwrap();
            let to_nodes = split.last().unwrap().split(" ").into_iter();

            nodes.insert(from.to_string());

            for to in to_nodes {
                nodes.insert(to.to_string());
                edges.insert(get_unique_edge(from.to_string(), to.to_string()));
            }
        }

        Graph { nodes: nodes.into_iter().collect(), edges }
    }
}

impl Graph {
    fn without_edge(&self, edge: &(String, String)) -> Graph {
        let mut new_edges = self.edges.clone();
        new_edges.remove(edge);

        Graph {
            nodes: self.nodes.clone(),
            edges: new_edges,
        }
    }

    fn get_neighbours(&self, node: &str) -> Vec<&str> {
        self.edges.iter()
            .filter_map(|(from, to)|
                if from == node {
                    Some(to.as_str())
                } else if to == node {
                    Some(from.as_str())
                } else {
                    None
                }
            )
            .collect()
    }

    fn find_connected_nodes(&self, start_node: &str) -> HashSet<String> {
        let mut connected_nodes = HashSet::new();

        let mut queue = Vec::from([start_node]);
        while let Some(node) = queue.pop() {
            if connected_nodes.contains(node) {
                continue;
            }

            connected_nodes.insert(node.to_string());

            for neighbour in self.get_neighbours(node) {
                queue.push(neighbour);
            }
        }

        connected_nodes
    }

    fn find_shortest_paths_from_node<'a>(&'a self, start_node: &'a str) -> HashMap<&'a str, &'a str> {
        #[derive(Copy, Clone, Eq, PartialEq)]
        struct State<'a> {
            distance: u32,
            node: &'a str,
        }

        impl PartialOrd<Self> for State<'_> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for State<'_> {
            fn cmp(&self, other: &Self) -> Ordering {
                other.distance.cmp(&self.distance).then_with(|| self.node.cmp(&other.node))
            }
        }

        let mut heap = BinaryHeap::new();
        let mut distances = HashMap::new();
        let mut predecessors = HashMap::new();

        heap.push(State { distance: 0, node: start_node });
        distances.insert(start_node, 0);
        predecessors.insert(start_node, "");

        while let Some(state) = heap.pop() {
            if distances.contains_key(&state.node) && *distances.get(&state.node).unwrap() < state.distance {
                continue;
            }

            for connected_node in self.get_neighbours(state.node) {
                let connected_node_state = State { node: connected_node, distance: state.distance + 1 };

                if !distances.contains_key(connected_node) || *distances.get(connected_node).unwrap() > connected_node_state.distance {
                    heap.push(connected_node_state);

                    distances.entry(connected_node)
                        .and_modify(|distance| *distance = connected_node_state.distance)
                        .or_insert(connected_node_state.distance);

                    predecessors.entry(connected_node)
                        .and_modify(|predecessor| *predecessor = state.node)
                        .or_insert(state.node);
                }
            }
        }

        predecessors
    }
}
