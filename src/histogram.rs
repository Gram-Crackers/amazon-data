//module contains both functions used to create and output a histogram.
use rand::seq::IteratorRandom;
use std::collections::HashMap;
use rand::rng;
use crate::bfs;

//takes an adjacency list and sample size, gives a histogram in a hashmap with distances as keys and count as values
pub fn avg_distance_histogram(adj_list: &Vec<Vec<usize>>, sample_size: usize) -> HashMap<usize, usize> {
    let mut rng = rng();

    let nodes_with_edges: Vec<usize> = adj_list.iter() //same filter
        .enumerate()
        .filter(|(_, neighbors)| !neighbors.is_empty())
        .map(|(i, _)| i)
        .collect();
    
    let samples = nodes_with_edges.iter() //same random sampler
        .copied()
        .choose_multiple(&mut rng, sample_size);

    //this time using a HashMap where key is distance and value is number of times it occurs
    let mut frequency: HashMap<usize, usize> = HashMap::new();

    //essentially the same loop, but instead of incrementing the total distance and count,
    //it either inserts or increments the count for its distance.
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

//this function takes a histogram (hashmap) and prints it in the terminal.
pub fn print_histogram(histogram: &HashMap<usize, usize>) {
    let mut output: Vec<(&usize, &usize)> = histogram.iter().collect();
    output.sort_by_key(|&(dist, _)| dist);

    //finds the maximum count
    let max_count = histogram.values().copied().max().unwrap_or(1);
    let max_bar_len = 80; //maximum 80 characters per bar to accomodate smaller window sizes

    println!("Distance:");
    for (distance, count) in output {
        let bar_len = (count * max_bar_len) / max_count; //calculate the bar length
        let bar = std::iter::repeat("*").take(bar_len).collect::<String>(); //create a string of the bar to be printed

        if bar_len > 0 { //don't print empty bars (arguably they should be printed, but the output doesn't really give any more info that way)
            println!("{}: {}", distance, bar);
        }
    }
    println!("Distances with very low counts are omitted")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avg_distance_histogram() {
        // Define a simple graph 0 -> 1 -> 2 -> 3 and 0 -> 2
        let adj_list = vec![
            vec![1, 2],  
            vec![2],     
            vec![3],     
            vec![],      
        ];

        // Use a smaller sample size for testing
        let sample_size = 3;

        // Calculate the histogram
        let histogram = avg_distance_histogram(&adj_list, sample_size);

        // Check that histogram is not empty
        assert!(!histogram.is_empty(), "Histogram should not be empty");

        let mut expected_histogram: HashMap<usize, usize> = HashMap::new();
        expected_histogram.insert(2, 2);
        expected_histogram.insert(1, 4); //fill new HashMap with human-counted distances

        assert_eq!(histogram, expected_histogram);
    }
}