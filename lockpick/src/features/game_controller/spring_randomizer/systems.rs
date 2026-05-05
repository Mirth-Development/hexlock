
use bevy::prelude::*;
use rand::prelude::StdRng;
use rand::seq::IndexedRandom;
use crate::features::lock::resource::LockSpriteHandles;
use crate::features::lock::tumblers::components::TumblerComponent;
use crate::features::lock::tumblers::resources::{TumblerSize, TumblerType};

pub fn get_random_spring(
    pos: u32,
    timer: Timer,
    rng : &mut StdRng,
    sprite_handles : &LockSpriteHandles,
) -> (Sprite, TumblerComponent)
{
    let tumbler_types: Vec<TumblerType> = vec![TumblerType::Normal, TumblerType::Magic, TumblerType::Electric];
    let tumbler_sizes: Vec<TumblerSize> = vec![TumblerSize::Small, TumblerSize::Medium, TumblerSize::Large];

    let Some(random_type) = tumbler_types.choose(rng) else {
        println!("RAND TYPE FAILURE!");
        panic!()
    };
    let Some(random_size) = tumbler_sizes.choose(rng)else {
        println!("RAND SIZE FAILURE!");
        panic!()
    };

    let tumbler_sprite = match random_size {
        TumblerSize::Small => {
            sprite_handles.tumbler_small_sprite.clone()
        },
        TumblerSize::Medium => {
            sprite_handles.tumbler_medium_sprite.clone()
        },
        TumblerSize::Large => {
            sprite_handles.tumbler_large_sprite.clone()
        }
    };

    let tumbler_color: Color = match random_type {
        TumblerType::Normal=> {
            Color::default()
        },
        TumblerType::Magic => {
            Color::srgb(1.0, 0.0, 1.0)
        },
        TumblerType::Electric => {
            Color::srgb(1.0, 1.0, 0.0)
        }
    };

    return (Sprite{
        image: tumbler_sprite,
        color: tumbler_color,
        ..default()
    },
            TumblerComponent {
                position: pos,
                timer: timer,
                size: *random_size,
                tumbler_type: *random_type,
                ..default()
            })


}