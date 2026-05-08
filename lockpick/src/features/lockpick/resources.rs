use bevy::prelude::*;

#[derive(Resource)]
pub enum LockpickType{
    Normal,
    Electric,
    Magic
}

#[derive(Resource)]
pub struct LockpickElectricCharge{
    pub is_charging: bool,
    pub max_charge: f32,
    pub charge_per_tick: f32,
    pub current_charge: f32,
}
