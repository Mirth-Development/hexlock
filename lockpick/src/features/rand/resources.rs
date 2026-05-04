use bevy::prelude::*;
use rand::prelude::*;


#[derive(Resource)]
pub struct RandomSeed {
    pub RandomNumberGenerator: StdRng
}