
use crate::graph::Graph;
use std::collections::{HashSet, VecDeque};

pub fn count_clusters(graph: &Graph) {
    let mut visited = HashSet::new();
    let mut cluster_count = 0;

    for node in graph.keys() {
        if !visited.contains(node) {
            cluster_count += 1;
            let mut queue = VecDeque::new();
            queue.push_back(node);

            while let Some(curr) = queue.pop_front() {
                if visited.insert(curr.clone()) {
                    if let Some(neighbors) = graph.get(curr) {
                        for neighbor in neighbors {
                            if !visited.contains(neighbor) {
                                queue.push_back(neighbor);
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Number of connected clusters: {}", cluster_count);
}
