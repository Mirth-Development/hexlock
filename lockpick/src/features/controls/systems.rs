use bevy::prelude::*;
use crate::features::controls::systems_for_control_schemas::{electric_pick_schema, magic_pick_schema, normal_pick_schema};
use crate::features::lock::messages::CatchTumbler;
use crate::features::interface::definitions::*;
use crate::features::lockpick::component::LockpickComponent;
use crate::features::lockpick::messages::{HexDirection, LockpickAction};
use crate::features::lockpick::resources::LockpickType;


//Move chamber focus
pub fn user_control_system(
    lockpick_query: Query<&LockpickComponent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut pick_event: MessageWriter<LockpickAction>,
    mut magic_event: MessageWriter<HexDirection>,
    mut tumbler_event: MessageWriter<CatchTumbler>,
    mut next_state: ResMut<NextState<Interfaces>>,
    mut state_history: ResMut<StateHistory>,

) {

    if let Ok(lockpick) = lockpick_query.single() {
        match lockpick.lockpick_type {
            LockpickType::Normal => {
                normal_pick_schema(
                    &keyboard_input,
                    &mut pick_event
                );
            },
            LockpickType::Electric => {
                electric_pick_schema(
                    &keyboard_input,
                    &mut pick_event
                );
            },
            LockpickType::Magic => {
                magic_pick_schema(
                    &keyboard_input,
                    &mut pick_event,
                    &mut magic_event,
                );
            }
        }

    }
        //Let controls still run if there is no lockpick


    if keyboard_input.just_pressed(KeyCode::KeyA){
        pick_event.write(LockpickAction::Left);
        //println!("Left Sent!");
    }

    if keyboard_input.just_pressed(KeyCode::KeyD){
        pick_event.write(LockpickAction::Right);
        //println!("Right Sent!");
    }

    if keyboard_input.just_pressed(KeyCode::KeyQ){
        pick_event.write(LockpickAction::SwitchLast);
        //println!("Switch Last");
    }

    if keyboard_input.just_pressed(KeyCode::KeyE){
        pick_event.write(LockpickAction::SwitchNext);
        //println!("Switch Next");
    }

    if keyboard_input.just_pressed(KeyCode::Space){
        //println!("Space Sent!");
        tumbler_event.write(CatchTumbler::Catch);
    }

    if keyboard_input.just_pressed(KeyCode::Escape){
        state_history.clear();
        next_state.set(Interfaces::StartMenu);
    }
}
