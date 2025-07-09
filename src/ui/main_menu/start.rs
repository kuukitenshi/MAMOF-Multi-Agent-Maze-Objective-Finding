use bevy::prelude::*;

use crate::{GameState, settings::SimulationSettings};

use super::MainMenuItem;

#[derive(Component)]
struct StartButton;

pub fn start_button_bundle() -> impl Bundle + use<> {
    (
        MainMenuItem,
        StartButton,
        Node {
            width: Val::Px(120.0),
            height: Val::Px(45.0),
            border: UiRect::all(Val::Px(5.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        Button,
        BorderRadius::MAX,
        BorderColor(Color::BLACK),
        children![(MainMenuItem, Text::new("Start"), TextColor(Color::WHITE))],
    )
}

pub struct StartButtonPlugin;

impl Plugin for StartButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            start_button_clicked.run_if(in_state(GameState::InMenu)),
        );
    }
}

fn start_button_clicked(
    query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    mut game_state: ResMut<NextState<GameState>>,
    settings: Res<SimulationSettings>,
) {
    for interation in &query {
        if let Interaction::Pressed = *interation {
            game_state.set(GameState::Simulation);
            println!("Starting simulation:");
            println!("{:#?}", *settings);
        }
    }
}
