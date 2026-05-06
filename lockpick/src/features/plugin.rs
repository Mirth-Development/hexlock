use bevy::prelude::*;
use crate::features::animation::plugin::AnimationFeaturesPlugin;
use crate::features::controls::messages::QuitGame;
use crate::features::game_controller::messages::GameStateMessage;
use crate::features::lock::messages::CatchTumbler;
use crate::features::lock::tumblers::messages::TumblerTimerMessage;
use crate::features::lockpick::messages::LockpickAction;
use crate::features::rand::systems::load_random_seed;
use super::camera::systems::spawn_camera;
use super::lock::systems::{load_sprite_resources};
use super::interface::plugin::UserInterface;

pub struct LockpickFeaturesPlugin;
impl Plugin for LockpickFeaturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, load_sprite_resources, load_random_seed).chain());
        app.add_plugins(UserInterface{});
        app.add_message::<LockpickAction>();
        app.add_message::<CatchTumbler>();
        app.add_message::<QuitGame>();
        app.add_message::<TumblerTimerMessage>();
        app.add_message::<GameStateMessage>();
        app.add_plugins(AnimationFeaturesPlugin);
    }
}
