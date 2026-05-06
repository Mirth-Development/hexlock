use bevy::prelude::*;
use crate::features::controls::messages::QuitGame;
use crate::features::lock::messages::CatchTumbler;
use crate::features::lockpick::messages::LockpickAction;
use super::super::lock::components::LockComponent;

//Move chamber focus
pub fn user_control_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut pick_event: MessageWriter<LockpickAction>,
    mut tumbler_event: MessageWriter<CatchTumbler>,
    mut quit_event: MessageWriter<QuitGame>,
) {

    if keyboard_input.just_pressed(KeyCode::KeyW) {
        pick_event.write(LockpickAction::Pick);
        println!("Pick Sent!");
    }

    if keyboard_input.just_pressed(KeyCode::KeyA){
        pick_event.write(LockpickAction::Left);
        println!("Left Sent!");
    }

    if keyboard_input.just_pressed(KeyCode::KeyD){
        pick_event.write(LockpickAction::Right);
        println!("Right Sent!");
    }

    if keyboard_input.just_pressed(KeyCode::KeyQ){
        pick_event.write(LockpickAction::SwitchLast);
        println!("Right Sent!");
    }

    if keyboard_input.just_pressed(KeyCode::KeyE){
        pick_event.write(LockpickAction::SwitchNext);
        println!("Right Sent!");
    }

    if keyboard_input.just_pressed(KeyCode::Space){
        println!("Space Sent!");
        tumbler_event.write(CatchTumbler::Catch);
    }

    if keyboard_input.just_pressed(KeyCode::Escape){
        quit_event.write(QuitGame::Quit);
    }
}
