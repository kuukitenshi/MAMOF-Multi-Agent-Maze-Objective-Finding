use bevy::prelude::*;

use crate::{
    GameState,
    agent::SharedMazeKnowledge,
    rng::{GlobalRng, seed_rng},
    settings::SimulationSettings,
    simulation::{SimulationStepData, SimulationStepEvent},
};

pub struct SimulationUiPlugin;

impl Plugin for SimulationUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Simulation), spawn_ui.after(seed_rng));
        app.add_systems(OnExit(GameState::Simulation), despawn_ui);
        app.add_systems(
            Update,
            (update_remaining_agents.run_if(resource_changed::<SharedMazeKnowledge>),)
                .run_if(in_state(GameState::Simulation)),
        );
        app.add_systems(
            FixedUpdate,
            update_timesteps.run_if(on_event::<SimulationStepEvent>),
        );
    }
}

#[derive(Component)]
struct SimulationUiItem;

#[derive(Component)]
struct TimestepsText;

#[derive(Component)]
struct RemainingAgentsText;

fn spawn_ui(mut commands: Commands, settings: Res<SimulationSettings>, rng: Res<GlobalRng>) {
    commands.spawn(simulation_ui_bundle(&settings, &rng));
}

fn despawn_ui(mut commands: Commands, query: Query<Entity, With<SimulationUiItem>>) {
    query.iter().for_each(|entity| {
        commands.entity(entity).despawn();
    });
}

fn update_timesteps(
    mut query: Query<&mut Text, With<TimestepsText>>,
    step: Res<SimulationStepData>,
) {
    query
        .iter_mut()
        .for_each(|mut text| text.0 = format!("Timesteps: {:<4}", step.timesteps));
}

fn update_remaining_agents(
    mut query: Query<&mut Text, With<RemainingAgentsText>>,
    knowledge: Res<SharedMazeKnowledge>,
) {
    query.iter_mut().for_each(|mut text| {
        text.0 = format!("Remaining agents: {:>2}", knowledge.remaining_agents)
    });
}

fn simulation_ui_bundle(
    settings: &Res<SimulationSettings>,
    rng: &Res<GlobalRng>,
) -> impl Bundle + use<> {
    (
        SimulationUiItem,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::End,
            ..default()
        },
        children![(
            Node {
                width: Val::Percent(20.0),
                height: Val::Percent(18.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::linear_rgba(0.05, 0.05, 0.05, 0.8)),
            children![
                (Text::new("Information"), TextColor(Color::WHITE)),
                (
                    Node {
                        width: Val::Percent(90.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    children![
                        (
                            Text::new(format!("Seed: {}", rng.seed)),
                            TextColor(Color::WHITE)
                        ),
                        (
                            Text::new(format!("Agents: {}", settings.num_agents)),
                            TextColor(Color::WHITE)
                        ),
                        (
                            Text::new(format!("Map size: {}", settings.grid_size)),
                            TextColor(Color::WHITE)
                        ),
                        (
                            TimestepsText,
                            Text::new(format!("Timesteps: {:<4}", 0)),
                            TextColor(Color::WHITE),
                        ),
                        (
                            RemainingAgentsText,
                            Text::new(format!("Remaining agents: {:<2}", 0)),
                            TextColor(Color::WHITE)
                        )
                    ]
                )
            ]
        )],
    )
}
