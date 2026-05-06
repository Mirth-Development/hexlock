use bevy::prelude::*;
use bevy::time::TimerMode::Once;
use crate::features::game_controller::messages::GameStateMessage;
use crate::features::interface::definitions::{InterfaceStates, StateHistory};
use crate::features::lock::components::LockComponent;
use crate::features::lock::tumblers::components::SetTumblerComponent;

pub fn check_game_state (
    //remove this and add in the Game timer

    mut writer: MessageWriter<GameStateMessage>,
    lock_query: Query<&LockComponent>,
    set_tumblers: Query<(), With<SetTumblerComponent>>

){
    //REPLACE WITH THE GAME TIMER ASSET! The following timer doesnt tick
    let game_timer = Timer::from_seconds(1.0, Once);


    let Ok(lock) = lock_query.single() else { return };
    //check if the number of set tumblers is equal to the number of tumblers
    if set_tumblers.iter().count() as u32 == lock.num_of_tumblers{
        writer.write(GameStateMessage::Win);
    } else if game_timer.is_finished(){
        writer.write(GameStateMessage::Lose);
    }
}

pub fn handle_game_state(
    mut actions: MessageReader<GameStateMessage>,
    get_state: Res<State<InterfaceStates>>,
    mut next_state: ResMut<NextState<InterfaceStates>>,
    mut state_history: ResMut<StateHistory>,
) {
    for action in actions.read() {
        match action {
            GameStateMessage::Win => {
                //Go to next level on completion, WinScreen is a "level"
                //HANDLE ATTRIBUTE CARD SCENE HERE?
                let current_state = get_state.get();
                state_history.clear();
                let state_to_set = match current_state{
                    InterfaceStates::Level1 => {
                        InterfaceStates::Level2
                    }
                    InterfaceStates::Level2 => {
                        InterfaceStates::Level3
                    }
                    InterfaceStates::Level3 => {
                        InterfaceStates::Level4
                    }
                    InterfaceStates::Level4 => {
                        InterfaceStates::Level5
                    }
                    InterfaceStates::Level5 => {
                        InterfaceStates::WinScreen
                    }
                    _ => {
                        panic!("You shouldnt be winning on this state")
                    }
                };
                next_state.set(state_to_set);

            }
            GameStateMessage::Lose => {
                state_history.clear();
                next_state.set(InterfaceStates::StartMenu);
            }

        }
    }
}