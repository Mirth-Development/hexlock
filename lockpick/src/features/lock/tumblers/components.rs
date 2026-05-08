use bevy::prelude::*;
use crate::features::lock::tumblers::resources::{Directions, TumblerSize, TumblerType};

//Naming Component
#[derive(Component, Clone)]
pub struct TumblerComponent{
    pub position: u32,
    pub velocity: Vec3,
    pub timer: Timer,
    pub tumbler_type: TumblerType ,
    pub size: TumblerSize,
    pub order_num_entity: Entity,
}

#[derive(Component)]
pub struct FocusedTumblerComponent;

#[derive(Component)]
pub struct SetTumblerComponent;

#[derive(Component)]
pub struct TumblerRustComponent{
    pub hits: u32,
}

#[derive(Component)]
pub struct TumblerMagicComponent{
    pub arrow_code: Vec<Directions>
}

impl Default for TumblerComponent {
    fn default() -> Self {
        Self {
            position: 0,
            velocity: Vec3::splat(0.0),
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            tumbler_type: TumblerType::Normal,
            size: TumblerSize::Medium,
            order_num_entity: Entity::PLACEHOLDER //This is how defaults are handled for entities
            //set: false //Handle this with a component
        }
    }
}



