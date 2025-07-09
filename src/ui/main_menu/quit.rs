use bevy::prelude::*;

use crate::GameState;

use super::MainMenuItem;

#[derive(Component)]
struct QuitButton;

pub fn quit_button_bundle() -> impl Bundle + use<> {
    (
        MainMenuItem,
        QuitButton,
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
        children![(MainMenuItem, Text::new("Quit"), TextColor(Color::WHITE))],
    )
}

pub struct QuitButtonPlugin;

impl Plugin for QuitButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            quit_button_clicked.run_if(in_state(GameState::InMenu)),
        );
    }
}

fn quit_button_clicked(
    query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
    mut event: EventWriter<AppExit>,
) {
    for interation in &query {
        if let Interaction::Pressed = *interation {
            event.write(AppExit::Success);
        }
    }
}
