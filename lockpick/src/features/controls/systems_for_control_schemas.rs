use bevy::input::ButtonInput;

use bevy::prelude::{KeyCode, MessageWriter};
use crate::features::lockpick::messages::{HexDirection, LockpickAction};


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

pub fn magic_pick_schema(
    keyboard_input: &ButtonInput<KeyCode>,
    mut pick_event: &mut MessageWriter<LockpickAction>,
    mut hex_event: &mut MessageWriter<HexDirection>
){

    if keyboard_input.just_pressed(KeyCode::KeyW){
        pick_event.write(LockpickAction::Hex);
        println!("Locking Hex!");
    }
    if keyboard_input.just_pressed(KeyCode::KeyI) || keyboard_input.just_pressed(KeyCode::ArrowUp)  { //up
        hex_event.write(HexDirection::Up);
        println!("Up");
    }
    if keyboard_input.just_pressed(KeyCode::KeyJ) || keyboard_input.just_pressed(KeyCode::ArrowLeft){ //left
        hex_event.write(HexDirection::Left);
        println!("Left");
    }
    if keyboard_input.just_pressed(KeyCode::KeyK) || keyboard_input.just_pressed(KeyCode::ArrowDown){ //down
        hex_event.write(HexDirection::Down);
        println!("Down");
    }
    if keyboard_input.just_pressed(KeyCode::KeyL) || keyboard_input.just_pressed(KeyCode::ArrowRight){ //right
        hex_event.write(HexDirection::Right);
        println!("Right");
    }


}
