use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, MessageWriter};
use crate::features::lockpick::messages::{LockpickAction};

pub fn normal_pick_schema(
    keyboard_input: &ButtonInput<KeyCode>,
    pick_event: &mut MessageWriter<LockpickAction>,
){

    if keyboard_input.just_pressed(KeyCode::KeyW) {
        pick_event.write(LockpickAction::Pick);
        println!("Pick Sent!");
    }
}

pub fn electric_pick_schema(
    keyboard_input: &ButtonInput<KeyCode>,
    pick_event: &mut MessageWriter<LockpickAction>
){

    if keyboard_input.just_pressed(KeyCode::KeyW){
        pick_event.write(LockpickAction::Charge);
        println!("CHARGING ELECTRIC PICK!");
    }

    if keyboard_input.just_released(KeyCode::KeyW){
        pick_event.write(LockpickAction::Release);
        println!("ELECTRIC CHARGE RELEASE!");
    }





}
