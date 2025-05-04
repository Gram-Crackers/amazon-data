use std::fs::File;
use std::io::BufRead;

//reads the file into an adjacency list format given a path
pub fn read_graph(path: &str) -> Vec<Vec<usize>> {
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