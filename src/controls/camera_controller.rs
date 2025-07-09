use bevy::{
    input::{
        common_conditions::input_pressed,
        mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    },
    prelude::*,
};

use crate::{
    GameState,
    maze::{self, Maze},
    render::maze_render::{TILE_GAP, TILE_SIZE},
};

const ZOOM_SPEED: f32 = 0.5;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(
            OnEnter(GameState::Simulation),
            center_camera_on_maze.after(maze::create_maze),
        );
        app.add_systems(
            Update,
            (
                mouse_motion.run_if(input_pressed(MouseButton::Left)),
                scroll_motion,
            )
                .run_if(in_state(GameState::Simulation)),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d::default(), Transform::default()));
}

fn center_camera_on_maze(mut query: Query<&mut Transform, With<Camera2d>>, maze: Res<Maze>) {
    query.iter_mut().for_each(|mut transform| {
        let maze_size = maze.tile_grid.len();
        let dim = (maze_size * TILE_SIZE + TILE_GAP * (maze_size - 1)) as f32;
        transform.translation.x = dim / 2.0;
        transform.translation.y = dim / 2.0;
    });
}

fn mouse_motion(
    mut query: Query<&mut Transform, With<Camera2d>>,
    mouse: Res<AccumulatedMouseMotion>,
) {
    for mut transform in &mut query {
        transform.translation.x += -mouse.delta.x;
        transform.translation.y += mouse.delta.y;
    }
}

fn scroll_motion(
    mut query: Query<&mut Projection, With<Camera2d>>,
    wheel: Res<AccumulatedMouseScroll>,
) {
    let zoom = -wheel.delta.y * ZOOM_SPEED;
    for projection in &mut query {
        if let Projection::Orthographic(ortho) = projection.into_inner() {
            ortho.scale = (ortho.scale + zoom).clamp(1.0, 10.0);
        }
    }
}
