use std::fs::File;
use std::io::BufRead;
use std::collections::VecDeque;
use rand::seq::IteratorRandom;
use std::collections::HashMap;
use rand::rng;

//reads the file into an adjacency list format
fn read_graph(path: &str) -> Vec<Vec<usize>> {
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    
    let mut edges = Vec::new();
    let mut max_node = 0;

    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let parts: Vec<usize> = line_str
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if parts.len() == 2 {
            let (from, to) = (parts[0], parts[1]);
            max_node = max_node.max(from).max(to);
            edges.push((from, to));
        }
    }
    
    let mut adj_list = vec![Vec::new(); max_node+1];
    
    for (from, to) in edges {
        adj_list[from].push(to)
    }
    adj_list
}

// Performs BFS from a given start node, returning a vector of shortest distances.
fn bfs(adj_list: &Vec<Vec<usize>>, start: usize) -> Vec<Option<usize>> {
    let n = adj_list.len();
    let mut visited = vec![false; n];
    let mut distance = vec![None; n];
    let mut queue = VecDeque::new();

    visited[start] = true;
    distance[start] = Some(0);
    queue.push_back(start);

    while let Some(u) = queue.pop_front() {
        for &v in &adj_list[u] {
            if !visited[v] {
                visited[v] = true;
                distance[v] = Some(distance[u].unwrap() + 1);
                queue.push_back(v);
            }
        }
    }

    distance
}

fn avg_distance(adj_list: &Vec<Vec<usize>>, sample_size: usize) -> f64 {
    let mut rng = rng();
    let nodes_with_edges: Vec<usize> = adj_list.iter()
        .enumerate()
        .filter(|(_, neighbors)| !neighbors.is_empty())
        .map(|(i, _)| i)
        .collect();
    
    let samples = nodes_with_edges.iter()
        .copied()
        .choose_multiple(&mut rng, sample_size);

    let mut total_distance = 0;
    let mut count = 0;

    for start in &samples {
        let distances = bfs(adj_list, *start);
        for d in &distances {
            if let Some(dist) = &d {
                if *dist > 0 as usize {
                    total_distance += dist;
                    count += 1;
                }
            }
        }
    }

    total_distance as f64 / count as f64
}

fn avg_distance_histogram(adj_list: &Vec<Vec<usize>>, sample_size: usize) -> HashMap<usize, usize> {
    let mut rng = rng();
    let nodes_with_edges: Vec<usize> = adj_list.iter()
        .enumerate()
        .filter(|(_, neighbors)| !neighbors.is_empty())
        .map(|(i, _)| i)
        .collect();
    
    let samples = nodes_with_edges.iter()
        .copied()
        .choose_multiple(&mut rng, sample_size);

    let mut frequency: HashMap<usize, usize> = HashMap::new();

    for start in &samples {
        let distances = bfs(adj_list, *start);
        for d in &distances {
            if let Some(dist) = &d {
                if *dist > 0 {
                    *frequency.entry(dist).or_insert(0) += 1;
                }

            }
        }
    }

    frequency
}



fn main() {
    let adj_list = read_graph("amazon0302.txt");
    let trial_bfs = avg_distance(&adj_list, 100);
    println!("Average shortest path for 100 samples: {:.2}", trial_bfs);
    let histogram = avg_distance_histogram(&adj_list, 50);
}
