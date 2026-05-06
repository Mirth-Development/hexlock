
// Imports
use bevy::prelude::*;
use crate::features::interface::definitions::*;
use crate::features::interface::systems_for_states::*;
use crate::features::interface::systems_for_spawns::*;

// Plugin
pub struct UserInterface {}
impl Plugin for UserInterface {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefinitionsForUserInterface{});
        app.add_plugins(SystemsForUserInterfaceStates{});
        app.add_plugins(SystemsForUserInterfaceSpawns{});
    }
}
