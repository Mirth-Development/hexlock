use bevy::prelude::*;
use crate::features::lock::tumblers::resources::{TumblerSize, TumblerType};

//Naming Component
#[derive(Component)]
pub struct TumblerComponent{
    pub position: u32,
    pub velocity: Vec3,
    pub timer: Timer,
    pub tumbler_type: TumblerType ,
    pub size: TumblerSize
}

#[derive(Component)]
pub struct FocusedTumblerComponent;

#[derive(Component)]
pub struct SetTumblerComponent;


impl Default for TumblerComponent {
    fn default() -> Self {
        Self {
            position: 0,
            velocity: Vec3::splat(0.0),
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            tumbler_type: TumblerType::Normal,
            size: TumblerSize::Medium,
            //set: false //Handle this with a component
        }
    }
}



