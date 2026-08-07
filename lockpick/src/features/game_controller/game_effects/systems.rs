use bevy::prelude::*;
use bevy::ui_render::ui_texture_slice_pipeline::init_ui_texture_slice_pipeline;
use crate::features::animation::components::{Animatable, Animated, AnimationFlip};
use crate::features::game_controller::game_effects::components::{ EffectLifetimeTimer};
use crate::features::game_controller::game_effects::events::{EffectEvent, EffectList};
use crate::features::game_controller::game_effects::resources::{EffectsSpriteHandles};
use crate::features::lock::tumblers::components::{FocusedTumblerComponent, TumblerComponent};
use crate::features::lockpick::resources::LockpickElectricCharge;

pub const HEIGHT_OF_LIGHTNING_SPRITE: f32 = 400.0;
pub const HEIGHT_OF_MAGIC_SPRITE: f32 = 400.0;
pub const HEIGHT_OF_RUST_SPRITE: f32 = 150.0;


//Load Resources
///Loads images from asset server and attaches their handles to a resource used for the effect sprites.
pub fn load_effects_sprite_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Sanity code
    println!("Loading EffectSprites!");

    let lightning_handle: Handle<Image> = asset_server.load("images/Zap.png");
    let rust_handle: Handle<Image> = asset_server.load("images/Head_Rust_Effect.png");
    let magic_handle: Handle<Image> = asset_server.load("images/Magic_Link.png");

    commands.insert_resource(EffectsSpriteHandles {
        lightning_effect: lightning_handle,
        magic_effect: magic_handle,
        rust_effect: rust_handle
    });
}


pub fn handle_game_effects(
    effect_event: On<EffectEvent>,
    mut commands: Commands,
    effects_sprite_handles: Res<EffectsSpriteHandles>,
) {

    let event_data = effect_event.event();

    match effect_event.effect_type {
        EffectList::MagicalPick => {
            on_magical_effect(commands, effects_sprite_handles, event_data);
        }
        EffectList::ElectricalPick { intensity: _ } => {
            on_electrical_effect(commands, effects_sprite_handles, event_data);
        }
        EffectList::RustDustPick => {
            //on_rust_effect();
        }
    }

}

//Triggers
///System which observes if a Zap Event occurs and spawns a temporary lightning entity between the pick and tumbler.
//Handle as a message? Rework this system, and potentially merge it into the folder for the lockpick. *FIX THIS*
pub fn on_electrical_effect(
    mut commands: Commands,
    effects_sprite_handles: Res<EffectsSpriteHandles>,
    event: &EffectEvent,
)
{

    println!("Spawn Lightning!");
    let midpoint = event.start.midpoint(event.end).extend(0.0);
    let EffectList::ElectricalPick { intensity} = event.effect_type else { return };
    let target_pos = event.end.trunc();
    let start_pos = event.start.trunc();
    let dir = target_pos - start_pos;
    //println!("rotate {0}", );
    let rotation = Quat::from_rotation_z(Vec2::Y.angle_to(dir));
    println!("rotation {0}", rotation);
    commands.spawn((
        Sprite {
            image: effects_sprite_handles.lightning_effect.clone(),
            color: Color::srgba(1.0, 1.0, 1.0, intensity),
            ..default()
        },
        Animated,
        AnimationFlip::new(0.3, midpoint, TimerMode::Once),
        EffectLifetimeTimer(event.life_timer.clone()), //Timer::from_seconds(0.4, TimerMode::Once)),
        Transform {
            //(bottom_y + gap / 2.0) = midpoint?
            translation: midpoint,
            rotation,
            // y =
            scale: vec3(1.0, (event.end.y-event.start.y) / HEIGHT_OF_LIGHTNING_SPRITE, 1.0),
            ..default()
        }
    )
    );

}

///System which observes if a Magic Event occurs and spawns a temporary magical entity between the pick and tumbler.
//Handle as a message? Rework this system, and potentially merge it into the folder for the lockpick. *FIX THIS*
pub fn on_magical_effect(
    mut commands: Commands,
    effects_sprite_handles: Res<EffectsSpriteHandles>,
    event: &EffectEvent,
)
{
    println!("Spawn Lightning!");
    let midpoint = event.start.midpoint(event.end).extend(0.0);
    let target_pos = event.end.trunc();
    let start_pos = event.start.trunc();
    let dir = target_pos - start_pos;
    //println!("rotate {0}", );
    let rotation = Quat::from_rotation_z(Vec2::Y.angle_to(dir));
    println!("rotation {0}", rotation);
    commands.spawn((
        Sprite {
            image: effects_sprite_handles.magic_effect.clone(),
            color: Color::srgba(1.0, 1.0, 1.0, 1.0),
            ..default()
        },
        Animated,
        AnimationFlip::new(0.3, midpoint, TimerMode::Once),
        EffectLifetimeTimer(event.life_timer.clone()), //Timer::from_seconds(0.4, TimerMode::Once)),
        Transform {
            //(bottom_y + gap / 2.0) = midpoint?
            translation: midpoint,
            rotation,
            // y =
            scale: vec3(1.0, (event.end.y-event.start.y) / HEIGHT_OF_MAGIC_SPRITE, 1.0),
            ..default()
        }
    )
    );

}



//Tick Lifettime timers
///System which ticks all EffectLifetimeTimers and then despawns them once finished.
pub fn handle_lifetime_timers(
    mut commands: Commands,
    time: Res<Time>,
    mut timed_entity_query : Query<(Entity, &mut EffectLifetimeTimer)>
) {

    for (timed_entity, mut lifetime_timer) in &mut timed_entity_query.iter_mut(){
        lifetime_timer.0.tick(time.delta());
        if lifetime_timer.0.just_finished(){
            //This removes the object and its children
            commands.entity(timed_entity).despawn();
        }
    }

}
