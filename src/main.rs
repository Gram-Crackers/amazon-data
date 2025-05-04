use std::fs::File;
use std::io::BufRead;
use std::collections::VecDeque;
mod closeness;
mod histogram;
mod bfs;
use crate::closeness::*;
use crate::histogram::*;
use crate::bfs::*;

//reads the file into an adjacency list format given a path
fn read_graph(path: &str) -> Vec<Vec<usize>> {
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    
    let mut edges = Vec::new();
    let mut max_node = 0;

    //for each line, parse into f64s and store in "parts" tuple, then push into edges
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
    
    //reformat from collection of (from, to) tuples to a vector where the index represents the from node
    for (from, to) in edges {
        adj_list[from].push(to)
    }
    adj_list
}

fn main() {
    let start = std::time::Instant::now();
    let adj_list = read_graph("amazon0302.txt");
    
    let trial_bfs = avg_distance(&adj_list, 1000);
    println!("Average shortest path for 1000 samples: {:.2}", trial_bfs);

    let histogram = avg_distance_histogram(&adj_list, 2000); //using around 1% of the data, should still give a good picture
    print_histogram(&histogram);

    let in_close = get_all_in_closeness(&adj_list, 1000);
    let out_close = get_all_out_closeness(&adj_list, 1000);

    println!("Top In Closenesses:");
    for node in in_close.into_iter().take(10) {
        println!("{:?}", node);
    }

    println!("Top Out Closenesses:");
    for node in out_close.into_iter().take(10) {
        println!("{:?}", node);
    }

    let duration = start.elapsed();
    println!("Elapsed: {:?}", duration);
}
