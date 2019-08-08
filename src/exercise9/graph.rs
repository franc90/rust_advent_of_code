use std::collections::HashSet;

use permutohedron::Heap;

#[derive(Debug)]
pub struct Graph {
    edges: Vec<Edge>,
    vertices: HashSet<String>,
}

#[derive(Debug)]
struct Edge {
    from: String,
    to: String,
    weight: u32,
}

impl Edge {
    fn new(from: String, to: String, weight: u32) -> Edge {
        Edge { from, to, weight }
    }
}

impl Graph {
    pub fn load(paths: Vec<(String, String, u32)>) -> Graph {
        let vertices = paths.iter()
            .map(|(from, _, _)| from.clone())
            .chain(
                paths.iter()
                    .map(|(_, to, _)| to.clone())
            ).collect();
        let edges = paths.into_iter()
            .map(|(from, to, weight)| Edge::new(from, to, weight))
            .collect();

        Graph { edges, vertices }
    }

    pub fn get_shortest_distance(&self) -> u32 {
        self.get_max(u32::max_value(), |x, y| x < y)
    }

    pub fn get_longest_distance(&self) -> u32 {
        self.get_max(u32::min_value(), |x, y| x > y)
    }

    fn get_max<P>(&self, start_val: u32, predicate: P) -> u32
        where P: Fn(u32, u32) -> bool {
        let mut best_dist = start_val;

        let mut vertices = self.all_vertices();
        let heap = Heap::new(&mut vertices);
        for permutation in heap {
            let mut curr_dist = 0;
            let mut iter = permutation.iter();
            let mut from = iter.next().unwrap();
            while let Some(to) = iter.next() {
                curr_dist += self.dist(from, to);
                from = to;
            }
            if predicate(curr_dist, best_dist) {
                best_dist = curr_dist;
            }
        }

        best_dist
    }

    fn dist(&self, from: &String, to: &String) -> u32 {
        self.edges.iter()
            .filter(|e| (e.from == *from && e.to == *to) || (e.from == *to && e.to == *from))
            .map(|e| e.weight)
            .next()
            .expect(&format!("No route between {} and {}", from, to))
    }

    fn all_vertices(&self) -> Vec<&String> {
        self.vertices.iter().collect()
    }
}
