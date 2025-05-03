
use crate::graph::Graph;

pub fn approximate_densest(graph: &Graph) {
    let mut nodes: Vec<String> = graph.keys().cloned().collect();
    let mut g = graph.clone();
    let mut best_density = 0.0;

    while !nodes.is_empty() {
        let edge_count: usize = g.values().map(|v| v.len()).sum();
        let n = g.len();
        if n == 0 { break; }

        let density = edge_count as f64 / n as f64;
        if density > best_density {
            best_density = density;
        }

        // Find node with the smallest degree safely
        if let Some(node) = nodes.iter()
            .min_by_key(|n| g.get(*n).map_or(0, |neighbors| neighbors.len()))
            .cloned() // clone because `nodes` is Vec<String>
        {
            g.remove(&node);
            for neighbors in g.values_mut() {
                neighbors.retain(|x| x != &node);
            }
            nodes.retain(|x| x != &node);
        } else {
            break;
        }
    }

    println!("Approximate densest subgraph density: {:.2}", best_density);
}
