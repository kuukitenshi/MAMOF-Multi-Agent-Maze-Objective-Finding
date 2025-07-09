use bevy::prelude::*;

use super::MainMenuItem;

pub fn title_bundle() -> impl Bundle + use<> {
    (
        MainMenuItem,
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                MainMenuItem,
                Text::new("MAMOF"),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextShadow::default()
            ),
            (
                MainMenuItem,
                Text::new("Multi-Agent Maze Objective Finder"),
                TextColor(Color::WHITE),
            )
        ],
    )
}
