
// Imports
use bevy::prelude::*;
use crate::features::game_timer::definitions::Definitions;
use crate::features::game_timer::systems::Systems;

// Plugin
pub struct GameTimer {}
impl Plugin for GameTimer {
    fn build(&self, app: &mut App) {
        app.add_plugins(Definitions{});
        app.add_plugins(Systems{});
    }
}
