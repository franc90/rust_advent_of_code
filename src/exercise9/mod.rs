use std::fs;

use graph::Graph;

mod graph;


pub fn run() {
    let input = fs::read_to_string("resources/2015/ex9_in")
        .expect("Couldn't read input");

    let graph = Graph::load(to_triples(input));
    eprintln!("shortest_dist = {:?}", graph.get_shortest_distance());
    eprintln!("longest_dist = {:?}", graph.get_longest_distance());
}

fn to_triples(input: String) -> Vec<(String, String, u32)> {
    let mut triples = Vec::new();
    for line in input.lines() {
        let vec: Vec<&str> = line.split(" = ").collect();
        if let [nodes, weight] = &*vec {
            let nodes: Vec<&str> = nodes.split(" to ").collect();
            if let [from, to] = &*nodes {
                let weight = weight.parse::<u32>()
                    .unwrap_or_else(|_| panic!("Could not parse '{}' to u32'", weight));
                triples.push((from.to_string(), to.to_string(), weight));
            }
        }
    }
    triples
}