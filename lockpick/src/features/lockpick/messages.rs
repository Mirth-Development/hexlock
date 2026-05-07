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
