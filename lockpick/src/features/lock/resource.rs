use bevy::prelude::*;

#[derive(Resource)]
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
pub struct TumblerSpringPairings {
    pub array: Vec<(Entity, Entity)> //Tumbler, Spring
}

#[derive(Resource)]
pub struct LockOffset {
    pub offset: u32,
}
