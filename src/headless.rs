use std::{
    collections::{HashMap, HashSet},
    fs,
};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    CliArgs, GameState,
    agent::{Agent, AgentsCompleteMazeEvent},
    rng::GlobalRng,
    settings::{SeedType, SimulationSettings},
    simulation::SimulationStepData,
};

#[derive(Serialize, Deserialize)]
struct SimulationSummary {
    configuration: SimulationConfiguration,
    results: SimulationResults,
}

#[derive(Serialize, Deserialize)]
struct SimulationResults {
    timesteps: u64,
    total_unique_explored_titles: usize,
    agent_explored_tiles: HashMap<usize, usize>,
}

#[derive(Serialize, Deserialize)]
struct SimulationConfiguration {
    num_agents: u32,
    map_size: usize,
    seed: u64,
    communication_options: CommunicationOptions,
}

impl SimulationConfiguration {
    pub fn create_from(seed: u64, settings: &SimulationSettings) -> Self {
        Self {
            num_agents: settings.num_agents,
            map_size: settings.grid_size,
            seed,
            communication_options: CommunicationOptions {
                share_goal: settings.share_goal,
                share_positions: settings.share_positions,
                share_maze: settings.share_tiles,
                agent_guiding: settings.enable_guiding,
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
struct CommunicationOptions {
    share_goal: bool,
    share_positions: bool,
    share_maze: bool,
    agent_guiding: bool,
}

pub struct MamofHeadlessPlugin;

impl Plugin for MamofHeadlessPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, launch_simulation);
        app.add_systems(
            Update,
            on_complete.run_if(on_event::<AgentsCompleteMazeEvent>),
        );
    }
}

fn launch_simulation(
    mut settings: ResMut<SimulationSettings>,
    cli_args: Res<CliArgs>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    settings.num_agents = cli_args.num_agents;
    settings.grid_size = cli_args.map_size;
    settings.share_goal = !cli_args.disable_goal_sharing;
    settings.share_positions = !cli_args.disable_position_sharing;
    settings.share_tiles = !cli_args.disable_map_sharing;
    settings.simulation_speed = cli_args.simulation_speed;
    settings.map_seed = cli_args
        .seed
        .map_or(SeedType::Random, |seed| SeedType::Selected(seed));
    println!("{:#?}", settings.into_inner());
    game_state.set(GameState::Simulation);
}

fn on_complete(
    step: Res<SimulationStepData>,
    mut event: EventWriter<AppExit>,
    cli_args: Res<CliArgs>,
    query: Query<&Agent>,
    settings: Res<SimulationSettings>,
    rand: Res<GlobalRng>,
) {
    println!("Simulation completed in {} timesteps", step.timesteps);
    let summary = create_simulation_summary(rand.seed, &settings, &query, step.timesteps);
    let contents =
        serde_json::to_string_pretty(&summary).expect("Failed to create simulation summary");
    if let Some(output_file) = &cli_args.output_file {
        fs::write(output_file, contents).expect("Failed to write to file!");
    }
    event.write(AppExit::Success);
}

fn create_simulation_summary(
    seed: u64,
    settings: &SimulationSettings,
    query: &Query<&Agent>,
    timesteps: u64,
) -> SimulationSummary {
    let union_set = query
        .iter()
        .map(|a| a.explored.clone())
        .reduce(|m1, m2| m1.union(&m2).map(|n| *n).collect());
    let count = union_set.unwrap_or(HashSet::new()).len();
    let mut agent_map = HashMap::new();
    query.iter().for_each(|a| {
        agent_map.insert(a.id, a.explored.len());
    });
    SimulationSummary {
        configuration: SimulationConfiguration::create_from(seed, settings),
        results: SimulationResults {
            timesteps: timesteps,
            total_unique_explored_titles: count,
            agent_explored_tiles: agent_map,
        },
    }
}
