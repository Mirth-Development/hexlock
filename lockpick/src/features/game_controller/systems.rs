use bevy::prelude::*;
use bevy::time::TimerMode::Once;
use rand::prelude::{ SliceRandom};
use crate::features::animation::components::Animated;
use crate::features::game_controller::components::{ChargeBarMarker, ChargeLoadingMarker, TumblerChamberNumberComponent};
use crate::features::game_controller::game_timer::definitions::TheTimer;
use crate::features::game_controller::messages::GameStateMessage;
use crate::features::game_controller::resources::{GameResourceHandles, InputtedArrowCode, NumberOfTumblers, TumblerOrdering};
use crate::features::interface::definitions::{Interfaces, StateHistory};
use crate::features::lock::components::{ LockComponent, TumblerChamberComponent};
use crate::features::lock::tumblers::components::{FocusedTumblerComponent, SetTumblerComponent, TumblerComponent, TumblerMagicComponent};
use crate::features::lock::tumblers::resources::Directions;
use crate::features::lock::tumblers::systems::TUMBLER_SET_RELEASE_VELOCITY;
use crate::features::lockpick::component::LockpickComponent;
use crate::features::lockpick::messages::{ChargeLockpick, HexDirection, StartHexCodeInput};
use crate::features::lockpick::resources::LockpickElectricCharge;
use crate::features::lockpick::systems::{shake_tumbler_help_function, LOCKPICK_HEAD_OFFSET};
use crate::features::rand::resources::RandomSeed;

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
            charge: charge_handle,
        });

}

pub fn load_game_controller_resources(

    mut commands: Commands
) {
    //Sanity code
    println!("Loading GameResource!");

    let ordering = TumblerOrdering {
        current_position: 1,
        order: Vec::new()
    };

    let code = InputtedArrowCode {
        inputting: false,
        entered_code: Vec::new(),
    };

    let number_of_tumblers = NumberOfTumblers{
        number_of_tumblers: 4,
    };



    commands.insert_resource(ordering);
    commands.insert_resource(code);
    commands.insert_resource(number_of_tumblers);

}

//Must spawn after Lockpick
pub fn spawn_charge_bar (
    mut commands: Commands,
    game_resource_handles: Res<GameResourceHandles>,
) {
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

pub fn spawn_lock_order (
    mut commands: Commands,
    mut random_seed: ResMut<RandomSeed>,
    asset_server:  Res<AssetServer>,
    mut tumbler_ordering: ResMut<TumblerOrdering>,
    lock_component: Query<&LockComponent>,
    mut tumblers: Query<(Entity, &mut TumblerComponent)>,
    tumbler_chamber_query: Query<Entity, With<TumblerChamberComponent>>
) {
    let Ok(lock) = lock_component.single() else {
        println!("No Lock!");
        return};

    tumbler_ordering.order = Vec::new();
    tumbler_ordering.current_position = 1;
    let mut number_lottery: Vec<u32> = Vec::new();
    for x in 1..=lock.num_of_tumblers {
        number_lottery.push(x)
    }
    println!("{:?}", number_lottery);
    println!("shuffling vec");
    number_lottery.shuffle(&mut random_seed.random_number_generator); //Shuffle Vec!
    println!("{:?}", number_lottery);
    tumbler_ordering.order = number_lottery.clone();

    let mut tumbler_by_position: Vec<Entity> = vec![Entity::PLACEHOLDER; (lock.num_of_tumblers + 1) as usize];
    for (tumbler_entity, tumbler_component) in tumblers.iter() {
        tumbler_by_position[tumbler_component.position as usize] = tumbler_entity;
    }

    for (i, chamber_entity) in tumbler_chamber_query.iter().enumerate() {
        let chamber_position = (i + 1) as u32;
        let displayed_number = number_lottery[i];

        // The tumbler PHYSICALLY in this chamber:
        let tumbler_entity = tumbler_by_position[chamber_position as usize];

        // Get child entity out of closure
        let mut num_entity_id = Entity::PLACEHOLDER;
        commands.entity(chamber_entity).with_children(|parent_node| {
            num_entity_id = parent_node.spawn((
                Sprite::from_image(asset_server.load(helper_get_number_handle(displayed_number))),
                TumblerChamberNumberComponent,
                Transform {
                    translation: Vec3::new(0.0, -350.0, 1.0),
                    scale: Vec3::splat(0.7),
                    ..default()
                },
            )).id();
        });
        println!("Spawned chamber number entity: {:?} for chamber {} displayed {}",
                 num_entity_id, chamber_position, displayed_number);

        // Wire that entity back onto the tumbler.
        if let Ok((_, mut tumbler)) = tumblers.get_mut(tumbler_entity) {
            println!("Wired tumbler at chamber {} -> sprite entity {:?}", chamber_position, num_entity_id);
            tumbler.order_num_entity = num_entity_id;
            println!("[SPAWN] Wired tumbler at chamber {} -> sprite {:?} (tumbler entity {:?}, position {})",
                     chamber_position, num_entity_id, tumbler_entity, tumbler.position);
        }
    }
}

pub fn check_tumbler_order(
    mut commands: Commands,
    set_tumbler_component: Query<(), With<SetTumblerComponent>>,
    mut tumbler_number_query: Query<&mut Sprite, With<TumblerChamberNumberComponent>>,
    mut tumbler_component_query: Query<(Entity, &mut TumblerComponent)>,
    tumbler_order: Res<TumblerOrdering>,
) {
    for (entity, mut tumbler) in tumbler_component_query.iter_mut() {
        if !set_tumbler_component.contains(entity) {
            continue;
        }

        let rank = tumbler_order.order[(tumbler.position - 1) as usize];

        // knock out each tumbler above amount
        if rank >= tumbler_order.current_position {

            match tumbler_number_query.get_mut(tumbler.order_num_entity) {
                Ok(mut sprite) => {

                    sprite.color = Color::srgba(1.0, 1.0, 1.0, 1.0);
                }
                Err(e) => {
                    panic!("{:?}",e)
                }
            }

            tumbler.timer.reset();
            tumbler.timer.pause();
            tumbler.velocity.y = TUMBLER_SET_RELEASE_VELOCITY;
            commands.entity(entity).remove::<SetTumblerComponent>();
        }
    }

}


pub fn charge_charge_bar(
    //mut commands: Commands,
    mut charge_bar_query: Query<( &mut Transform,  &mut Visibility,), With<ChargeBarMarker>>,
    mut charge_loading_bar_query: Query<(&mut Transform, &mut Sprite,), (With<ChargeLoadingMarker>, Without<ChargeBarMarker>, Without<LockpickComponent>)>,
    mut charge_actions: MessageReader<ChargeLockpick>,
    lockpick_electric_charge: Res<LockpickElectricCharge>,
    lockpick_component: Query<&Transform, (With<LockpickComponent>, Without<ChargeBarMarker>)>
){
    let Ok(lockpick_transform) = lockpick_component.single() else {
        println!("No Lockpick Transform");
        return};
    let Ok((mut charge_loading_bar_transform, mut charge_loading_bar_sprite, )) = charge_loading_bar_query.single_mut() else {
        println!("No Charge Loading Bar");
        return};
    let Ok((mut charge_bar_transform, mut charge_loading_bar_visibility)) = charge_bar_query.single_mut() else {
        println!("No Charge Loading Sprite");
        return};
    let charge_offset = vec3(-LOCKPICK_HEAD_OFFSET - CHARGE_BAR_SPRITE_WIDTH - 30.0, CHARGE_BAR_SPRITE_HEIGHT + 30.0, 100.0);

    let scale = (CHARGE_BAR_SPRITE_WIDTH/CHARGE_LOADING_SPRITE_WIDTH)*(lockpick_electric_charge.current_charge/lockpick_electric_charge.max_charge);

    let mut is_ready = 1.0;
    if lockpick_electric_charge.current_charge < lockpick_electric_charge.max_charge /4.0 {
        is_ready = 0.0;
    }

    for action in charge_actions.read(){
        println!("Action found!");
        match action {
            ChargeLockpick::Charge => {
                println!("Make Visible!");
                *charge_loading_bar_visibility = Visibility::Visible;
            }
            ChargeLockpick::Release => {
                println!("Make Invisible!");
                *charge_loading_bar_visibility = Visibility::Hidden;
            }
        }
    }
    //Show red when not charged enough
    charge_loading_bar_sprite.color = Color::srgb(1.0, is_ready, is_ready);
    charge_bar_transform.translation = lockpick_transform.translation + charge_offset;
    charge_loading_bar_transform.scale.x = scale;
}


pub fn enter_arrow_code(
    mut commands: Commands,
    mut arrow_resource : ResMut<InputtedArrowCode>,
    mut lock_pick_query : Query<&mut LockpickComponent>,
    mut start_hex_code_action: MessageReader<StartHexCodeInput>,
    mut focused_tumbler_component: Query<(&mut TumblerComponent, &TumblerMagicComponent, &Children), With<FocusedTumblerComponent>>,
    mut animated_sprite_query: Query<&mut Sprite, With<Animated>>,
    //mut combo_code_query: Query<(&mut ImageNode,&ComboArrow)>,
    mut hex_actions: MessageReader<HexDirection>,

){
    let mut tumbler_weights= 0.0;

    for action in start_hex_code_action.read(){
        arrow_resource.inputting = true;
        tumbler_weights = action.0;
    }

    if arrow_resource.inputting {
        //println!("Entering Code!");
        let Ok((mut focused_tumbler, tumbler_magic, focused_children)) = focused_tumbler_component.single_mut() else {
            println!("No focused tumbler or not magic tumbler!");
            return
        };

        let Ok(mut lockpick) = lock_pick_query.single_mut() else {
            println!("No lockpick!");
            return;
        };

        for action in hex_actions.read(){
            match action {
                HexDirection::Up => {
                    arrow_resource.entered_code.push(Directions::Up);
                }
                HexDirection::Down => {
                    arrow_resource.entered_code.push(Directions::Down);
                }
                HexDirection::Left => {
                    arrow_resource.entered_code.push(Directions::Left);
                }
                HexDirection::Right => {
                    arrow_resource.entered_code.push(Directions::Right);
                }
            }
            for (entered_code, expected_code) in arrow_resource.entered_code.iter().zip(tumbler_magic.arrow_code.iter()){
                if entered_code != expected_code {
                    arrow_resource.entered_code.clear();
                    lockpick.is_moving = false;
                    arrow_resource.inputting = false;
                    shake_tumbler_help_function(focused_children,&mut animated_sprite_query, &mut commands);
                    break;
                }
            }

            if arrow_resource.entered_code.len() > 4 {
                arrow_resource.entered_code.remove(0);
            }

        }
        //println!("input: {:?}", arrow_resource.entered_code);
        if arrow_resource.entered_code == tumbler_magic.arrow_code {
            arrow_resource.entered_code.clear();
            arrow_resource.inputting = false;
            lockpick.is_moving = false;
            focused_tumbler.velocity.y = 200.0 + tumbler_weights;
        }

    }



}


pub fn handle_lose_game(
    game_timer: Res<TheTimer>,
    mut game_state_message: MessageWriter<GameStateMessage>,
){
    if game_timer.chronolog.get_countdown_string(2,2) == "00.00".to_string() {
        game_state_message.write(GameStateMessage::Lose);
    }
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
                        Interfaces::StartMenu
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

pub fn increase_tumbler_amount_per_level(
    mut number_of_tumblers_resource: ResMut <NumberOfTumblers>
) {
    number_of_tumblers_resource.number_of_tumblers += 1;
}


pub fn helper_get_number_handle (num: u32) -> String{
    if num > 9 {
        panic!("Should never go higher than 9 or lower than 0")
    }
    let digit_images: [&str; 10] = [
        "images/0.png",
        "images/1.png",
        "images/2.png",
        "images/3.png",
        "images/4.png",
        "images/5.png",
        "images/6.png",
        "images/7.png",
        "images/8.png",
        "images/9.png",
    ];
    format!("{}",digit_images[num as usize])
}
