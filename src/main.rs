mod closeness;
mod histogram;
mod bfs;
mod filereader;
use crate::filereader::*;
use crate::closeness::*;
use crate::histogram::*;
use crate::bfs::*;
use std::io::{self};

fn main() {
    println!("What sample size would you like to use? Note: higher sample size increases runtime");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let sample_size:usize = input.trim().parse().expect("Not a valid sample size");
    println!("sample_size: {}", sample_size);

    
    let start = std::time::Instant::now();

    let adj_list = read_graph("amazon0302.txt");
    
    let trial_bfs = avg_distance(&adj_list, sample_size);
    println!("Average shortest path for {} samples: {:.2}", sample_size, trial_bfs);

    let histogram = avg_distance_histogram(&adj_list, sample_size); //using around 1% of the data, should still give a good picture
    print_histogram(&histogram);

    let in_close = get_all_in_closeness(&adj_list, sample_size);
    let out_close = get_all_out_closeness(&adj_list, sample_size);

    println!("Top 10 In Closenesses:");
    for node in in_close.into_iter().take(10) {
        println!("{:?}", node);
    }

    println!("Top 10 Out Closenesses:");
    for node in out_close.into_iter().take(10) {
        println!("{:?}", node);
    }

    let duration = start.elapsed();
    println!("Elapsed: {:?}", duration);
}
