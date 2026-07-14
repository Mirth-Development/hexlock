use bevy::prelude::*;
use rand::prelude::*;

///Resource which contains the Rand Generator and the seed that it uses.
#[derive(Resource)]
pub struct RandomSeed {
    pub random_number_generator: StdRng,
    pub seed: u64
}
