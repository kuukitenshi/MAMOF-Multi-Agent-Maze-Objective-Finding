use agent::AgentPlugin;
use bevy::{app::PluginGroupBuilder, log::LogPlugin, prelude::*, state::app::StatesPlugin};
use clap::Parser;
use cli_args::CliArgs;
use controls::{
    camera_controller::CameraControllerPlugin, simulation_controller::SimulationControllerPlugin,
};
use headless::MamofHeadlessPlugin;
use maze::MazePlugin;
use render::{agent_render::AgentRenderPlugin, maze_render::MazeRenderPlugin};
use rng::RngPlugin;
use settings::SettingsPlugin;
use simulation::SimulationPlugin;
use ui::{main_menu::MainMenuPlugin, simulation::SimulationUiPlugin};

mod agent;
mod cli_args;
mod controls;
mod headless;
mod maze;
mod render;
mod rng;
mod settings;
mod simulation;
mod ui;
mod utils;

#[derive(States, PartialEq, Eq, Debug, Clone, Copy, Hash, Default)]
pub enum GameState {
    #[default]
    InMenu,
    Simulation,
}

fn main() {
    let args = CliArgs::parse();

    let mut app = App::new();
    app.add_plugins(StatesPlugin);
    app.insert_state(GameState::InMenu);
    app.add_plugins(MamofCorePlugins);
    app.insert_resource(args.clone());

    if args.headless {
        println!("Running in HEADLESS mode");
        app.add_plugins(MinimalPlugins);
        app.add_plugins(MamofHeadlessPlugin);
    } else {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "MAMOF - Multi-Agent Maze Objective Finder".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .build()
                .disable::<LogPlugin>()
                .disable::<StatesPlugin>(),
        )
        .add_plugins(MamofRenderPlugins);
    }

    let exit = app.run();
    if let AppExit::Error(code) = exit {
        std::process::exit(code.get() as i32);
    }
}

struct MamofCorePlugins;

impl PluginGroup for MamofCorePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SettingsPlugin)
            .add(RngPlugin)
            .add(SimulationPlugin)
            .add(MazePlugin)
            .add(AgentPlugin)
    }
}

struct MamofRenderPlugins;

impl PluginGroup for MamofRenderPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CameraControllerPlugin)
            .add(SimulationControllerPlugin)
            .add(MainMenuPlugin)
            .add(SimulationUiPlugin)
            .add(MazeRenderPlugin)
            .add(AgentRenderPlugin)
    }
}
