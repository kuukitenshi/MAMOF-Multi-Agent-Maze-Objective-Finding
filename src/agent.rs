use std::collections::HashSet;

use bevy::prelude::*;

use crate::{
    GameState,
    maze::{self, Maze, Tile},
    rng::GlobalRng,
    settings::SimulationSettings,
    simulation::{SimulationStepData, SimulationStepEvent},
    utils::{heuristics, node_utils},
};
use rand::seq::SliceRandom;

#[derive(Resource, Default)]
pub struct SharedMazeKnowledge {
    pub exit_pos: Option<(usize, usize)>,
    pub agent_positions: Vec<(usize, usize)>,
    pub explored_tiles: HashSet<(usize, usize)>,
    pub remaining_agents: u32,
}

#[derive(Component, Default)]
pub struct Agent {
    pub id: usize,
    pub explored: HashSet<(usize, usize)>,
    pub frontier: Vec<(usize, usize)>,
    pub position: (usize, usize),
    pub current_path: Option<Vec<(usize, usize)>>,
    pub found_goal: Option<(usize, usize)>,
    pub current_goal: (usize, usize),
}

#[derive(Event)]
pub struct AgentsCompleteMazeEvent;

pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AgentsCompleteMazeEvent>();
        app.insert_resource(SharedMazeKnowledge::default());
        app.add_systems(
            OnEnter(GameState::Simulation),
            (
                (spawn_agents).after(maze::create_maze),
                setup_shared_knowledge.after(spawn_agents),
            ),
        );
        app.add_systems(OnExit(GameState::Simulation), despawn_agents);
        app.add_systems(
            FixedUpdate,
            (
                agent_explore_node,
                agent_check_path.after(agent_explore_node),
                agent_backtrack_path.after(agent_check_path),
                agent_check_frontier.after(agent_backtrack_path),
                move_agent_path.after(agent_check_frontier),
            )
                .run_if(on_event::<SimulationStepEvent>),
        );
    }
}

impl Agent {
    fn new(id: usize, position: (usize, usize), goal: (usize, usize)) -> Self {
        Self {
            id,
            position,
            current_goal: goal,
            ..default()
        }
    }

    fn has_path(&self) -> bool {
        match &self.current_path {
            Some(path) => !path.is_empty(),
            None => false,
        }
    }
}

//---------------------- funcs ------------------------------------------------

fn get_valid_map_positions(grid: &Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, tile)| ((x, y), *tile))
                .collect::<Vec<((usize, usize), Tile)>>()
        })
        .filter(|(_, tile)| *tile == Tile::Floor)
        .map(|((x, y), _)| (x, y))
        .collect::<Vec<(usize, usize)>>()
}

pub fn spawn_agents(
    mut commands: Commands,
    maze: Res<Maze>,
    mut rand: ResMut<GlobalRng>,
    settings: Res<SimulationSettings>,
) {
    let mut possible_positions = get_valid_map_positions(&maze.tile_grid);
    possible_positions.shuffle(&mut rand.rng);
    possible_positions
        .iter()
        .take(settings.num_agents as usize)
        .enumerate()
        .for_each(|(id, (x, y))| {
            commands.spawn(Agent::new(id, (*x, *y), maze.goal));
        });
}

fn despawn_agents(mut commands: Commands, query: Query<Entity, With<Agent>>) {
    query
        .iter()
        .for_each(|entity| commands.entity(entity).despawn());
}

fn setup_shared_knowledge(
    query: Query<&Agent>,
    mut knowledge: ResMut<SharedMazeKnowledge>,
    settings: Res<SimulationSettings>,
) {
    knowledge.remaining_agents = settings.num_agents;
    knowledge.exit_pos = None;
    knowledge.agent_positions = Vec::with_capacity(settings.num_agents as usize);
    knowledge.explored_tiles = HashSet::new();
    query.iter().for_each(|agent| {
        let (x, y) = agent.position;
        if settings.share_positions {
            knowledge.agent_positions.push((x, y));
        }
    });
}

fn agent_explore_node(
    mut query: Query<&mut Agent>,
    mut knowledge: ResMut<SharedMazeKnowledge>,
    maze: Res<Maze>,
    settings: Res<SimulationSettings>,
) {
    query
        .iter_mut()
        .filter(|a| a.position != maze.goal)
        .filter(|a| !a.has_path())
        .for_each(|mut agent| {
            let current_position = agent.position;
            let children =
                node_utils::explore_node(agent.position, &maze, &agent.explored, &agent.frontier);
            if let Some(found_goal) = children.iter().find(|pos| **pos == maze.goal) {
                agent.found_goal = Some(*found_goal);
                if settings.share_goal {
                    knowledge.exit_pos = Some(*found_goal);
                }
            }
            agent.explored.insert(current_position);
            agent.frontier.extend(children);
            if settings.share_tiles {
                knowledge.explored_tiles.insert(current_position);
            }
        });
}

fn agent_check_path(
    mut query: Query<&mut Agent>,
    knowledge: Res<SharedMazeKnowledge>,
    maze: Res<Maze>,
) {
    query
        .iter_mut()
        .filter(|a| a.position != maze.goal)
        .filter(|a| a.current_goal == maze.goal)
        .filter(|a| a.found_goal.is_some() || knowledge.exit_pos.is_some())
        .for_each(|mut agent| {
            let union_explored = agent
                .explored
                .union(&knowledge.explored_tiles)
                .map(|n| *n)
                .collect();
            if let Some(path) = node_utils::find_know_path_to_node(
                agent.position,
                maze.goal,
                &maze,
                &union_explored,
            ) {
                agent.current_path = Some(path);
                agent.found_goal = Some(maze.goal);
            }
        });
}

fn agent_backtrack_path(
    mut query: Query<&mut Agent>,
    settings: Res<SimulationSettings>,
    maze: Res<Maze>,
) {
    if settings.share_positions && settings.enable_guiding {
        let mut lost_agents = query
            .iter()
            .filter(|a| a.found_goal.is_none())
            .map(|a| a.position)
            .collect::<Vec<(usize, usize)>>();
        if lost_agents.len() > 0 {
            let helping = lost_agents.pop().expect("No agent");
            query
                .iter_mut()
                .filter(|a| a.found_goal.is_some())
                .for_each(|mut agent| {
                    if agent.current_goal == maze.goal {
                        agent.current_path = None;
                    }
                    agent.current_goal = helping;
                });
        } else {
            query
                .iter_mut()
                .for_each(|mut a| a.current_goal = maze.goal);
        }
    }
}

fn agent_check_frontier(
    mut query: Query<&mut Agent>,
    knowledge: Res<SharedMazeKnowledge>,
    maze: Res<Maze>,
) {
    query
        .iter_mut()
        .filter(|a| a.position != maze.goal)
        .filter(|a| !a.has_path())
        .for_each(|mut agent| {
            if agent.found_goal.is_none() && knowledge.exit_pos.is_none() {
                let current_position = agent.position;
                agent.frontier.sort_by(|n1, n2| {
                    heuristics::goal_cmp(*n1, *n2, maze.goal)
                        .then_with(|| heuristics::explored_cmp(*n1, *n2, &knowledge.explored_tiles))
                        .then_with(|| heuristics::manhattan_cmp(*n1, *n2, current_position))
                        .then_with(|| heuristics::border_cmp(*n1, *n2, maze.tile_grid.len()))
                        .then_with(|| {
                            heuristics::neighbors_cmp(*n1, *n2, &knowledge.agent_positions)
                        })
                });
            } else {
                let current_goal = agent.current_goal;
                agent.frontier.sort_by(|n1, n2| {
                    let ord = heuristics::goal_cmp(*n1, *n2, current_goal)
                        .then_with(|| heuristics::explored_cmp(*n1, *n2, &knowledge.explored_tiles))
                        .then_with(|| heuristics::manhattan_cmp(*n1, *n2, current_goal));
                    if current_goal != maze.goal {
                        heuristics::goal_cmp(*n2, *n1, maze.goal).then(ord)
                    } else {
                        ord
                    }
                });
            }
        });
}

fn move_agent_path(
    mut query: Query<&mut Agent>,
    maze: Res<Maze>,
    mut knowledge: ResMut<SharedMazeKnowledge>,
    settings: Res<SimulationSettings>,
    mut step_data: ResMut<SimulationStepData>,
    mut events: EventWriter<AgentsCompleteMazeEvent>,
) {
    query
        .iter_mut()
        .filter(|a| a.position != maze.goal)
        .for_each(|mut agent| {
            if !agent.has_path() {
                if let Some(node) = agent.frontier.pop() {
                    let union_explored = agent
                        .explored
                        .union(&knowledge.explored_tiles)
                        .map(|n| *n)
                        .collect();
                    agent.current_path = node_utils::find_know_path_to_node(
                        agent.position,
                        node,
                        &maze,
                        &union_explored,
                    );
                }
            }
            if let Some(path) = &mut agent.current_path {
                if let Some(next_node) = path.pop() {
                    agent.position = next_node;
                    if agent.position == maze.goal {
                        knowledge.remaining_agents = knowledge.remaining_agents.saturating_sub(1);
                        if knowledge.remaining_agents == 0 {
                            step_data.stop();
                            events.write(AgentsCompleteMazeEvent);
                        }
                    }
                    if settings.share_positions {
                        knowledge.agent_positions[agent.id] = agent.position;
                    }
                }
            }
        });
}
