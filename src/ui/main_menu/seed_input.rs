use bevy::prelude::*;

use crate::{
    GameState,
    rng::GlobalRng,
    settings::{SeedType, SimulationSettings},
};

use super::{MainMenuItem, spawn_menu};

const DIGITS_KEYCODES: &'static [KeyCode; 10] = &[
    KeyCode::Digit0,
    KeyCode::Digit1,
    KeyCode::Digit2,
    KeyCode::Digit3,
    KeyCode::Digit4,
    KeyCode::Digit5,
    KeyCode::Digit6,
    KeyCode::Digit7,
    KeyCode::Digit8,
    KeyCode::Digit9,
];

#[derive(Component)]
struct SeedInputBox;

#[derive(Component)]
struct LastSeedButton;

pub fn seed_input_bundle() -> impl Bundle + use<> {
    (
        MainMenuItem,
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.0),
            ..default()
        },
        children![
            (
                MainMenuItem,
                Node {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(20.0),
                    ..default()
                },
                children![
                    (MainMenuItem, Text::new("Seed: "), TextColor(Color::WHITE)),
                    (
                        MainMenuItem,
                        Node {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            width: Val::Px(270.0),
                            height: Val::Px(45.0),
                            border: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        BorderColor(Color::BLACK),
                        children![(
                            MainMenuItem,
                            Text::new("121382789147"),
                            TextColor(Color::WHITE),
                            SeedInputBox
                        ),]
                    ),
                    (
                        MainMenuItem,
                        LastSeedButton,
                        Button,
                        Node {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            width: Val::Px(55.0),
                            height: Val::Px(45.0),
                            border: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        BorderColor(Color::BLACK),
                        children![(MainMenuItem, Text::new("<-"), TextColor(Color::WHITE),)]
                    )
                ]
            ),
            (
                MainMenuItem,
                Text::new("Leave empty to be random"),
                TextColor(Color::WHITE),
                TextFont {
                    font_size: 12.0,
                    ..default()
                }
            )
        ],
    )
}

pub struct SeedInputPlugin;

impl Plugin for SeedInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InMenu),
            seed_text_update.after(spawn_menu),
        );
        app.add_systems(
            Update,
            (key_input, erase_input, last_seed_click).run_if(in_state(GameState::InMenu)),
        );
        app.add_systems(
            Update,
            seed_text_update
                .run_if(in_state(GameState::InMenu).and(resource_changed::<SimulationSettings>)),
        );
    }
}

fn seed_text_update(
    mut query: Query<&mut Text, With<SeedInputBox>>,
    settings: ResMut<SimulationSettings>,
) {
    query.iter_mut().for_each(|mut text| {
        let string = match settings.map_seed {
            SeedType::Random => String::new(),
            SeedType::Selected(seed) => format!("{}", seed),
        };
        text.0 = string;
    });
}

fn erase_input(keys: Res<ButtonInput<KeyCode>>, mut settings: ResMut<SimulationSettings>) {
    if keys.just_pressed(KeyCode::Backspace) {
        if let SeedType::Selected(seed) = settings.map_seed {
            if let Some(seed) = seed.checked_div(10) {
                settings.map_seed = if seed == 0 {
                    SeedType::Random
                } else {
                    SeedType::Selected(seed)
                }
            }
        }
    }
}

fn key_input(keys: Res<ButtonInput<KeyCode>>, mut settings: ResMut<SimulationSettings>) {
    let mut input_number: Option<u8> = None;
    for (i, keycode) in DIGITS_KEYCODES.iter().enumerate() {
        if keys.just_pressed(*keycode) {
            input_number = Some(i as u8);
            break;
        }
    }

    if let Some(input_number) = input_number {
        match settings.map_seed {
            SeedType::Random => settings.map_seed = SeedType::Selected(input_number as u64),
            SeedType::Selected(seed) => {
                if let Some(seed) = seed.checked_mul(10) {
                    if let Some(seed) = seed.checked_add(input_number as u64) {
                        settings.map_seed = SeedType::Selected(seed)
                    }
                }
            }
        }
    }
}

fn last_seed_click(
    query: Query<&Interaction, (Changed<Interaction>, With<LastSeedButton>)>,
    mut settings: ResMut<SimulationSettings>,
    rand: Res<GlobalRng>,
) {
    for interation in query {
        if let Interaction::Pressed = *interation {
            settings.map_seed = SeedType::Selected(rand.seed);
        }
    }
}
