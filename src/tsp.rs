use std::cmp::min;
use std::collections::{
    HashMap,
    HashSet
};

pub fn tsp (
    graph   : &Vec<Vec<usize>>,
    current : usize,
    visited : HashSet<usize>,
    memo    : &mut HashMap<(usize, Vec<usize>), usize>,
    start   : usize
) -> usize {
    let mut visited_vec: Vec<usize> = visited.iter()
                                             .copied()
                                             .collect();
    visited_vec.sort();
    
    if visited.len() == graph.len() {
        return graph[current][start];
    }

    if let Some(&cost) = memo.get(&(current, visited_vec.clone())) {
        return cost;
    }

    let mut min_cost = usize::MAX;

    for next in 0..graph.len() {
        if !visited.contains(&next) {
            let mut next_visited = visited.clone();
            next_visited.insert(next);

            let cost = graph[current][next] + tsp(graph, next, next_visited, memo, start);
            min_cost = min(min_cost, cost);
        }
    }

    memo.insert((current, visited_vec), min_cost);
    return min_cost;
}