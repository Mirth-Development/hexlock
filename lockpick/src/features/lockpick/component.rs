use bevy::prelude::*;

//Amount of time before you can activate the lockpick again
#[derive(Component)]
pub struct LockpickTimer{
    timer: Timer
}

#[derive(Component)]
pub struct LockpickComponent{
    pub is_moving: bool,
    pub current_tumbler: u32,
}


