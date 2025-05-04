use crate::graph::Graph;
use crate::shortest_path::bfs;

pub fn compute_closeness(graph: &Graph) {
    println!("Top closeness centrality:");

    let mut scores = vec![];

    // Compute centrality for each node in the graph
    for node in graph.keys() {
        let dist_map = bfs(graph, node);
        let sum: usize = dist_map.values().sum();

        if sum > 0 {
            scores.push((node, 1.0 / (sum as f64)));
        }
    }
    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Print top 5 central nodes
    for (node, score) in scores.iter().take(5) {
        println!("{}: {:.4}", node, score);
    }
}
