use bevy::prelude::*;

#[derive(Component)]
pub struct TumblerChamberComponent;

#[derive(Component)]
pub struct GameObjectAnchorMarker;

#[derive(Component, Clone, Copy)]
pub struct LockComponent {
    pub num_of_tumblers: u32,
}

//Implementations
impl Default for LockComponent {
    fn default() -> Self {
        Self {
            num_of_tumblers : 4,
        }
    }
}
