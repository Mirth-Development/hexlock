use bevy::prelude::*;
use rand::prelude::*;
use rand::random;
use crate::features::rand::resources::RandomSeed;


///Function which creates an RNG and random seed that is loaded into the App Resources.
pub fn load_random_seed(
    mut commands: Commands,
) {
    let seed = random(); //Random number go
    let rng = StdRng::seed_from_u64(seed);
    commands.insert_resource(RandomSeed{
        random_number_generator: rng,
        seed
    })
}
