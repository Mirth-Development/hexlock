use bevy::prelude::*;
use crate::features::lock::components::LockComponent;
use crate::features::lock::tumblers::components::TumblerComponent;

#[derive(Resource)]
pub struct LockSpriteHandles {
    pub start_sprite: Handle<Image>,
    pub tumbler_chamber_sprite: Handle<Image>,
    pub end_sprite: Handle<Image>,
    pub spring_sprite: Handle<Image>,
    pub tumbler_sprite: Handle<Image>,
}

#[derive(Resource)]
pub struct TumblerSpringPairings {
    pub array: Vec<(Entity, Entity)> //Tumbler, Spring
}
