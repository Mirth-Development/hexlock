use bevy::prelude::*;

#[derive(Event)]
pub struct BreakRust{
    pub id: Entity,
}