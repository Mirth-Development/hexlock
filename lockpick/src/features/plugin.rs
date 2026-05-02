use bevy::prelude::*;

use super::lockpick::systems::spawn_lockpick;
use super::camera::systems::spawn_camera;
use super::lock::resource::LockSpriteHandles;
use super::lock::systems::{spawn_lock, load_sprite_resources, load_game_resources};

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
            .add_systems(Startup, (load_game_resources, load_sprite_resources, spawn_lock).chain());
    }
}