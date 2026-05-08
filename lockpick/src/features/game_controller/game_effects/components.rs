use bevy::prelude::*;


#[derive(Component)]
pub struct EffectLifetimeTimer(pub Timer);

#[derive(Component)]
pub struct EffectKillMarker(pub Entity);