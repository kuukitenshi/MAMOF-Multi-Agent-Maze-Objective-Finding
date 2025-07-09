use std::{cmp::Ordering, collections::HashSet};

pub fn manhattan(a: (usize, usize), b: (usize, usize)) -> i32 {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as i32
}

pub fn manhattan_cmp(n1: (usize, usize), n2: (usize, usize), goal: (usize, usize)) -> Ordering {
    let n1_dist = manhattan(n1, goal);
    let n2_dist = manhattan(n2, goal);
    n2_dist.cmp(&n1_dist)
}

pub fn explored_cmp(
    n1: (usize, usize),
    n2: (usize, usize),
    explored: &HashSet<(usize, usize)>,
) -> Ordering {
    if explored.contains(&n1) && !explored.contains(&n2) {
        Ordering::Less
    } else if !explored.contains(&n1) && explored.contains(&n2) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

pub fn neighbors_cmp(
    n1: (usize, usize),
    n2: (usize, usize),
    neighbors: &Vec<(usize, usize)>,
) -> Ordering {
    let n1_dist = min_neighbor_distance(neighbors, n1);
    let n2_dist = min_neighbor_distance(neighbors, n2);
    n2_dist.cmp(&n1_dist)
}

pub fn border_cmp(n1: (usize, usize), n2: (usize, usize), map_size: usize) -> Ordering {
    let n1_dist = border_dist(n1, map_size);
    let n2_dist = border_dist(n2, map_size);
    n2_dist.cmp(&n1_dist)
}

pub fn goal_cmp(n1: (usize, usize), n2: (usize, usize), goal: (usize, usize)) -> Ordering {
    if n1 == goal {
        Ordering::Greater
    } else if n2 == goal {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn min_neighbor_distance(neighbors: &Vec<(usize, usize)>, position: (usize, usize)) -> i32 {
    neighbors
        .iter()
        .map(|n| manhattan(*n, position))
        .min()
        .unwrap_or(i32::MAX)
}

fn border_dist(node: (usize, usize), map_size: usize) -> usize {
    node.0
        .min(node.1)
        .min(map_size - node.0)
        .min(map_size - node.1)
}
