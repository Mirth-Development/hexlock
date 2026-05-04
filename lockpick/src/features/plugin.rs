use bevy::prelude::*;
use crate::features::controls::messages::QuitGame;
use crate::features::lock::messages::CatchTumbler;
use crate::features::lock::spring::systems::stretch_to_tumbler;
use crate::features::lock::tumblers::systems::{ tumbler_movement};
use crate::features::lockpick::events::LockpickAction;
use crate::features::rand::systems::load_random_seed;
use super::lockpick::systems::{handle_lockpick_message, lockpick_movement, move_to_focused_tumbler, spawn_lockpick};
use super::camera::systems::spawn_camera;
use super::lock::systems::{spawn_lock, load_sprite_resources, load_lock_resources, handle_catching_tumblers};
use super::controls::systems::user_control_system;


fn load_plugin() -> () {
    println!("Feature plugin loaded");
}
pub struct LockpickFeaturesPlugin;

impl Plugin for LockpickFeaturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,load_plugin)
            .add_systems(Startup, spawn_camera)
            .add_systems(Startup, spawn_lockpick)
            // .add_systems(Startup, spawn_lock)
            // .add_systems(Startup, (load_sprites, build_lock.run_if(resource_exists::<LockSprites>).chain()));
            .add_message::<LockpickAction>()
            .add_message::<CatchTumbler>()
            .add_message::<QuitGame>()
            .add_systems(Startup, (load_lock_resources, load_sprite_resources, load_random_seed, spawn_lock).chain())
            // .add_systems(PostUpdate, populate_global_positions.run_if(run_once)) //Global Transform is only populated in PostUpdate
            .add_systems(Update, (move_to_focused_tumbler, tumbler_movement, lockpick_movement, user_control_system, stretch_to_tumbler))
            .add_systems(Update, (handle_lockpick_message, handle_catching_tumblers));
            //.add_systems(Startup, center_lock);
    }
}