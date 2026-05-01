mod assets;
mod features;

use bevy::input::keyboard::Key::Lock;
use assets::plugin::LockpickAssetPlugin;
use features::plugin::LockpickFeaturesPlugin;

use bevy::prelude::*;
use rand::prelude::*;




fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LockpickFeaturesPlugin)
        .add_plugins(LockpickAssetPlugin)
        .run();
}