use bevy::prelude::*;

#[derive(Event)]
///Event for effect while picking electric pick. Top and bottom determine the size of the effect.
pub struct Zap{
    pub life_timer: Timer,
    pub top: f32,
    pub bottom:f32,
}

///Event for effect while picking magic pick. Top and bottom determine the size of the effect.
#[derive(Event)]
pub struct Magic{
    pub life_timer: Timer,
    pub top: f32,
    pub bottom:f32,
}