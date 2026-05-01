use bevy::prelude::*;

fn return_none() -> () {
    println!("Feature plugin loaded");
}
pub struct LockpickFeaturesPlugin;

impl Plugin for LockpickFeaturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,return_none);
    }
}