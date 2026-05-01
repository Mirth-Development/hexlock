use std::ptr::null;
use bevy::prelude::*;

fn return_none() -> () {
    println!("Asset plugin loaded");
}

pub struct LockpickAssetPlugin;

impl Plugin for LockpickAssetPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, return_none);
    }
}