use bevy::prelude::*;
use rand::prelude::*;
use crate::features::rand::resources::RandomSeed;

pub fn load_random_seed(
    mut commands: Commands,
) {
    let seed = 164124702147098127; //Random number go
    let rng = StdRng::seed_from_u64(seed);
    commands.insert_resource(RandomSeed{
        RandomNumberGenerator: rng,
        seed
    })
}
