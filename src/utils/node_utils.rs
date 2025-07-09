use std::collections::HashSet;

use crate::maze::{Maze, Tile};

use super::heuristics;

fn calculate_neighbors(node: (usize, usize), max_size: usize) -> [(usize, usize); 4] {
    let mut neighbors = [(0, 0); 4];
    neighbors[0] = (node.0.saturating_sub(1), node.1);
    neighbors[1] = (
        std::cmp::min(node.0.saturating_add(1), max_size - 1),
        node.1,
    );
    neighbors[2] = (node.0, node.1.saturating_sub(1));
    neighbors[3] = (
        node.0,
        std::cmp::min(node.1.saturating_add(1), max_size - 1),
    );
    neighbors
}

pub fn explore_node(
    node: (usize, usize),
    maze: &Maze,
    explored: &HashSet<(usize, usize)>,
    frontier: &Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    calculate_neighbors(node, maze.tile_grid.len())
        .iter()
        .filter(|n| !explored.contains(*n))
        .filter(|n| !frontier.contains(*n))
        .filter(|n| **n != node)
        .filter(|n| maze.tile_grid[n.1][n.0] != Tile::Wall)
        .map(|n| *n)
        .collect()
}

pub fn find_know_path_to_node(
    current: (usize, usize),
    dest: (usize, usize),
    maze: &Maze,
    explored: &HashSet<(usize, usize)>,
) -> Option<Vec<(usize, usize)>> {
    let succ = |node: &(usize, usize)| {
        calculate_neighbors(*node, maze.tile_grid.len())
            .iter()
            .filter(|n| explored.contains(n) || **n == dest)
            .map(|n| (*n, 1))
            .collect::<Vec<((usize, usize), i32)>>()
    };
    let heuristic = |node: &(usize, usize)| heuristics::manhattan(*node, dest);
    pathfinding::prelude::astar(&current, succ, heuristic, |pos| *pos == dest).map(|opt| {
        opt.0
            .iter()
            .skip(1)
            .rev()
            .map(|n| *n)
            .collect::<Vec<(usize, usize)>>()
    })
}
