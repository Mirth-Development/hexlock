
// Imports
use bevy::prelude::*;
use crate::features::game_controller::game_timer::definitions::*;
use crate::features::game_controller::game_timer::systems::*;

// Plugin
pub struct GameTimer {}
impl Plugin for GameTimer {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefinitionsForGameTimer{});
        app.add_plugins(SystemsForGameTimer{});
    }
}
