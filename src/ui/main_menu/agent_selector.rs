use bevy::prelude::*;

use crate::{GameState, settings::SimulationSettings};

use super::{MainMenuItem, spawn_menu};

#[derive(Component)]
struct AgentDecreaseButton;

#[derive(Component)]
struct AgentIncreaseButton;

#[derive(Component)]
struct AgentTextLabel;

pub fn agent_selector_bundle() -> impl Bundle + use<> {
    (
        MainMenuItem,
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            column_gap: Val::Px(20.0),
            ..default()
        },
        children![
            (
                MainMenuItem,
                AgentDecreaseButton,
                Button,
                BorderRadius::MAX,
                BorderColor(Color::BLACK),
                Node {
                    width: Val::Px(45.0),
                    height: Val::Px(45.0),
                    border: UiRect::all(Val::Px(5.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![MainMenuItem, Text::new("-"), TextColor(Color::WHITE)]
            ),
            (
                MainMenuItem,
                Node {
                    width: Val::Px(270.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(AgentTextLabel, Text::default(), TextColor(Color::WHITE))]
            ),
            (
                MainMenuItem,
                AgentIncreaseButton,
                Button,
                BorderRadius::MAX,
                BorderColor(Color::BLACK),
                Node {
                    width: Val::Px(45.0),
                    height: Val::Px(45.0),
                    border: UiRect::all(Val::Px(5.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![MainMenuItem, Text::new("+"), TextColor(Color::WHITE)]
            )
        ],
    )
}

pub struct AgentSelectorPlugin;

impl Plugin for AgentSelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InMenu),
            agent_text_update.after(spawn_menu),
        );
        app.add_systems(
            Update,
            (agent_decrease, agent_increase).run_if(in_state(GameState::InMenu)),
        );
        app.add_systems(
            Update,
            agent_text_update
                .run_if(in_state(GameState::InMenu).and(resource_changed::<SimulationSettings>)),
        );
    }
}

fn agent_decrease(
    query: Query<&Interaction, (Changed<Interaction>, With<AgentDecreaseButton>)>,
    mut settings: ResMut<SimulationSettings>,
) {
    for interation in query {
        if let Interaction::Pressed = *interation {
            settings.num_agents = std::cmp::max(1, settings.num_agents.saturating_sub(1));
        }
    }
}

fn agent_increase(
    query: Query<&Interaction, (Changed<Interaction>, With<AgentIncreaseButton>)>,
    mut settings: ResMut<SimulationSettings>,
) {
    for interation in query {
        if let Interaction::Pressed = *interation {
            settings.num_agents = std::cmp::min(32, settings.num_agents.saturating_add(1));
        }
    }
}

fn agent_text_update(
    mut query: Query<&mut Text, With<AgentTextLabel>>,
    settings: Res<SimulationSettings>,
) {
    for mut text in &mut query {
        text.0 = format!("Agents: {:<2}", settings.num_agents);
    }
}
