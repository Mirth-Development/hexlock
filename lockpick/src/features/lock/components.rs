use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct LockComponent {
    pub num_of_tumblers: u32
}