use std::collections::{HashMap, HashSet, VecDeque};
use crate::graph::Graph;

pub fn analyze_paths(graph: &Graph) {
    // Use all nodes in the graph instead of just 10
    let nodes: Vec<_> = graph.keys().collect();
    let mut total = 0;
    let mut count = 0;

    for &start in &nodes {
        let dist_map = bfs(graph, start);
        for (_, dist) in dist_map {
            total += dist;
            count += 1;
        }
    }

    if count > 0 {
        println!("Average shortest path length: {:.2}", total as f64 / count as f64);
    } else {
        println!("No paths to analyze.");
    }
}

pub fn bfs(graph: &Graph, start: &str) -> HashMap<String, usize> {
    let mut visited = HashSet::new();
    let mut dist = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back((start.to_string(), 0));
    visited.insert(start.to_string());

    while let Some((u, d)) = queue.pop_front() {
        dist.insert(u.clone(), d);
        if let Some(neighbors) = graph.get(&u) {
            for v in neighbors {
                if !visited.contains(v) {
                    visited.insert(v.clone());
                    queue.push_back((v.clone(), d + 1));
                }
            }
        }
    }

    dist
}
