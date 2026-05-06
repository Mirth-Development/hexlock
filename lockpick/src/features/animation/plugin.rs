use bevy::prelude::*;
use crate::features::animation::components::AnimationShake;
use crate::features::animation::systems::animation_controller;

pub struct AnimationFeaturesPlugin;
impl Plugin for AnimationFeaturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
                        (animation_controller::<AnimationShake>)
        );
    }
}
