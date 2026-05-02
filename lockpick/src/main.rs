mod features;

use features::plugin::LockpickFeaturesPlugin;

use bevy::prelude::*;
use rand::prelude::*;




fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LockpickFeaturesPlugin)
        //.add_plugins(LockpickAssetPlugin)
        .run();
}