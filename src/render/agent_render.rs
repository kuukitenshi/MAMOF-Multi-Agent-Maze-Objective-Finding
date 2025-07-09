use bevy::prelude::*;

use crate::{
    GameState,
    agent::{self, Agent},
};

use super::maze_render::{TILE_GAP, TILE_SIZE};

const AGENT_COLORS: [Color; 10] = [
    Color::linear_rgb(1.0, 0.0, 0.0),
    Color::linear_rgb(0.0, 1.0, 0.0),
    Color::linear_rgb(0.0, 0.0, 1.0),
    Color::linear_rgb(1.0, 1.0, 0.0),
    Color::linear_rgb(0.0, 1.0, 1.0),
    Color::linear_rgb(1.0, 0.0, 1.0),
    Color::linear_rgb(1.0, 0.5, 0.0),
    Color::linear_rgb(0.5, 0.0, 0.5),
    Color::linear_rgb(0.5, 1.0, 0.0),
    Color::linear_rgb(1.0, 0.0, 0.5),
];

pub struct AgentRenderPlugin;

impl Plugin for AgentRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Simulation),
            setup_agent_sprites.after(agent::spawn_agents),
        );
        app.add_systems(
            Update,
            update_agent_transform.run_if(in_state(GameState::Simulation)),
        );
    }
}

fn update_agent_transform(query: Query<(&Agent, &mut Transform)>) {
    for (agent, mut transform) in query {
        let pos = agent.position;
        let pos_x = (pos.0 * TILE_SIZE + pos.0 * TILE_GAP) as f32;
        let pos_y = (pos.1 * TILE_SIZE + pos.1 * TILE_GAP) as f32;
        transform.translation = Vec3::new(pos_x, pos_y, 1.0);
    }
}

fn setup_agent_sprites(
    mut commands: Commands,
    query: Query<(Entity, &Agent)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    query.iter().for_each(|(entity, agent)| {
        let id = agent.id;
        let mesh = meshes.add(Mesh::from(Circle::new((TILE_SIZE / 2) as f32)));
        let material = materials.add(ColorMaterial::from(AGENT_COLORS[id % AGENT_COLORS.len()]));

        commands
            .entity(entity)
            .insert((
                Mesh2d(mesh),
                MeshMaterial2d(material),
                Transform::from_translation(Vec3::Z),
            ))
            .with_children(|builder| {
                builder.spawn((
                    Text2d::new(format!("{}", id)),
                    TextShadow::default(),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    Transform::from_translation(Vec3::Z),
                ));
            });
    });
}
