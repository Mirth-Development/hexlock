use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct LockComponent {
    pub num_of_tumblers: u32,
}

impl Default for LockComponent {
    fn default() -> Self {
        Self {
            num_of_tumblers : 4,
        }
    }
}