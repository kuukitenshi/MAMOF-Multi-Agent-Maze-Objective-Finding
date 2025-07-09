use bevy::prelude::*;

use crate::{GameState, settings::SimulationSettings};

#[derive(Event)]
pub struct SimulationStepEvent;

#[derive(Resource, Default)]
pub struct SimulationStepData {
    pub timesteps: u64,
    pub stopped: bool,
    pub paused: bool,
}

impl SimulationStepData {
    pub fn reset(&mut self) {
        self.timesteps = 0;
        self.stopped = false;
        self.paused = false;
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }
}

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationStepData::default());
        app.add_event::<SimulationStepEvent>();
        app.add_systems(OnEnter(GameState::Simulation), start_simulation);
        app.add_systems(
            FixedUpdate,
            tick_simulation.run_if(in_state(GameState::Simulation)),
        );
    }
}

fn start_simulation(
    mut step: ResMut<SimulationStepData>,
    settings: Res<SimulationSettings>,
    mut time: ResMut<Time<Fixed>>,
) {
    let seconds = settings.simulation_speed.to_duration().as_secs_f64();
    time.set_timestep_seconds(seconds);
    step.reset();
}

pub fn tick_simulation(
    mut step: ResMut<SimulationStepData>,
    mut event: EventWriter<SimulationStepEvent>,
) {
    if !step.stopped && !step.paused {
        step.timesteps = step.timesteps.saturating_add(1);
        event.write(SimulationStepEvent);
    }
}
