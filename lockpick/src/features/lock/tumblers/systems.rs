use std::arch::x86_64::__cpuid;
use bevy::ecs::error::panic;
use bevy::prelude::*;
use bevy::ui::debug::print_ui_layout_tree;
use rand::RngExt;
use crate::features::lock::spring::systems::HEIGHT_OF_SPRING_SPRITE;
use crate::features::lock::systems::TOP_OF_CHAMBER;
use crate::features::lock::tumblers::components::{FocusedTumblerComponent, SetTumblerComponent, TumblerComponent};
use crate::features::lockpick::component::LockpickComponent;
use crate::features::lockpick::events::LockpickAction;
use crate::features::rand::resources::RandomSeed;

pub const HEIGHT_OF_TUMBLER_SPRITE: f32= 150.0;

pub fn tumbler_movement(
    time: Res<Time>,
    mut tumblers: Query<(&mut Transform, &mut TumblerComponent)>,
) {
    let top = (TOP_OF_CHAMBER);
    let bottom = (TOP_OF_CHAMBER-(HEIGHT_OF_TUMBLER_SPRITE /2.0)-(HEIGHT_OF_SPRING_SPRITE/2.0));

    for (mut transform, mut tumbler) in &mut tumblers {
        if transform.translation.y + (HEIGHT_OF_TUMBLER_SPRITE/2.0) >= top{
            tumbler.velocity.y *= -1.0;
        } else if transform.translation.y + (HEIGHT_OF_TUMBLER_SPRITE/2.0) < bottom{
            tumbler.velocity.y = 0.0;
            transform.translation.y = bottom - (HEIGHT_OF_TUMBLER_SPRITE /2.0);
        }
        transform.translation += tumbler.velocity * time.delta_secs();
    }

}