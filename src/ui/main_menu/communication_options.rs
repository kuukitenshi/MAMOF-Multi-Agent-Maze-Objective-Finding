use bevy::prelude::*;

use crate::{GameState, settings::SimulationSettings};

use super::{MainMenuItem, spawn_menu};

#[derive(Component)]
struct SharePositionsButton;

#[derive(Component)]
struct ShareGoalButton;

#[derive(Component)]
struct ShareMazeButton;

#[derive(Component)]
struct GuidingButton;

pub fn communication_options_bundle() -> impl Bundle + use<> {
    (
        MainMenuItem,
        Node {
            width: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
        children![
            (Text::new("Communication Options"), TextColor(Color::WHITE)),
            (
                MainMenuItem,
                Node {
                    width: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(50.0),
                    ..default()
                },
                children![
                    (
                        MainMenuItem,
                        Node {
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(5.0),
                            ..default()
                        },
                        children![
                            (
                                MainMenuItem,
                                ShareGoalButton,
                                Button,
                                BorderColor(Color::BLACK),
                                Node {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    border: UiRect::all(Val::Px(5.0)),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                children![(Text::new("X"), TextColor(Color::WHITE))]
                            ),
                            (
                                MainMenuItem,
                                Text::new("Share Goal"),
                                TextColor(Color::WHITE)
                            )
                        ],
                    ),
                    (
                        MainMenuItem,
                        Node {
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(5.0),
                            ..default()
                        },
                        children![
                            (
                                MainMenuItem,
                                SharePositionsButton,
                                Button,
                                BorderColor(Color::BLACK),
                                Node {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    border: UiRect::all(Val::Px(5.0)),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                children![(Text::new("X"), TextColor(Color::WHITE))]
                            ),
                            (
                                MainMenuItem,
                                Text::new("Share Positions"),
                                TextColor(Color::WHITE)
                            )
                        ],
                    ),
                    (
                        MainMenuItem,
                        Node {
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(5.0),
                            ..default()
                        },
                        children![
                            (
                                MainMenuItem,
                                ShareMazeButton,
                                Button,
                                BorderColor(Color::BLACK),
                                Node {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    border: UiRect::all(Val::Px(5.0)),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                children![(Text::new("X"), TextColor(Color::WHITE))]
                            ),
                            (
                                MainMenuItem,
                                Text::new("Share Maze"),
                                TextColor(Color::WHITE)
                            )
                        ],
                    ),
                    (
                        MainMenuItem,
                        Node {
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(5.0),
                            ..default()
                        },
                        children![
                            (
                                MainMenuItem,
                                GuidingButton,
                                Button,
                                BorderColor(Color::BLACK),
                                Node {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    border: UiRect::all(Val::Px(5.0)),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                children![(Text::new("X"), TextColor(Color::WHITE))]
                            ),
                            (
                                MainMenuItem,
                                Text::new("Agent guiding"),
                                TextColor(Color::WHITE)
                            )
                        ],
                    )
                ],
            )
        ],
    )
}

pub struct CommunicationOptionsPlugin;

impl Plugin for CommunicationOptionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InMenu),
            (
                update_maze_checkbox,
                update_position_checkbox,
                update_goal_checkbox,
                update_guiding_checkbox,
            )
                .after(spawn_menu),
        );
        app.add_systems(
            Update,
            (
                update_position_checkbox,
                update_maze_checkbox,
                update_goal_checkbox,
                update_guiding_checkbox,
            )
                .run_if(in_state(GameState::InMenu).and(resource_changed::<SimulationSettings>)),
        );
        app.add_systems(
            Update,
            (
                share_positions_click,
                share_goal_click,
                share_maze_click,
                guiding_click,
            )
                .run_if(in_state(GameState::InMenu)),
        );
    }
}

fn update_position_checkbox(
    query: Query<&Children, With<SharePositionsButton>>,
    mut query_child: Query<&mut Text>,
    settings: Res<SimulationSettings>,
) {
    for children in query.iter() {
        for child in children.iter() {
            if let Ok(mut text) = query_child.get_mut(child) {
                text.0 = match settings.share_positions {
                    true => String::from("X"),
                    false => String::new(),
                }
            }
        }
    }
}

fn update_maze_checkbox(
    query: Query<&Children, With<ShareMazeButton>>,
    mut query_child: Query<&mut Text>,
    settings: Res<SimulationSettings>,
) {
    for children in query.iter() {
        for child in children.iter() {
            if let Ok(mut text) = query_child.get_mut(child) {
                text.0 = match settings.share_tiles {
                    true => String::from("X"),
                    false => String::new(),
                }
            }
        }
    }
}

fn update_goal_checkbox(
    query: Query<&Children, With<ShareGoalButton>>,
    mut query_child: Query<&mut Text>,
    settings: Res<SimulationSettings>,
) {
    for children in query.iter() {
        for child in children.iter() {
            if let Ok(mut text) = query_child.get_mut(child) {
                text.0 = match settings.share_goal {
                    true => String::from("X"),
                    false => String::new(),
                }
            }
        }
    }
}

fn update_guiding_checkbox(
    query: Query<&Children, With<GuidingButton>>,
    mut query_child: Query<&mut Text>,
    settings: Res<SimulationSettings>,
) {
    for children in query.iter() {
        for child in children.iter() {
            if let Ok(mut text) = query_child.get_mut(child) {
                text.0 = match settings.enable_guiding {
                    true => String::from("X"),
                    false => String::new(),
                }
            }
        }
    }
}

fn share_positions_click(
    query: Query<&Interaction, (Changed<Interaction>, With<SharePositionsButton>)>,
    mut settings: ResMut<SimulationSettings>,
) {
    for interation in query {
        if let Interaction::Pressed = *interation {
            settings.share_positions = !settings.share_positions;
        }
    }
}

fn share_goal_click(
    query: Query<&Interaction, (Changed<Interaction>, With<ShareGoalButton>)>,
    mut settings: ResMut<SimulationSettings>,
) {
    for interation in query {
        if let Interaction::Pressed = *interation {
            settings.share_goal = !settings.share_goal;
        }
    }
}

fn share_maze_click(
    query: Query<&Interaction, (Changed<Interaction>, With<ShareMazeButton>)>,
    mut settings: ResMut<SimulationSettings>,
) {
    for interation in query {
        if let Interaction::Pressed = *interation {
            settings.share_tiles = !settings.share_tiles;
        }
    }
}

fn guiding_click(
    query: Query<&Interaction, (Changed<Interaction>, With<GuidingButton>)>,
    mut settings: ResMut<SimulationSettings>,
) {
    for interation in query {
        if let Interaction::Pressed = *interation {
            settings.enable_guiding = !settings.enable_guiding;
        }
    }
}
