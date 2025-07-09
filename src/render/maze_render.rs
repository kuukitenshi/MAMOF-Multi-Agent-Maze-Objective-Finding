use bevy::prelude::*;

use crate::{
    GameState,
    maze::{self, Maze, Tile},
};

pub const TILE_SIZE: usize = 60;
pub const TILE_GAP: usize = 1;

pub struct MazeRenderPlugin;

#[derive(Component)]
struct MazeTile;

impl Plugin for MazeRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Simulation),
            spawn_maze_tile_sprites.after(maze::create_maze),
        );
        app.add_systems(OnExit(GameState::Simulation), despawn_maze_tile_sprites);
    }
}

fn spawn_maze_tile_sprites(mut commands: Commands, maze: Res<Maze>) {
    for y in 0..maze.tile_grid.len() {
        for x in 0..maze.tile_grid.len() {
            let color = match maze.tile_grid[y][x] {
                Tile::Wall => Color::BLACK,
                Tile::Floor => Color::WHITE,
                Tile::Goal => Color::linear_rgb(1.0, 1.0, 0.0),
            };
            let sprite = Sprite {
                color,
                custom_size: Some(Vec2::splat(TILE_SIZE as f32)),
                ..default()
            };
            let y = y * TILE_SIZE + y * TILE_GAP;
            let x = x * TILE_SIZE + x * TILE_GAP;
            let transform = Transform::from_xyz(x as f32, y as f32, 0.0);
            commands.spawn((MazeTile, sprite, transform));
        }
    }
}

fn despawn_maze_tile_sprites(mut commands: Commands, query: Query<Entity, With<MazeTile>>) {
    query.iter().for_each(|entity| {
        commands.entity(entity).despawn();
    });
}
