use bevy::prelude::*;

use crate::{GameState, settings::SimulationSettings};

use super::{MainMenuItem, spawn_menu};

#[derive(Component)]
struct SpeedDecreaseButton;

#[derive(Component)]
struct SpeedIncreaseButton;

#[derive(Component)]
struct SpeedTextLabel;

pub fn speed_selector_bundle() -> impl Bundle + use<> {
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
                SpeedDecreaseButton,
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
                children![MainMenuItem, Text::new("<"), TextColor(Color::WHITE)]
            ),
            (
                MainMenuItem,
                Node {
                    width: Val::Px(270.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(SpeedTextLabel, Text::default(), TextColor(Color::WHITE))]
            ),
            (
                MainMenuItem,
                SpeedIncreaseButton,
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
                children![MainMenuItem, Text::new(">"), TextColor(Color::WHITE)]
            )
        ],
    )
}

pub struct SpeedSelectorPlugin;

impl Plugin for SpeedSelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InMenu),
            speed_text_update.after(spawn_menu),
        );
        app.add_systems(
            Update,
            (speed_decrease, speed_increase).run_if(in_state(GameState::InMenu)),
        );
        app.add_systems(
            Update,
            speed_text_update
                .run_if(in_state(GameState::InMenu).and(resource_changed::<SimulationSettings>)),
        );
    }
}

fn speed_decrease(
    query: Query<&Interaction, (Changed<Interaction>, With<SpeedDecreaseButton>)>,
    mut settings: ResMut<SimulationSettings>,
) {
    for interation in query {
        if let Interaction::Pressed = *interation {
            settings.simulation_speed = settings.simulation_speed.prev();
        }
    }
}

fn speed_increase(
    query: Query<&Interaction, (Changed<Interaction>, With<SpeedIncreaseButton>)>,
    mut settings: ResMut<SimulationSettings>,
) {
    for interation in query {
        if let Interaction::Pressed = *interation {
            settings.simulation_speed = settings.simulation_speed.next();
        }
    }
}

fn speed_text_update(
    mut query: Query<&mut Text, With<SpeedTextLabel>>,
    settings: Res<SimulationSettings>,
) {
    for mut text in &mut query {
        text.0 = format!("Simulation speed: {}", settings.simulation_speed.as_str());
    }
}
