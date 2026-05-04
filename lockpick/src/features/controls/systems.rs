use bevy::prelude::*;
use crate::features::lockpick::events::LockpickAction;
use super::super::lock::tumblers::systems::move_tumbler_focus;
use super::super::lock::components::LockComponent;
use super::super::lockpick::component::LockpickComponent;

//Load Resources
// pub fn load_lockpick_resource(
//     mut commands: Commands,
// ){
//     //Sanity code
//     println!("Loading Lockpick Resources!");
//
//     //Adds the storage for Tumbler positions into the system
//     commands.insert_resource(TumblerPositionCollection{
//         tumbler_positions: Vec::new(),
//     });
// }


// //Collect positions
// fn lockpick_location_collector(
//     mut commands: Commands,
//     mut tumbler_resource: ResMut<TumblerPositionCollection>,
//     tumbler_query: Query<&Transform, With<TumblerComponent>>
// ) {
//     for tumbler_pos in tumbler_query{
//         tumbler_resource.tumbler_positions.push() //RESUME HERE!
//     }
// }


//Need timer for lockpick activation like in oblivion
//Motion tweens will come later

//Move chamber focus
pub fn user_control_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    lock_component: Query<(&Transform, &LockComponent)>,
    //mut current_focused_tumbler: ResMut<CurrentFocusedTumbler>,
    mut pick_event: MessageWriter<LockpickAction>,
    mut exit: MessageWriter<AppExit>

) {
    let Ok((lock_transform, lock)) = lock_component.single() else {return};
    //let Some(mut curr_tumbler) = current_focused_tumbler else {return};

    if keyboard_input.just_pressed(KeyCode::KeyW) {
        pick_event.write(LockpickAction::Pick);
        println!("Pick Sent!");
    }
    if keyboard_input.just_pressed(KeyCode::KeyA){
        pick_event.write(LockpickAction::Left);
        println!("Left Sent!");

        //move_tumbler_focus(MovementDirection::Left, &mut current_focused_tumbler, lock.num_of_tumblers)
    }
    if keyboard_input.just_pressed(KeyCode::KeyD){
        pick_event.write(LockpickAction::Right);
        println!("Right Sent!");
        //move_tumbler_focus(MovementDirection::Right, &mut current_focused_tumbler, lock.num_of_tumblers)
    }
    if keyboard_input.just_pressed(KeyCode::Space){
        println!("Space Sent!");
    }
    if keyboard_input.just_pressed(KeyCode::Escape){
        exit.write(AppExit::Success);
    }

}

