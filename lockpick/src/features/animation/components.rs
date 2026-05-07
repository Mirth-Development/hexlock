use std::time::Duration;
use bevy::prelude::*;


//Added transform to child node containing the sprites so that the animation transform is different from the parent and "functional" transform.
//Add nodes with custom timer, which will automatically trigger on creation, dont forget to tick it


#[derive(Component, Default)]
pub struct Animated;


#[derive(Component)]
#[require(Animated)]
pub struct AnimationShake{
    pub original_translation: Vec3,
    pub animation_timer: Timer,
    pub animation_velocity: Vec3,

}

#[derive(Component)]
#[require(Animated)]
pub struct AnimationFlip{
    pub original_translation: Vec3,
    pub animation_timer: Timer,
}

pub trait Animatable {
    fn new(duration_seconds: f32, original_translation: Vec3) -> Self;
    fn animate_step(&mut self, time: Duration, transform: &mut Transform);

    fn reset_animation_transform(&mut self, transform: &mut Transform);
    fn animation_has_finished(&mut self) -> bool;
}