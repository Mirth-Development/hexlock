use bevy::prelude::*;
use crate::features::lock::resource::TumblerSpringPairings;
use crate::features::lock::spring::components::SpringComponent;
use crate::features::lock::tumblers::components::{TumblerComponent};
use crate::features::lock::tumblers::systems::{HEIGHT_OF_LARGE_TUMBLER_SPRITE, HEIGHT_OF_MEDIUM_TUMBLER_SPRITE, HEIGHT_OF_SMALL_TUMBLER_SPRITE};
use crate::features::lock::systems::TOP_OF_CHAMBER;
use crate::features::lock::tumblers::resources::TumblerSize;
use rand::prelude::StdRng;
use rand::seq::IndexedRandom;
use crate::features::lock::resource::LockSpriteHandles;
use crate::features::lock::spring::resources::SpringSize;

pub const HEIGHT_OF_SPRING_SPRITE: f32= 440.0;
pub fn stretch_to_tumbler(
    tumbler_spring_pairings: Res<TumblerSpringPairings>,
    transforms: Query<&Transform, Without<SpringComponent>>,
    tumblers: Query<&TumblerComponent>,
    mut spring_transforms: Query<&mut Transform, With<SpringComponent>>

) {
    for (tumbler, spring) in &tumbler_spring_pairings.array {
        let Ok(tumbler_transform) = transforms.get(*tumbler) else { panic!() };
        let Ok(tumbler) = tumblers.get(*tumbler) else { panic!() };
        let height = match tumbler.size {
            TumblerSize::Small => {
                HEIGHT_OF_SMALL_TUMBLER_SPRITE
            },
            TumblerSize::Medium => {
                HEIGHT_OF_MEDIUM_TUMBLER_SPRITE
            },
            TumblerSize::Large => {
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


pub fn gen_random_spring(
    pos: u32,
    rng : &mut StdRng,
    sprite_handles : &LockSpriteHandles,
) -> (Sprite, SpringComponent)
{
    let spring_size: Vec<SpringSize> = vec![SpringSize::Thin, SpringSize::Regular, SpringSize::Thick];

    let Some(random_spring_size) = spring_size.choose(rng) else {
        println!("RAND TYPE FAILURE!");
        panic!()
    };

    let spring_color: Color = match random_spring_size {
        SpringSize::Regular=> {
            Color::default()
        },
        SpringSize::Thin => {
            Color::srgb(0.0, 1.0, 0.0)
        },
        SpringSize::Thick => {
            Color::srgb(0.0, 0.0, 1.0)
        }
    };

    return (
        Sprite{
            image: sprite_handles.spring_sprite.clone(),
            color: spring_color,
        ..default()
    },
        SpringComponent{
            position: pos,
            spring_size: *random_spring_size
        })
}