use std::time::Duration;

use bevy::prelude::*;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationSettings::default());
    }
}

#[derive(Resource, Debug)]
pub struct SimulationSettings {
    pub map_seed: SeedType,
    pub num_agents: u32,
    pub grid_size: usize,
    pub simulation_speed: SimulationSpeed,
    pub share_goal: bool,
    pub share_positions: bool,
    pub share_tiles: bool,
    pub enable_guiding: bool,
}

impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            map_seed: SeedType::Random,
            num_agents: 2,
            grid_size: 8,
            simulation_speed: SimulationSpeed::X1,
            share_goal: true,
            share_positions: true,
            share_tiles: true,
            enable_guiding: true,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SeedType {
    Random,
    Selected(u64),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, clap::ValueEnum)]
pub enum SimulationSpeed {
    X1,
    X2,
    X4,
    X8,
    X16,
    X32,
    X64,
    X128,
}

impl SimulationSpeed {
    pub fn next(&self) -> Self {
        match self {
            SimulationSpeed::X1 => SimulationSpeed::X2,
            SimulationSpeed::X2 => SimulationSpeed::X4,
            SimulationSpeed::X4 => SimulationSpeed::X8,
            SimulationSpeed::X8 => SimulationSpeed::X16,
            SimulationSpeed::X16 => SimulationSpeed::X32,
            SimulationSpeed::X32 => SimulationSpeed::X64,
            SimulationSpeed::X64 => SimulationSpeed::X128,
            SimulationSpeed::X128 => SimulationSpeed::X128,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            SimulationSpeed::X1 => SimulationSpeed::X1,
            SimulationSpeed::X2 => SimulationSpeed::X1,
            SimulationSpeed::X4 => SimulationSpeed::X2,
            SimulationSpeed::X8 => SimulationSpeed::X4,
            SimulationSpeed::X16 => SimulationSpeed::X8,
            SimulationSpeed::X32 => SimulationSpeed::X16,
            SimulationSpeed::X64 => SimulationSpeed::X32,
            SimulationSpeed::X128 => SimulationSpeed::X64,
        }
    }

    pub fn to_duration(&self) -> Duration {
        let secs = 1.0
            / match self {
                SimulationSpeed::X1 => 1.0,
                SimulationSpeed::X2 => 2.0,
                SimulationSpeed::X4 => 4.0,
                SimulationSpeed::X8 => 8.0,
                SimulationSpeed::X16 => 16.0,
                SimulationSpeed::X32 => 32.0,
                SimulationSpeed::X64 => 64.0,
                SimulationSpeed::X128 => 128.0,
            };
        Duration::from_secs_f32(secs)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            SimulationSpeed::X1 => "1x",
            SimulationSpeed::X2 => "2x",
            SimulationSpeed::X4 => "4x",
            SimulationSpeed::X8 => "8x",
            SimulationSpeed::X16 => "16x",
            SimulationSpeed::X32 => "32x",
            SimulationSpeed::X64 => "64x",
            SimulationSpeed::X128 => "128x",
        }
    }
}
