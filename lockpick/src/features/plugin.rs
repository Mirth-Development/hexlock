use bevy::prelude::*;
use crate::features::controls::messages::QuitGame;
use crate::features::lock::messages::CatchTumbler;
use crate::features::lock::spring::systems::stretch_to_tumbler;
use crate::features::lock::tumblers::systems::{ tumbler_movement};
use crate::features::lockpick::messages::LockpickAction;
use crate::features::rand::systems::load_random_seed;
use super::lockpick::systems::{handle_lockpick_message, lockpick_movement, move_to_focused_tumbler, spawn_lockpick};
use super::camera::systems::spawn_camera;
use super::lock::systems::{spawn_lock, load_sprite_resources, load_lock_resources, handle_catching_tumblers};
use super::controls::systems::user_control_system;
use super::interface::plugin::UserInterface;

pub struct LockpickFeaturesPlugin;
impl Plugin for LockpickFeaturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, load_sprite_resources, load_random_seed).chain());
        app.add_plugins(UserInterface{});
        app.add_message::<LockpickAction>();
        app.add_message::<CatchTumbler>();
        app.add_message::<QuitGame>();
    }
}
