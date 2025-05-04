//contains basic breadth-first search and a function that helps call it on all nodes and average.
use rand::seq::IteratorRandom;
use rand::rng;
use std::collections::VecDeque;

//Performs BFS from a given start node and adjacency list, returning a vector of shortest distances from start to each.
pub fn bfs(adj_list: &Vec<Vec<usize>>, start: usize) -> Vec<Option<usize>> {
    let n = adj_list.len();
    let mut visited = vec![false; n];
    let mut distance = vec![None; n]; //Initalize all as None, they will be Some(distance)
    let mut queue = VecDeque::new();

    visited[start] = true; //visited self
    distance[start] = Some(0); //distance to self is 0
    queue.push_back(start); //push visited nodes into the queue

    //loops through all neighbors of node u, then puts each of those neighbors into queue
    
    while let Some(u) = queue.pop_front() {
        for &v in &adj_list[u] {
            if !visited[v] { //prevents double counting nodes
                visited[v] = true;
                distance[v] = Some(distance[u].unwrap() + 1); //distance becomes one more than neighbor
                queue.push_back(v);
            }
        }
    }

    distance //vector with indices as nodes, contains either Some<distance> or None
}

//given the adjacency list and a sample size of nodes, return the average distance between each node
pub fn avg_distance(adj_list: &Vec<Vec<usize>>, sample_size: usize) -> f64 {
    let mut rng = rng();
    //filters out nodes with no edges while keeping indices (node, (neighbors))
    let nodes_with_edges: Vec<usize> = adj_list.iter()
        .enumerate()
        .filter(|(_, neighbors)| !neighbors.is_empty())
        .map(|(i, _)| i)
        .collect();

    let samples = nodes_with_edges.iter() //randomly select n samples
        .copied()
        .choose_multiple(&mut rng, sample_size);

    let mut total_distance = 0;
    let mut count = 0;

    //put each node into the bfs function, then iterate through the vector it returns
    //add each distance and increment counter, as long as it is reachable (what the if let is for)
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

    total_distance as f64 / count as f64 //return the average distance
}