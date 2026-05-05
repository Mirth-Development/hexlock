use bevy::prelude::*;
use crate::features::lock::resource::TumblerSpringPairings;
use crate::features::lock::spring::components::SpringComponent;
use crate::features::lock::tumblers::components::{TumblerComponent};
use crate::features::lock::tumblers::systems::{HEIGHT_OF_LARGE_TUMBLER_SPRITE, HEIGHT_OF_MEDIUM_TUMBLER_SPRITE, HEIGHT_OF_SMALL_TUMBLER_SPRITE};
use crate::features::lock::systems::TOP_OF_CHAMBER;
use crate::features::lock::tumblers::resources::TumblerSize;

pub const HEIGHT_OF_SPRING_SPRITE: f32= 440.0;
pub fn stretch_to_tumbler(
    tumbler_spring_pairings: Res<TumblerSpringPairings>,
    transforms: Query<&Transform, Without<SpringComponent>>,
    tumblers: Query<&TumblerComponent>,
    mut spring_transforms: Query<&mut Transform, With<SpringComponent>>

) {
    for (tumbler, spring) in &tumbler_spring_pairings.array{

        let Ok(tumbler_transform) = transforms.get(*tumbler) else { panic!() };
        let Ok(tumbler) = tumblers.get(*tumbler) else { panic!() };
        let height = match tumbler.size {
            TumblerSize::Small =>{
                HEIGHT_OF_SMALL_TUMBLER_SPRITE
            },
            TumblerSize::Medium =>{
                HEIGHT_OF_MEDIUM_TUMBLER_SPRITE
            },
            TumblerSize::Large =>{
                HEIGHT_OF_LARGE_TUMBLER_SPRITE
            }
        };
        let bottom_y = tumbler_transform.translation.y + height / 2.0;

        let Ok(mut spring_t) = spring_transforms.get_mut(*spring) else { panic!() };
        let gap = TOP_OF_CHAMBER - bottom_y;
        spring_t.translation.y = bottom_y + gap / 2.0;
        spring_t.scale.y = gap / HEIGHT_OF_SPRING_SPRITE;
    }
}
