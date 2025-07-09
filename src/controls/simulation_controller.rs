use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    GameState,
    simulation::{SimulationStepData, SimulationStepEvent},
};

pub struct SimulationControllerPlugin;

impl Plugin for SimulationControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                toggle_pause.run_if(input_just_pressed(KeyCode::Space)),
                next_step.run_if(input_just_pressed(KeyCode::ArrowRight)),
                back_simulation.run_if(input_just_pressed(KeyCode::Escape)),
            )
                .run_if(in_state(GameState::Simulation)),
        );
    }
}

fn back_simulation(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::InMenu);
}

fn toggle_pause(mut step: ResMut<SimulationStepData>) {
    step.paused = !step.paused;
}

fn next_step(mut step: ResMut<SimulationStepData>, mut events: EventWriter<SimulationStepEvent>) {
    if step.paused && !step.stopped {
        step.timesteps = step.timesteps.saturating_add(1);
        events.write(SimulationStepEvent);
    }
}
