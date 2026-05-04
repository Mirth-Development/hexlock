use bevy::prelude::*;

#[derive(Resource)]
pub enum LockpickType{
    Normal,
    Electric,
    Magic
}