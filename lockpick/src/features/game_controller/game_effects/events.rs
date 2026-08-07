use bevy::prelude::*;
use crate::features::game_controller::game_effects::components::EffectLifetimeTimer;


#[derive(Event, Clone)]
pub struct EffectEvent{
    pub effect_type: EffectList,
    pub life_timer: Timer,
    pub start: Vec2,
    pub end: Vec2,
}

#[derive( Copy, Clone)]
pub enum EffectList{
    MagicalPick,
    ElectricalPick { intensity: f32 }, //bake the effect parameter into the enum itself
    RustDustPick //Want to have "dust" on hitting rust
}