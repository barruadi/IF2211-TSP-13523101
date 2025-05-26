mod tsp;
mod util;

use tsp::tsp;
use util::euclidean_distance;
use std::io::{
    self,
    BufRead
};
use std::collections::{
    HashMap,
    HashSet
};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    println!("Select mode: \n1 = Coordinate input\n2 = Distance matrix input");
    let mode = lines
        .next().unwrap().unwrap()
        .trim().parse::<u32>().unwrap();

    let graph: Vec<Vec<usize>>;

    match mode {
        1 => {   // coordinate
            println!("Enter number of nodes:");
            let n: usize = lines
                .next().unwrap().unwrap()
                .trim().parse().unwrap();

            println!("Enter coordinates (x y) one per line:");
            let mut positions = Vec::new();
            for _ in 0..n {
                let line = lines.next().unwrap().unwrap();
                let parts: Vec<i32> = line
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
                positions.push((parts[0], parts[1]));
            }

            let mut g = vec![vec![0; n]; n];
            for i in 0..n {
                for j in 0..n {
                    if i != j {
                        g[i][j] = euclidean_distance(positions[i], positions[j]);
                    }
                }
            }

            graph = g;
        }

        2 => {  // adjacency matrix
            println!("Enter number of nodes:");
            let n: usize = lines
                .next().unwrap().unwrap()
                .trim().parse().unwrap();

            println!("Enter adjacency matrix (one row per line):");
            let mut g = Vec::new();
            for _ in 0..n {
                let line = lines.next().unwrap().unwrap();
                let row: Vec<usize> = line
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
                g.push(row);
            }

            graph = g;
        }

        _ => {
            eprintln!("Invalid mode selected.");
            return;
        }
    }

    let start = 0;
    let mut visited = HashSet::new();
    visited.insert(start);
    let mut memo = HashMap::new();

    let result = tsp(&graph, start, visited, &mut memo, start);
    println!("Minimum TSP cost: {}", result);
}