use bevy::prelude::*;

use crate::{GameState, settings::SimulationSettings};

use super::{MainMenuItem, spawn_menu};

#[derive(Component)]
struct GridDecreaseButton;

#[derive(Component)]
struct GridIncreaseButton;

#[derive(Component)]
struct GridTextLabel;

pub fn grid_selector_bundle() -> impl Bundle + use<> {
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
                GridDecreaseButton,
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
                children![(GridTextLabel, Text::default(), TextColor(Color::WHITE))]
            ),
            (
                MainMenuItem,
                GridIncreaseButton,
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

pub struct GridSelectorPlugin;

impl Plugin for GridSelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InMenu),
            grid_text_update.after(spawn_menu),
        );
        app.add_systems(
            Update,
            (grid_decrease, grid_increase).run_if(in_state(GameState::InMenu)),
        );
        app.add_systems(
            Update,
            grid_text_update
                .run_if(in_state(GameState::InMenu).and(resource_changed::<SimulationSettings>)),
        );
    }
}

fn grid_decrease(
    query: Query<&Interaction, (Changed<Interaction>, With<GridDecreaseButton>)>,
    mut settings: ResMut<SimulationSettings>,
) {
    for interation in query {
        if let Interaction::Pressed = *interation {
            settings.grid_size = std::cmp::max(8, settings.grid_size.saturating_sub(1));
        }
    }
}

fn grid_increase(
    query: Query<&Interaction, (Changed<Interaction>, With<GridIncreaseButton>)>,
    mut settings: ResMut<SimulationSettings>,
) {
    for interation in query {
        if let Interaction::Pressed = *interation {
            settings.grid_size = std::cmp::min(64, settings.grid_size.saturating_add(1));
        }
    }
}

fn grid_text_update(
    mut query: Query<&mut Text, With<GridTextLabel>>,
    settings: Res<SimulationSettings>,
) {
    for mut text in &mut query {
        text.0 = format!("Map size: {:<2}", settings.grid_size);
    }
}
