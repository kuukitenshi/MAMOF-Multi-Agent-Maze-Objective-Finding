use agent_selector::{AgentSelectorPlugin, agent_selector_bundle};
use bevy::prelude::*;
use communication_options::{CommunicationOptionsPlugin, communication_options_bundle};
use grid_size_selector::{GridSelectorPlugin, grid_selector_bundle};
use quit::{QuitButtonPlugin, quit_button_bundle};
use seed_input::{SeedInputPlugin, seed_input_bundle};
use speed_selector::{SpeedSelectorPlugin, speed_selector_bundle};
use start::{StartButtonPlugin, start_button_bundle};
use title::title_bundle;

use crate::GameState;

mod agent_selector;
mod communication_options;
mod grid_size_selector;
mod quit;
mod seed_input;
mod speed_selector;
mod start;
mod title;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AgentSelectorPlugin,
            GridSelectorPlugin,
            SpeedSelectorPlugin,
            SeedInputPlugin,
            CommunicationOptionsPlugin,
            StartButtonPlugin,
            QuitButtonPlugin,
        ));
        app.add_systems(OnEnter(GameState::InMenu), spawn_menu);
        app.add_systems(OnExit(GameState::InMenu), despawn_menu);
        app.add_systems(Update, button_hover.run_if(in_state(GameState::InMenu)));
    }
}

#[derive(Component)]
struct MainMenuItem;

pub fn spawn_menu(mut commands: Commands) {
    commands.spawn(main_menu_bundle());
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MainMenuItem>>) {
    query
        .iter()
        .for_each(|entity| commands.entity(entity).despawn());
}

fn main_menu_bundle() -> impl Bundle + use<> {
    (
        MainMenuItem,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
        children![
            title_bundle(),
            (
                MainMenuItem,
                Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(30.0),
                    ..default()
                },
                children![
                    agent_selector_bundle(),
                    grid_selector_bundle(),
                    speed_selector_bundle(),
                    seed_input_bundle(),
                    communication_options_bundle()
                ]
            ),
            (
                MainMenuItem,
                Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(30.0),
                    ..default()
                },
                children![quit_button_bundle(), start_button_bundle()]
            )
        ],
    )
}

fn button_hover(
    mut query: Query<(&Interaction, &mut BorderColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interation, mut border_color) in &mut query {
        *border_color = match interation {
            Interaction::None => BorderColor(Color::BLACK),
            _ => BorderColor(Color::WHITE),
        }
    }
}
