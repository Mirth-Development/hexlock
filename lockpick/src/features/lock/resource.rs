use bevy::prelude::*;

#[derive(Resource)]
///Resource which holds the Image handles of all sprites which constitute the Lock.
pub struct LockSpriteHandles {
    pub start_sprite: Handle<Image>,
    pub tumbler_chamber_sprite: Handle<Image>,
    pub end_sprite: Handle<Image>,
    pub spring_sprite: Handle<Image>,
    pub tumbler_small_sprite: Handle<Image>,
    pub tumbler_medium_sprite: Handle<Image>,
    pub tumbler_large_sprite: Handle<Image>,
}

#[derive(Resource)]
///Holds the pairings of corresponding tumblers and springs
pub struct TumblerSpringPairings {
    pub array: Vec<(Entity, Entity)> //Tumbler, Spring
}

#[derive(Resource)]
///Resource which holds the pixel offset for the Lock
pub struct LockOffset {
    pub offset: u32,
}
