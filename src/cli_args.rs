use bevy::ecs::resource::Resource;
use clap::Parser;

use crate::settings::SimulationSpeed;

#[derive(Parser, Resource, Clone)]
#[command(version, about)]
pub struct CliArgs {
    #[arg(
        long,
        action,
        help("Run a single simulation instance in headless mode")
    )]
    pub headless: bool,

    #[arg(
        short,
        long,
        default_value_t = 2,
        value_parser = parse_agent_count,
        help("The number of agents to spawn")
    )]
    pub num_agents: u32,

    #[arg(short, long, default_value_t = 8, value_parser = parse_map_size, help("The size of the maze"))]
    pub map_size: usize,

    #[arg(
        long,
        action,
        help("Disable agents from communicating the discovered goal position")
    )]
    pub disable_goal_sharing: bool,

    #[arg(
        long,
        action,
        help("Disable agents from communicating their positions")
    )]
    pub disable_position_sharing: bool,

    #[arg(
        long,
        action,
        help("Disable agents from sharing the discovered map tiles")
    )]
    pub disable_map_sharing: bool,

    #[arg(
        long,
        action,
        help("Disable agents from trying to help guiding others to exit")
    )]
    pub disable_agent_guiding: bool,

    #[arg(
        long,
        default_value("x1"),
        help(
            "Changes the speed of the simulation, each value maps to the number of timesteps per second (e.g x128 -> 128 timesteps/second)"
        )
    )]
    pub simulation_speed: SimulationSpeed,

    #[arg(
        short,
        long,
        help("The seed to use to generate the maze and agent positions, leave empty of random")
    )]
    pub seed: Option<u64>,

    #[arg(short, long, help("Output file path to write the simulation results"))]
    pub output_file: Option<String>,
}

fn parse_agent_count(s: &str) -> Result<u32, String> {
    let count = s.parse().map_err(|e| format!("{}", e))?;
    if count < 1 || count > 32 {
        return Err(String::from(
            "Number of agents needs to be between 2 and 32",
        ));
    }
    Ok(count)
}

fn parse_map_size(s: &str) -> Result<usize, String> {
    let size = s.parse().map_err(|e| format!("{}", e))?;
    if size < 8 || size > 64 {
        return Err(String::from("Map size needs to be between 8 and 64"));
    }
    Ok(size)
}
