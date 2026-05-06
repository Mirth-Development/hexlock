use bevy::prelude::*;
use rand::prelude::*;

#[derive(Resource)]
pub struct RandomSeed {
    pub random_number_generator: StdRng,
    pub seed: u64
}
