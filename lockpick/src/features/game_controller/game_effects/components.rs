use bevy::prelude::*;


#[derive(Component)]
///Determines the duration of the animation effect
pub struct EffectLifetimeTimer(pub Timer);

