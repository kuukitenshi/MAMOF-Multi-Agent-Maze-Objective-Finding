use bevy::prelude::*;
use knossos::maze::{GameMap, GrowingTree, Method, OrthogonalMazeBuilder};

use crate::{
    GameState,
    rng::{GlobalRng, seed_rng},
    settings::SimulationSettings,
};

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Tile {
    #[default]
    Wall,
    Floor,
    Goal,
}

#[derive(Resource, Default)]
pub struct Maze {
    pub tile_grid: Vec<Vec<Tile>>,
    pub goal: (usize, usize),
}

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Maze::default());
        app.add_systems(OnEnter(GameState::Simulation), create_maze.after(seed_rng));
    }
}

pub fn create_maze(
    mut maze: ResMut<Maze>,
    rand: ResMut<GlobalRng>,
    settings: Res<SimulationSettings>,
) {
    let generated_maze = OrthogonalMazeBuilder::new()
        .width(settings.grid_size)
        .height(settings.grid_size)
        .seed(Some(rand.seed))
        .algorithm(Box::new(GrowingTree::new(Method::Random)))
        .build();
    let formatter = GameMap::new()
        .wall('#')
        .span(1)
        .with_start_goal()
        .seed(Some(rand.seed))
        .goal('G')
        .start('#');
    let game_map = generated_maze.format(formatter).into_inner();
    maze.tile_grid = Vec::new();
    for (y, line) in game_map.lines().enumerate() {
        let mut vec = Vec::with_capacity(line.len());
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                vec.push(Tile::Wall);
            } else if c == 'G' {
                vec.push(Tile::Goal);
                maze.goal = (x, y);
            } else {
                vec.push(Tile::Floor);
            }
        }
        maze.tile_grid.push(vec);
    }
}
