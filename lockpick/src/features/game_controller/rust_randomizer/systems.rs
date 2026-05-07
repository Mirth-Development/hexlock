use bevy::ecs::system::entity_command::despawn;
use bevy::log::tracing::Instrument;
use bevy::prelude::*;
use rand::RngExt;
use rand::rngs::StdRng;
use crate::features::animation::components::Animated;
use crate::features::game_controller::game_effects::resources::EffectsSpriteHandles;
use crate::features::game_controller::game_effects::systems::HEIGHT_OF_RUST_SPRITE;
use crate::features::lock::tumblers::components::TumblerRustComponent;
use crate::features::lock::tumblers::resources::TumblerSize;
use crate::features::rand::resources::RandomSeed;

pub fn chance_to_add_rust(
    random_seed: &mut StdRng,
    tumbler_entity_commands: &mut EntityCommands,
    effects_sprite_handles: &EffectsSpriteHandles,
    tumbler_size: f32,
) {
    let random_chance = random_seed.random_bool(0.3); //30% chance
    let random_hit = random_seed.random_range(1..=4);

    let scale = tumbler_size/HEIGHT_OF_RUST_SPRITE;

    if random_chance{
        tumbler_entity_commands.with_child(
            (
                    Sprite{
                    image: effects_sprite_handles.rust_effect.clone(),
                        ..default()

                },
                Animated,
                TumblerRustComponent{
                    //parent_id: tumbler_entity_commands.id(),
                    hits: random_hit,
                },
                Transform{
                    scale: vec3(1.0, scale, 1.0),
                    translation: vec3(0.0,0.0,1.0),
                    ..default()
                })
        );
    }
}