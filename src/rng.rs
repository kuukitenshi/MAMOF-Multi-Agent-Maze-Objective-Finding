use bevy::prelude::*;
use rand::{RngCore, SeedableRng, rngs::StdRng};

use crate::{
    GameState,
    settings::{SeedType, SimulationSettings},
};

pub struct RngPlugin;

#[derive(Resource)]
pub struct GlobalRng {
    pub rng: StdRng,
    pub seed: u64,
}

impl Default for GlobalRng {
    fn default() -> Self {
        let seed = gen_random_seed();
        Self {
            rng: StdRng::seed_from_u64(seed),
            seed,
        }
    }
}

impl Plugin for RngPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalRng::default());
        app.add_systems(OnEnter(GameState::Simulation), seed_rng);
    }
}

pub fn seed_rng(mut rng: ResMut<GlobalRng>, settings: Res<SimulationSettings>) {
    let seed = match settings.map_seed {
        SeedType::Random => {
            let seed = gen_random_seed();
            println!("Generated random seed: {}", seed);
            seed
        }
        SeedType::Selected(seed) => seed,
    };
    rng.rng = StdRng::seed_from_u64(seed);
    rng.seed = seed;
}

fn gen_random_seed() -> u64 {
    StdRng::from_os_rng().next_u64()
}
