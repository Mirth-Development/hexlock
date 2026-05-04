use bevy::prelude::*;
use super::resources::LockpickType;

//Amount of time before you can activate the lockpick again *unused*
#[derive(Component)]
pub struct LockpickTimer{
    timer: Timer
}

#[derive(Component)]
pub struct LockpickComponent{
    pub is_moving: bool,
    pub velocity: Vec3,
    pub current_tumbler: u32,
    pub lockpick_type: LockpickType
}

impl Default for LockpickComponent {
    fn default() -> Self {
        Self {
            is_moving: false,
            velocity: Vec3::splat(0.0),
            current_tumbler: 1,
            lockpick_type: LockpickType::Normal
        }
    }
}
