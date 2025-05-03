
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
pub struct Item {
    pub id: String,
    pub demand: Vec<i32>,
}

pub type Graph = HashMap<String, Vec<String>>;

pub fn read_dataset(path: &str) -> Vec<Item> {
    let file = File::open(path).expect("CSV file not found");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    lines.next(); // Skip header

    let mut items = Vec::new();
    for line in lines {
        if let Ok(row) = line {
            let cols: Vec<&str> = row.split(',').collect();
            let id = cols[0].to_string();
            let demand: Vec<i32> = cols[3..15].iter().map(|v| v.parse().unwrap_or(0)).collect();
            items.push(Item { id, demand });
        }
    }
    items
}

pub fn build_graph(items: &[Item], k: usize) -> Graph {
    let mut graph = Graph::new();
    for (i, item) in items.iter().enumerate() {
        let mut sims: Vec<(String, f64)> = items.iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .map(|(_, other)| {
                let sim = cosine_similarity(&item.demand, &other.demand);
                (other.id.clone(), sim)
            })
            .collect();
        sims.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let neighbors: Vec<String> = sims.into_iter().take(k).map(|x| x.0).collect();
        graph.insert(item.id.clone(), neighbors);
    }
    graph
}

fn cosine_similarity(v1: &[i32], v2: &[i32]) -> f64 {
    let dot = v1.iter().zip(v2).map(|(a, b)| a * b).sum::<i32>();
    let norm1 = (v1.iter().map(|x| x * x).sum::<i32>() as f64).sqrt();
    let norm2 = (v2.iter().map(|x| x * x).sum::<i32>() as f64).sqrt();
    if norm1 == 0.0 || norm2 == 0.0 { 0.0 } else { dot as f64 / (norm1 * norm2) }
}
