use bevy::prelude::*;
use rand::RngExt;
use crate::features::lock::spring::systems::HEIGHT_OF_SPRING_SPRITE;
use crate::features::lock::systems::TOP_OF_CHAMBER;
use crate::features::lock::tumblers::components::{FocusedTumblerComponent, SetTumblerComponent, TumblerComponent};
use crate::features::lockpick::events::LockpickAction;
use crate::features::rand::resources::RandomSeed;

pub const HEIGHT_OF_TUMBLER_SPRITE: f32= 92.0;

//Run once after creation to get the positions
// pub fn populate_global_positions(
//     tumbler_query : Query<(&GlobalTransform, &TumblerComponent)>,
//     mut tumbler_position_collection: ResMut<TumblerPositionCollection>
// ) {
//     for (global_pos, tumbler) in tumbler_query{
//         tumbler_position_collection.tumbler_positions.push((tumbler.position, global_pos.translation().x));
//         println!("position {}, location{}", tumbler.position, global_pos.translation().x);
//     }
// }



//This was moved into the lockpick systems
// pub fn move_tumbler_focus(
//     mut commands: Commands,
//     direction: MovementDirection,
//     tumblers: Query<(Entity, &TumblerComponent), Without<FocusedTumblerComponent>>,
//     focused_tumbler_component: Query<(Entity,&TumblerComponent), With<FocusedTumblerComponent>>,
//     mut current_tumbler_pos: u32, //From lockpick component
//     number_of_tumblers: u32 //total # of tumblers
// ){
//
//     let Ok((focused_entity ,focused_tumbler)) = focused_tumbler_component.single() else {return};
//
//
//     match direction {
//         MovementDirection::Left => {
//             if focused_tumbler.position > 1 {
//                 for (tumbler_entity, tumbler_component) in tumblers{
//                     if tumbler_component.position == current_tumbler_pos - 1 {
//                         commands.entity(focused_entity).remove::<FocusedTumblerComponent>();
//                         commands.entity(tumbler_entity).insert(FocusedTumblerComponent);
//                         current_tumbler_pos -= 1;
//                     }
//                 }
//             }
//
//         },
//         MovementDirection::Right => {
//             if focused_tumbler.position < number_of_tumblers {
//                 for (tumbler_entity, tumbler_component) in tumblers{
//                     if tumbler_component.position == current_tumbler_pos + 1 {
//                         commands.entity(focused_entity).remove::<FocusedTumblerComponent>();
//                         commands.entity(tumbler_entity).insert(FocusedTumblerComponent);
//                         current_tumbler_pos += 1;
//                     }
//                 }
//             }
//         }
//
//     }
// }



pub fn handle_pick_message(
    mut actions: MessageReader<LockpickAction>,
    mut random: ResMut<RandomSeed>,
    mut focused_tumbler_query: Query<(Entity, &mut TumblerComponent), With<FocusedTumblerComponent>>,
    check_set: Query<(), With<SetTumblerComponent>> //Call all set elements

){
    let Ok((tumbler_entity, mut tumbler)) = focused_tumbler_query.single_mut() else {return};

    for action in actions.read(){
        match action {
            LockpickAction::Pick => {
                if !check_set.contains(tumbler_entity){
                    let rand = random.RandomNumberGenerator.random_range(0..=2);
                    tumbler.velocity.y = 150.0 + (100.0 * rand as f32);
                }
            }
            _ => {
                continue
            }
        }
    }
}

// pub fn move_tumbler_up (
//     time: Res<Time>,
//     force: i32,
//     mut tumbler: &mut TumblerComponent,
//     mut tumbler_transform: &mut Transform
// ) {
//     if tumbler_transform.translation.y != (TOP_OF_CHAMBER + (HEIGHT_OF_TUMBLER_SPRITE/2.0)){
//         tumbler.velocity.y = force as f32 * 10.0;
//     };
// }

pub fn tumbler_movement(
    time: Res<Time>,
    mut tumblers: Query<(&mut Transform, &mut TumblerComponent)>,
) {
    let top = (TOP_OF_CHAMBER);
    let bottom = (TOP_OF_CHAMBER-(HEIGHT_OF_TUMBLER_SPRITE /2.0)-(HEIGHT_OF_SPRING_SPRITE/2.0));

    for (mut transform, mut tumbler) in &mut tumblers {
        if transform.translation.y + (HEIGHT_OF_TUMBLER_SPRITE/2.0) >= top{
            tumbler.velocity.y *= -1.0;
        } else if transform.translation.y + (HEIGHT_OF_TUMBLER_SPRITE/2.0) < bottom{
            tumbler.velocity.y = 0.0;
            transform.translation.y = bottom - (HEIGHT_OF_TUMBLER_SPRITE /2.0);
        }
        transform.translation += tumbler.velocity * time.delta_secs();
    }

}