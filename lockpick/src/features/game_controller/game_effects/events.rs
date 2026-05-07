use bevy::prelude::*;

#[derive(Event)]
pub struct Zap{
    pub life_timer: Timer,
    pub top: f32,
    pub bottom:f32,
}

#[derive(Event)]
pub struct Rust{
    pub top: f32,
    pub bottom:f32,
}