use bevy::prelude::*;

#[derive(Message)]
pub enum LockpickAction {
    Pick,
    Left,
    Right,
    Charge,
    Release,
    Hex,
    SwitchNext,
    SwitchLast,
}

#[derive(Message)]
pub enum ChargeLockpick {
    Charge,
    Release
}

#[derive(Message)]
pub enum HexDirection {
    Up,
    Down,
    Left,
    Right
}

#[derive(Message)]
pub struct StartHexCodeInput(pub f32);
