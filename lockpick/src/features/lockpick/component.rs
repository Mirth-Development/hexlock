use bevy::prelude::*;
use bevy::time::Stopwatch;
use super::resources::LockpickType;

#[derive(Component)]
///Player controlled Lockpick Component
pub struct LockpickComponent{
    ///Determine if the lockpick is currently moving to block movement/effects
    pub is_moving: bool,
    ///Velocity of the Lockpick component
    pub velocity: Vec3,
    ///The index of the current focused tumbler
    pub current_tumbler: u32,
    ///Enum of Lockpick type, i. e. Electric, Normal
    pub lockpick_type: LockpickType,
    ///Timer/Stopwatch which controls how long the electric pick charges.
    pub charge_timer: Stopwatch
}

impl Default for LockpickComponent {
    fn default() -> Self {
        Self {
            is_moving: false,
            velocity: Vec3::splat(0.0),
            current_tumbler: 1,
            lockpick_type: LockpickType::Normal,
            charge_timer: Stopwatch::new(),
        }
    }
}
