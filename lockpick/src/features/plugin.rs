use bevy::prelude::*;
use crate::features::controls::messages::QuitGame;
use crate::features::lock::messages::CatchTumbler;
use crate::features::lock::spring::systems::stretch_to_tumbler;
use crate::features::lock::tumblers::messages::TumblerTimerMessage;
use crate::features::lock::tumblers::systems::{handle_tumbler_set, timer_tumbler_finished, tumbler_movement};
use crate::features::lockpick::messages::LockpickAction;
use crate::features::rand::systems::load_random_seed;
use super::lockpick::systems::{handle_lockpick_message, lockpick_movement, move_to_focused_tumbler, spawn_lockpick};
use super::camera::systems::spawn_camera;
use super::lock::systems::{spawn_lock, load_sprite_resources, load_lock_resources, handle_catching_tumblers};
use super::controls::systems::user_control_system;

pub struct LockpickFeaturesPlugin;
impl Plugin for LockpickFeaturesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Startup, spawn_lockpick)
            //Messages
            .add_message::<LockpickAction>()
            .add_message::<CatchTumbler>()
            .add_message::<QuitGame>()
            .add_message::<TumblerTimerMessage>()

            .add_systems(Startup, (load_lock_resources, load_sprite_resources, load_random_seed, spawn_lock).chain())
            //movement
            .add_systems(Update, (move_to_focused_tumbler, tumbler_movement, lockpick_movement, stretch_to_tumbler))
            //User controls
            .add_systems(Update, user_control_system)
            //Message systems
            .add_systems(Update, timer_tumbler_finished)
            //Timer Systems
            .add_systems(Update, (handle_lockpick_message, handle_catching_tumblers, handle_tumbler_set));
    }
}
