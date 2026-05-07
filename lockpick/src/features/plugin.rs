use bevy::prelude::*;
use crate::features::animation::plugin::AnimationFeaturesPlugin;
use crate::features::controls::messages::QuitGame;
use crate::features::game_controller::game_effects::systems::{load_effects_sprite_resources, on_lightning_effect};
use crate::features::game_controller::messages::GameStateMessage;
use crate::features::lock::messages::CatchTumbler;
use crate::features::lock::tumblers::messages::TumblerTimerMessage;
use crate::features::lockpick::messages::{ChargeLockpick, LockpickAction};
use crate::features::lockpick::resources::LockpickElectricCharge;
use crate::features::lockpick::systems::load_lockpick_resources;
use crate::features::rand::systems::load_random_seed;
use super::camera::systems::spawn_camera;
use super::lock::systems::{load_lock_sprite_resources};
use super::interface::plugin::UserInterface;

pub struct LockpickFeaturesPlugin;
impl Plugin for LockpickFeaturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, load_lock_sprite_resources, load_random_seed, load_lockpick_resources, load_effects_sprite_resources).chain());
        app.add_observer(on_lightning_effect);
        app.add_plugins(AnimationFeaturesPlugin);
        app.add_plugins(UserInterface{});
        app.add_message::<LockpickAction>();
        app.add_message::<CatchTumbler>();
        app.add_message::<QuitGame>();
        app.add_message::<TumblerTimerMessage>();
        app.add_message::<GameStateMessage>();
        app.add_message::<ChargeLockpick>();
        
    }
}
