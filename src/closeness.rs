use rand::seq::IteratorRandom;
use std::collections::HashMap;
use rand::rng;
use crate::bfs;

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

//This function is nearly identical to out_closeness, but reverses the adjacency list. 
pub fn in_closeness(adj_list: &Vec<Vec<usize>>, start: usize) -> f64 {
    let rev_adj_list = reverse_al(adj_list);
    let distances = bfs(&rev_adj_list, start);
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

    if total_distance == 0 { //had to add a 0 check just in case, it failed once
        return 0.0
    }

    count as f64 / total_distance as f64
}

//This takes an adjacency list and start node and returns the node's out closeness
pub fn out_closeness(adj_list: &Vec<Vec<usize>>, start: usize) -> f64 {
    let distances = bfs(adj_list, start);
    let mut count = 0;
    let mut total_distance = 0;

    //similar to avg_distance loop, but only performs it for one start node
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
pub fn get_all_out_closeness(adj_list: &Vec<Vec<usize>>, sample_size: usize) -> Vec<(usize, f64)> {
    //initializing a HashMap because that is the easiest way to store each node and closeness
    let mut rng = rng();
    let sample_size = 1000;
    let mut closeness_map: HashMap<usize, f64> = HashMap::new();

    let nodes_with_edges: Vec<usize> = adj_list.iter() //same filter
        .enumerate()
        .filter(|(_, neighbors)| !neighbors.is_empty())
        .map(|(i, _)| i)
        .collect();

    let samples = nodes_with_edges.iter() //same random sampler
        .copied()
        .choose_multiple(&mut rng, sample_size);

    for node in samples { //get all out closenesses and put into map
        let out_closeness = out_closeness(adj_list, node);
        closeness_map.insert(node, out_closeness);
    }

    let mut closeness_vec: Vec<(usize, f64)> = closeness_map.into_iter().collect(); //reformat to a vector of tuples
    closeness_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); //sort descending by closeness

    closeness_vec
}

//again nearly identical to get_all_out_closeness.
pub fn get_all_in_closeness(adj_list: &Vec<Vec<usize>>, sample_size: usize) -> Vec<(usize, f64)> {
    let mut closeness_map: HashMap<usize, f64> = HashMap::new();
    let mut rng = rng();
    let sample_size = 1000;

    let rev_adj_list = reverse_al(adj_list);

    let nodes_with_edges: Vec<usize> = rev_adj_list.iter() //same filter
        .enumerate()
        .filter(|(_, neighbors)| !neighbors.is_empty())
        .map(|(i, _)| i)
        .collect();

    let samples = nodes_with_edges.iter() //same random sampler
        .copied()
        .choose_multiple(&mut rng, sample_size);

    for node in samples {
        let out_closeness = in_closeness(adj_list, node);
        closeness_map.insert(node, out_closeness);
    }

    let mut closeness_vec: Vec<(usize, f64)> = closeness_map.into_iter().collect();
    closeness_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    closeness_vec
}