use bevy::prelude::*;

#[derive(Message)]
pub enum  LockpickAction {
    Pick,
    Left,
    Right
}