use bevy::prelude::*;
use crate::features::lock::spring::systems::HEIGHT_OF_SPRING_SPRITE;
use crate::features::lock::systems::TOP_OF_CHAMBER;
use crate::features::lock::tumblers::components::{SetTumblerComponent, TumblerComponent};
use super::messages::TumblerTimerMessage;

pub const HEIGHT_OF_TUMBLER_SPRITE: f32= 150.0;
pub const TUMBLER_SET_RELEASE_VELOCITY: f32= -150.0;

pub fn tumbler_movement(
    time: Res<Time>,
    mut tumblers: Query<(&mut Transform, &mut TumblerComponent)>,
) {
    let top = (TOP_OF_CHAMBER);
    let bottom = (TOP_OF_CHAMBER-(HEIGHT_OF_TUMBLER_SPRITE /2.0)-(HEIGHT_OF_SPRING_SPRITE/2.0));

    for (mut transform, mut tumbler) in &mut tumblers {
        if transform.translation.y + (HEIGHT_OF_TUMBLER_SPRITE/2.0) > top{
            //Prevent tumbler from getting caught in an inversion of itself
            if tumbler.velocity.y > 0.0 {
                tumbler.velocity.y *= -1.0;
            }
        } else if transform.translation.y + (HEIGHT_OF_TUMBLER_SPRITE/2.0) < bottom{
            tumbler.velocity.y = 0.0;
            transform.translation.y = bottom - (HEIGHT_OF_TUMBLER_SPRITE /2.0);
        }
        transform.translation += tumbler.velocity * time.delta_secs();
    }

}


pub fn tumbler_timer_finished (
    mut commands: Commands,
    mut tumbler_query: Query<(Entity ,&mut TumblerComponent), With<SetTumblerComponent>>,
) {

    for (tumbler_entity, mut tumbler) in &mut tumbler_query{
        if tumbler.timer.is_finished(){
            tumbler.timer.reset();
            tumbler.timer.pause();
            tumbler.velocity.y = TUMBLER_SET_RELEASE_VELOCITY;
            commands.entity(tumbler_entity).remove::<SetTumblerComponent>();

        }
    }
}

// pub fn handle_tumbler_set (
//     mut actions: MessageReader<TumblerTimerMessage>
// ){
//
// }
