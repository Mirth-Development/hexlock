
// Imports
use bevy::prelude::*;
use crate::features::interface::definitions::Definitions;
use crate::features::interface::systems_for_interface_states::Interfaces;
use crate::features::interface::systems_for_interface_spawns::Spawns;

// Plugin
pub struct UserInterface {}
impl Plugin for UserInterface {
    fn build(&self, app: &mut App) {
        app.add_plugins(Definitions{});
        app.add_plugins(Interfaces{});
        app.add_plugins(Spawns{});
    }
}
