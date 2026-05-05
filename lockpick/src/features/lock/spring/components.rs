use bevy::prelude::*;
use crate::features::lock::spring::resources::SpringSize;

//Naming Component
#[derive(Component)]
pub struct SpringComponent{
    pub position: u32,
    pub spring_size: SpringSize
}

impl Default for SpringComponent{
    fn default() -> Self {
        Self {
            position: 0,
            spring_size: SpringSize::Regular
        }
    }
}
