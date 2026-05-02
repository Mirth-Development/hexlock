use bevy::prelude::*;

#[derive(Resource)]
pub struct LockSpriteHandles {
    pub start_sprite: Handle<Image>,
    pub tumbler_sprite: Handle<Image>,
    pub end_sprite: Handle<Image>,
}

#[derive(Resource)]
pub struct NumberOfTumblers (pub i32);

