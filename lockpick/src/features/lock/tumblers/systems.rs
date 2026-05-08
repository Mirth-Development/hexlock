use bevy::prelude::*;
use crate::features::animation::components::{Animatable, Animated, AnimationShake};
use crate::features::lock::spring::systems::HEIGHT_OF_SPRING_SPRITE;
use crate::features::lock::systems::TOP_OF_CHAMBER;
use crate::features::lock::tumblers::components::{SetTumblerComponent, TumblerComponent, TumblerRustComponent};
use crate::features::lock::tumblers::event::BreakRust;
use crate::features::lock::tumblers::resources::TumblerSize;
use super::messages::TumblerTimerMessage;

pub const HEIGHT_OF_SMALL_TUMBLER_SPRITE: f32= 80.0;
pub const HEIGHT_OF_MEDIUM_TUMBLER_SPRITE: f32= 150.0;
pub const HEIGHT_OF_LARGE_TUMBLER_SPRITE: f32= 220.0;


pub const TUMBLER_SET_RELEASE_VELOCITY: f32= -150.0;

pub const TUMBLER_DEFAULT_SET_TIME: f32= 20.0;

pub fn tumbler_movement(
    time: Res<Time>,
    // tumbler_spring_pairings: Res<TumblerSpringPairings>,
    // springs: Query<(&SpringComponent)>,
    mut tumblers: Query<(Entity, &mut Transform, &mut TumblerComponent)>,
) {

    for (_entity, mut transform, mut tumbler) in &mut tumblers {

        let top = TOP_OF_CHAMBER;
        let bottom: f32;
        let height = match tumbler.size {
            TumblerSize::Small =>{
                bottom = TOP_OF_CHAMBER - (HEIGHT_OF_SMALL_TUMBLER_SPRITE / 2.0) - (HEIGHT_OF_SPRING_SPRITE / 2.0) - 105.0; //Add offset to get each equal to medium
                HEIGHT_OF_SMALL_TUMBLER_SPRITE
            },
            TumblerSize::Medium =>{
                bottom = TOP_OF_CHAMBER - (HEIGHT_OF_MEDIUM_TUMBLER_SPRITE / 2.0) - (HEIGHT_OF_SPRING_SPRITE / 2.0);
                HEIGHT_OF_MEDIUM_TUMBLER_SPRITE
            },
            TumblerSize::Large =>{
                bottom = TOP_OF_CHAMBER - (HEIGHT_OF_LARGE_TUMBLER_SPRITE / 2.0) - (HEIGHT_OF_SPRING_SPRITE / 2.0) + 105.0; //Add offset to get each equal to medium
                HEIGHT_OF_LARGE_TUMBLER_SPRITE
            }
        };


        if transform.translation.y + (height /2.0) > top{
            //Prevent tumbler from getting caught in an inversion of itself
            if tumbler.velocity.y > 0.0 {
                tumbler.velocity.y *= -1.0;
            }
        } else if transform.translation.y + (height /2.0) < bottom{
            tumbler.velocity.y = 0.0;
            transform.translation.y = bottom - (height /2.0);
        }

        transform.translation += (tumbler.velocity)* time.delta_secs();
    }

}


pub fn timer_tumbler_finished (
    time: Res<Time>,
    mut commands: Commands,
    mut tumbler_query: Query<(Entity , &mut TumblerComponent, &Children), With<SetTumblerComponent>>,
    mut animated_sprite_query: Query<(&mut Sprite, Has<AnimationShake>), With<Animated>>, //Can get a bool on whether a component exists or not

) {


    for (tumbler_entity, mut tumbler, tumbler_children) in &mut tumbler_query{
        //println!("Time:{}, Tumbler pos:{}", tumbler.timer.remaining_secs(), tumbler.position);

        if tumbler.timer.is_finished(){
            println!("Timer at {} Finished!", tumbler.position);
            tumbler.timer.reset();
            tumbler.timer.pause();
            tumbler.velocity.y = TUMBLER_SET_RELEASE_VELOCITY;
            commands.entity(tumbler_entity).remove::<SetTumblerComponent>();

        } else if tumbler.timer.remaining_secs() < 2.0{
            tumbler.timer.tick(time.delta());
            for child in tumbler_children.iter(){
                if let Ok((_sprite, is_shaking)) = animated_sprite_query.get_mut(child) {
                    //commands.entity(child).remove::<Sprite>(); //test - works
                    if !is_shaking{
                        commands.entity(child).insert(AnimationShake::new(0.5, Vec3::splat(0.0), TimerMode::Once));
                    }

                }
            }

        } else {
            tumbler.timer.tick(time.delta());
        }
    }
}

 pub fn handle_tumbler_set (
     check_set: Query<(), With<SetTumblerComponent>>, //Call all set elements
     mut commands: Commands,
     mut actions: MessageReader<TumblerTimerMessage>,
     mut tumblers: Query<(Entity, &mut Transform, &mut TumblerComponent)>
 ){

     for action in actions.read(){
         let Ok((focused_entity, mut focused_transform, mut focused_tumbler)) = tumblers.get_mut(action.0) else {
             println!("FAILED TO HANDLE SETTING TUMBLER!");
             return
         };
         if !check_set.contains(focused_entity){
             println!("Setting tumbler {}", focused_tumbler.position);
             let height = match focused_tumbler.size {
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
             focused_tumbler.velocity = Vec3::splat(0.0);
             focused_transform.translation.y = TOP_OF_CHAMBER - (height / 2.0);
             focused_tumbler.timer.reset();
             focused_tumbler.timer.unpause();
             commands.entity(action.0).insert(SetTumblerComponent);

         }

     }

 }


//events
pub fn on_break_rust(
    rust_trigger_entity: On<BreakRust>,
    mut commands: Commands,
    mut tumbler_rust_component: Query<(Entity,&mut TumblerRustComponent)>
){
    for (rust_component_entity,mut rust_component) in tumbler_rust_component.iter_mut() {
        if rust_trigger_entity.id == rust_component_entity{
            if rust_component.hits <= 1 {
                commands.entity(rust_component_entity).despawn()
            } else {
                rust_component.hits -= 1;
            }
        }
    }

}
