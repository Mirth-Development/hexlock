
mod features;

use features::plugin::LockpickFeaturesPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LockpickFeaturesPlugin)
        .run();
}
