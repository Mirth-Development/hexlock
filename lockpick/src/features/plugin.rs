use bevy::prelude::*;

use super::lockpick::systems::spawn_lockpick;
use super::camera::systems::spawn_camera;

fn load_plugin() -> () {
    println!("Feature plugin loaded");
}
pub struct LockpickFeaturesPlugin;

impl Plugin for LockpickFeaturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,load_plugin)
            .add_systems(Startup, spawn_camera)
            .add_systems(Startup, spawn_lockpick)
            
        ;
    }
}