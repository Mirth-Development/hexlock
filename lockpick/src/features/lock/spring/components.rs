use bevy::prelude::*;
use crate::features::lock::spring::resources::SpringSize;

//Naming Component
#[derive(Component)]
///Component which tracks the index of the Spring left to right and the springs size.
pub struct SpringComponent{
    pub position: u32,
    pub size: SpringSize
}

impl Default for SpringComponent{
    fn default() -> Self {
        Self {
            position: 0,
            size: SpringSize::Regular
        }
    }
}
