
mod graph;
mod shortest_path;
mod centrality;
mod clustering;
mod densest;

use graph::build_graph;

fn main() {
    let items = graph::read_dataset("abc_xyz_dataset.csv");
    let g = build_graph(&items, 3); // Using top-3 similar neighbors

    shortest_path::analyze_paths(&g);
    centrality::compute_closeness(&g);
    clustering::count_clusters(&g);
    densest::approximate_densest(&g);
}

#[cfg(test)]
mod tests {
    fn cosine_similarity(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
        let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
        let norm_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot / (norm_a * norm_b)
        }
    }

    #[test]
    fn test_cosine_similarity_identical() {
        let a = vec![1.0, 2.0];
        let b = vec![1.0, 2.0];
        let result = cosine_similarity(&a, &b);
        assert!((result - 1.0).abs() < 1e-6);
    }
}
