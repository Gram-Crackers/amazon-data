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

//from normal adjacency list, turns into vector of incoming edges for each node
fn reverse_al(adj_list: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut reversed = vec![Vec::new(); adj_list.len()];

    for (u, neighbors) in adj_list.iter().enumerate() {
        for &v in neighbors {
            reversed[v].push(u);
        }
    }

    reversed
}

//Performs BFS from a given start node, returning a vector of shortest distances from start to each.
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

//I want this to read out to either a csv or write another function to help print the histogram in the terminal
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
                    *frequency.entry(*dist).or_insert(0) += 1;
                }

            }
        }
    }

    frequency
}

fn out_closeness(adj_list: &Vec<Vec<usize>>, start: usize) -> f64 {
    let distances = bfs(adj_list, start);
    let mut count = 0;
    let mut total_distance = 0;

    for d in &distances {
        if let Some(dist) = &d {
            if *dist > 0 as usize {
                total_distance += dist;
                count += 1;
            }
        }
    }
    count as f64 / total_distance as f64
}

//gives a sorted list of closeness in tuple (node, closeness) format
fn get_all_out_closeness(adj_list: &Vec<Vec<usize>>) -> Vec<(usize, f64)> {
    let mut closeness_map: HashMap<usize, f64> = HashMap::new();

    let nodes_with_edges: Vec<usize> = adj_list.iter()
        .enumerate()
        .filter(|(_, neighbors)| !neighbors.is_empty())
        .map(|(i, _)| i)
        .collect();

    for node in nodes_with_edges {
        let out_closeness = out_closeness(adj_list, node);
        closeness_map.insert(node, out_closeness);
    }

    let mut closeness_vec: Vec<(usize, f64)> = closeness_map.into_iter().collect();
    closeness_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    closeness_vec
}

fn in_closeness(adj_list: &Vec<Vec<usize>>, start: usize) -> f64 {
    let rev_adj_list = reverse_al(adj_list);
    let mut distances = bfs(&rev_adj_list, start);
    let mut count = 0;
    let mut total_distance = 0;

    for d in &distances {
        if let Some(dist) = &d {
            if *dist > 0 as usize {
                total_distance += dist;
                count += 1;
            }
        }
    }

    if total_distance == 0 {
        return 0.0
    }

    count as f64 / total_distance as f64
}

fn get_all_in_closeness(adj_list: &Vec<Vec<usize>>) -> Vec<(usize, f64)> {
    let mut closeness_map: HashMap<usize, f64> = HashMap::new();

    let rev_adj_list = reverse_al(adj_list);

    let nodes_with_edges: Vec<usize> = rev_adj_list.iter()
        .enumerate()
        .filter(|(_, neighbors)| !neighbors.is_empty())
        .map(|(i, _)| i)
        .collect();

    for node in nodes_with_edges {
        let out_closeness = in_closeness(adj_list, node);
        closeness_map.insert(node, out_closeness);
    }

    let mut closeness_vec: Vec<(usize, f64)> = closeness_map.into_iter().collect();
    closeness_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    closeness_vec
}

fn main() {
    let adj_list = read_graph("amazon0302.txt");
    
    let trial_bfs = avg_distance(&adj_list, 100);
    println!("Average shortest path for 100 samples: {:.2}", trial_bfs);

    let histogram = avg_distance_histogram(&adj_list, 50);
}
