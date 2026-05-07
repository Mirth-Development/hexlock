
use bevy::prelude::*;
use crate::features::animation::components::{Animatable, Animated, AnimationFlip};
use crate::features::game_controller::game_effects::components::EffectLifetimeTimer;
use crate::features::game_controller::game_effects::events::Zap;
use crate::features::game_controller::game_effects::resources::{EffectsSpriteHandles};
use crate::features::lock::tumblers::components::{FocusedTumblerComponent, };
use crate::features::lockpick::resources::LockpickElectricCharge;

pub const HEIGHT_OF_LIGHTNING_SPRITE: f32 = 400.0;
pub const HEIGHT_OF_RUST_SPRITE: f32 = 150.0;


//Load Resources
pub fn load_effects_sprite_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Sanity code
    println!("Loading EffectSprites!");

    let lightning_handle: Handle<Image> = asset_server.load("images/Zap.png");
    let rust_handle: Handle<Image> = asset_server.load("images/Head_Rust_Effect.png");

    commands.insert_resource(EffectsSpriteHandles {
        lightning_effect: lightning_handle,
        rust_effect: rust_handle
    });
}

// pub fn load_effects_resources(mut commands: Commands) {
//     //Sanity code
//     println!("Loading EffectResources!");
//     commands.insert_resource(LightningTimer(Timer::from_seconds(0.3, TimerMode::Once)));
// }


//Process Effects



//Triggers
pub fn on_lightning_effect(
    zap: On<Zap>,
    mut commands: Commands,
    lockpick_electric_charge: Res<LockpickElectricCharge>,
    tumbler_component: Query<(&GlobalTransform),With<FocusedTumblerComponent>>,
    effects_sprite_handles: Res<EffectsSpriteHandles>
)
{
    //let Ok(lockpick_transform) = lockpick_component.single() else {panic!("No Lockpick!")};
    let Ok(focused_tumbler_transform) = tumbler_component.single() else {panic!("No Focused Tumbler!")};

    println!("Spawn Lightning!");
    //THERE IS A MIDPOINT FUNCTION DEAR FUCKING LORD
    let midpoint = zap.top.midpoint(zap.bottom);


    // let height = match focused_tumbler.size {
    //     TumblerSize::Small => {
    //         HEIGHT_OF_SMALL_TUMBLER_SPRITE
    //     },
    //     TumblerSize::Medium => {
    //         HEIGHT_OF_MEDIUM_TUMBLER_SPRITE
    //     },
    //     TumblerSize::Large => {
    //         HEIGHT_OF_LARGE_TUMBLER_SPRITE
    //     }
    // };
    let top_y = zap.top;
    let bottom_y = zap.bottom; //focused_tumbler_transform.translation().y + height / 2.0;
    let gap = top_y - bottom_y;//TOP_OF_CHAMBER - bottom_y;
    let pos = vec3(focused_tumbler_transform.translation().x, midpoint, 1.0 );
    let charge_intensity = lockpick_electric_charge.current_charge / lockpick_electric_charge.max_charge;

    commands.spawn((
        Sprite {
            image: effects_sprite_handles.lightning_effect.clone(),
            color: Color::srgba(1.0, 1.0, 1.0, charge_intensity),
            ..default()
        },
        Animated,
        AnimationFlip::new(0.3, pos),
        EffectLifetimeTimer(zap.life_timer.clone()),//Timer::from_seconds(0.4, TimerMode::Once)),
        Transform{
            //(bottom_y + gap / 2.0) = midpoint?
            translation: pos,
            // y =
            scale: vec3(1.0, (gap / HEIGHT_OF_LIGHTNING_SPRITE), 1.0),
            ..default()
        }
        )

    );

}



//Tick Lifettime timers

pub fn handle_lifetime_timers(
    mut commands: Commands,
    time: Res<Time>,
    mut timed_entity_query : Query<(Entity, &mut EffectLifetimeTimer)>
) {

    for (timed_entity, mut lifetime_timer) in &mut timed_entity_query.iter_mut(){
        lifetime_timer.0.tick(time.delta());
        if lifetime_timer.0.just_finished(){
            commands.entity(timed_entity).despawn();
        }
    }

}