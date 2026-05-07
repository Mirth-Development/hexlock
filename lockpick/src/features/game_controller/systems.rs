use bevy::prelude::*;
use bevy::time::TimerMode::Once;
use crate::features::game_controller::components::{ChargeBarMarker, ChargeLoadingMarker};
use crate::features::game_controller::messages::GameStateMessage;
use crate::features::game_controller::resources::{GameResourceHandles};
use crate::features::interface::definitions::{Interfaces, StateHistory};
use crate::features::lock::components::LockComponent;
use crate::features::lock::tumblers::components::SetTumblerComponent;
use crate::features::lockpick::component::LockpickComponent;
use crate::features::lockpick::messages::ChargeLockpick;
use crate::features::lockpick::resources::LockpickElectricCharge;
use crate::features::lockpick::systems::LOCKPICK_HEAD_OFFSET;

const CHARGE_BAR_SPRITE_WIDTH: f32 = 150.0;
const CHARGE_BAR_SPRITE_HEIGHT: f32 = 32.0;

const CHARGE_LOADING_SPRITE_WIDTH: f32 = 17.0;

pub fn load_game_controller_sprites(

    mut commands: Commands, asset_server: Res<AssetServer>
) {
    //Sanity code
    println!("Loading GameResourceSprites!");

    let charge_bar_handle: Handle<Image> = asset_server.load("images/Charge_Bar.png");
    let charge_handle: Handle<Image> = asset_server.load("images/Charge_Sprite.png");


    commands.insert_resource(
        GameResourceHandles {
            charge_bar: charge_bar_handle,
            charge: charge_handle
        });

}

//Must spawn after Lockpick
pub fn spawn_charge_bar (
    mut commands: Commands,
    game_resource_handles: Res<GameResourceHandles>,
) {
    //let Ok(lockpick_transform) = lockpick_component.single() else {return};
    //let charge_offset = vec3(LOCKPICK_HEAD_OFFSET - CHARGE_BAR_SPRITE_WIDTH - 30.0, CHARGE_BAR_SPRITE_HEIGHT + 30.0, 0.0);

    commands.spawn((
        Sprite::from_image(game_resource_handles.charge_bar.clone()),
        Visibility::Hidden,
        ChargeBarMarker,
        Transform{
            translation: Vec3::splat(0.0),
            ..default()
        },
        children![(
            Sprite::from_image(game_resource_handles.charge.clone()),
            ChargeLoadingMarker,
            Transform::from_xyz(0.0,0.0,0.0)
        )]
        )

    );


}

pub fn charge_chargebar(
    //mut commands: Commands,
    mut charge_bar_query: Query<(&mut Visibility, &mut Transform), With<ChargeBarMarker>>,
    mut charge_loading_bar_query: Query<&mut Transform, (With<ChargeLoadingMarker>, Without<ChargeBarMarker>, Without<LockpickComponent>)>,
    mut charge_actions: MessageReader<ChargeLockpick>,
    lockpick_electric_charge: Res<LockpickElectricCharge>,
    lockpick_component: Query<&Transform, (With<LockpickComponent>, Without<ChargeBarMarker>)>
){
    let Ok(lockpick_transform) = lockpick_component.single() else {return};
    let Ok(mut charge_loading_bar_transform) = charge_loading_bar_query.single_mut() else {return};
    let Ok((mut charge_bar_visiblity, mut charge_bar_transform)) = charge_bar_query.single_mut() else {return};
    let charge_offset = vec3(LOCKPICK_HEAD_OFFSET - CHARGE_BAR_SPRITE_WIDTH - 30.0, CHARGE_BAR_SPRITE_HEIGHT + 30.0, 0.0);

    let scale = (CHARGE_BAR_SPRITE_WIDTH/CHARGE_LOADING_SPRITE_WIDTH)*(lockpick_electric_charge.current_charge/lockpick_electric_charge.max_charge);


    for action in charge_actions.read(){
        match action {
            ChargeLockpick::Charge => {
                *charge_bar_visiblity = Visibility::Visible;
            }
            ChargeLockpick::Release => {
                *charge_bar_visiblity = Visibility::Hidden;
            }
        }
    }
    charge_bar_transform.translation = lockpick_transform.translation + charge_offset;
    charge_loading_bar_transform.scale.x = scale;










    //     Sprite::from_image(game_resource_handles.charge_bar.clone()),
    //     Visibility::Hidden,
    //     ChargeBar{
    //         charge_max: lockpick_electric_charge.max_charge,
    //         charge: lockpick_electric_charge.current_charge,
    //     },
    //     Transform{
    //         translation: lockpick_transform.translation + charge_offset,
    //         ..default()
    //     },
    //     children![(
    //         Sprite::from_image(game_resource_handles.charge.clone()),
    //         Transform::from_xyz(0.0,0.0,0.0)
    //     )]
    // )
    //
    // );
}

//Message

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
    get_state: Res<State<Interfaces>>,
    mut next_state: ResMut<NextState<Interfaces>>,
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
                    Interfaces::Level1 => {
                        Interfaces::Level2
                    }
                    Interfaces::Level2 => {
                        Interfaces::Level3
                    }
                    Interfaces::Level3 => {
                        Interfaces::Level4
                    }
                    Interfaces::Level4 => {
                        Interfaces::Level5
                    }
                    Interfaces::Level5 => {
                        Interfaces::Won
                    }
                    _ => {
                        panic!("You shouldnt be winning on this state")
                    }
                };
                next_state.set(state_to_set);

            }
            GameStateMessage::Lose => {
                state_history.clear();
                next_state.set(Interfaces::StartMenu);
            }

        }
    }
}
