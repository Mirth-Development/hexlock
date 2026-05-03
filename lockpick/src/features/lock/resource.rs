use bevy::prelude::*;

#[derive(Resource)]
pub struct LockSpriteHandles {
    pub start_sprite: Handle<Image>,
    pub tumbler_chamber_sprite: Handle<Image>,
    pub end_sprite: Handle<Image>,
    pub spring_sprite: Handle<Image>,
    pub tumbler_sprite: Handle<Image>,
}

#[derive(Resource)]
pub struct NumberOfTumblersToSpawn(pub i32);

