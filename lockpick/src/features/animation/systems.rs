use std::time::Duration;
use bevy::ecs::component::Mutable;
use bevy::prelude::*;
use crate::features::animation::components::{Animated, Animatable, AnimationShake, AnimationFlip};


//this function will check all Animatable objects and activate their animation


pub fn animation_controller<T: Component<Mutability = Mutable> + Animatable>(
    //Generics have to declare their mutability in Bevy 18 ~<~
    mut commands: Commands,
    time: Res<Time>,
    mut animatable: Query<(Entity, &mut T, &mut Transform,), With<Animated>>,

) {
    for (sprite_entity, mut animated_sprite, mut sprite_transform) in &mut animatable {
        if animated_sprite.animation_has_finished() && !animated_sprite.repeats() {
            animated_sprite.reset_animation_transform(&mut sprite_transform);
            commands.entity(sprite_entity).remove::<T>();
        } else {
            animated_sprite.animate_step(time.delta(), &mut sprite_transform);
        };
    }
}


//Implementations
impl Animatable for AnimationShake {
    fn new(duration_seconds: f32, original_translation: Vec3, timer_mode: TimerMode) -> Self {
        Self{
            original_translation,
            animation_timer: Timer::from_seconds(duration_seconds, timer_mode),
            animation_velocity: vec3(200.0,0.0,0.0),
        }
    }
    fn animate_step(&mut self, delta: Duration, transform: &mut Transform){
        //println!("Animating!");
        self.animation_timer.tick(delta);
        let left_bound = -5.0;
        let right_bound = 5.0;
        
        if transform.translation.x > right_bound {
            transform.translation.x = right_bound;
            self.animation_velocity.x  *= -1.0;
        } else if transform.translation.x < left_bound {
            transform.translation.x = left_bound;
            self.animation_velocity.x *= -1.0;
        }
        //println!("x:{}, speed:{}, Right bound: {}, Left bound: {}", transform.translation.x, self.animation_velocity.x, right_bound, left_bound);
        transform.translation.x += self.animation_velocity.x * delta.as_secs_f32();



    }

    fn repeats(&mut self) -> bool {
        if self.animation_timer.mode() == TimerMode::Repeating {
            true
        } else {
            false
        }
    }

    fn reset_animation_transform(&mut self, transform: &mut Transform) {
        transform.translation = self.original_translation;
    }
    fn animation_has_finished(&mut self) -> bool {

        self.animation_timer.is_finished()
    }
}
// Pick switching will be instant due to time restraint

impl Animatable for AnimationFlip {
    fn new(duration_seconds: f32, offset_translation: Vec3, timer_mode: TimerMode) -> Self {
        Self{
            original_translation: offset_translation,
            animation_timer: Timer::from_seconds(duration_seconds, timer_mode),
            //animation_velocity: vec3(0.0,0.0,0.0),
        }
    }

    fn animate_step(&mut self, delta: Duration, transform: &mut Transform){
        println!("Animating!");
        self.animation_timer.tick(delta);
        if (self.animation_timer.elapsed_secs() * 10.0).trunc() as i32 % 2 != 0{
            transform.scale.y *= -1.0;
        }

    }

    fn repeats(&mut self) -> bool {
        if self.animation_timer.mode() == TimerMode::Repeating {
            true
        } else {
            false
        }
    }

    fn reset_animation_transform(&mut self, transform: &mut Transform) {
        transform.translation = self.original_translation;
    }
    fn animation_has_finished(&mut self) -> bool {

        self.animation_timer.is_finished()
    }
}
