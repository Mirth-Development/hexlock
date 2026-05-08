use std::f32::consts::PI;
use std::time::Duration;
use bevy::prelude::*;
use bevy::time::TimerMode::Once;
use rand::prelude::{ SliceRandom};
use crate::features::game_controller::components::{ChargeBarMarker, ChargeLoadingMarker, MagicArrowMarker, TumblerChamberNumberComponent};
use crate::features::game_controller::messages::GameStateMessage;
use crate::features::game_controller::resources::{GameResourceHandles, TumblerOrdering};
use crate::features::interface::definitions::{Interfaces, StateHistory};
use crate::features::lock::components::{GameObjectAnchorMarker, LockComponent, TumblerChamberComponent};
use crate::features::lock::tumblers::components::{SetTumblerComponent, TumblerComponent};
use crate::features::lock::tumblers::systems::TUMBLER_SET_RELEASE_VELOCITY;
use crate::features::lockpick::component::LockpickComponent;
use crate::features::lockpick::messages::{ChargeLockpick, HexDirection};
use crate::features::lockpick::resources::LockpickElectricCharge;
use crate::features::lockpick::systems::LOCKPICK_HEAD_OFFSET;
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
    let magic_arrow_handle: Handle<Image> = asset_server.load("images/Magic_Arrow.png");


    commands.insert_resource(
        GameResourceHandles {
            charge_bar: charge_bar_handle,
            charge: charge_handle,
            magic_arrow: magic_arrow_handle
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


    commands.insert_resource(ordering);

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

//Spawn after lock, start of lock changes with config changes
pub fn spawn_magic_arrow (
    mut commands: Commands,
    game_resource_handles: Res<GameResourceHandles>,
) {

    commands.spawn((
        Sprite::from_image(game_resource_handles.magic_arrow.clone()),
        //Visibility::Hidden, //want to see it for now
        MagicArrowMarker,
        Transform{
            translation: Vec3::splat(0.0),
            ..default()
        },
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
    tumbler_chamber_query: Query<(Entity), With<TumblerChamberComponent>>
) {
    let Ok(lock) = lock_component.single() else {return};

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

        // Wire that entity back onto the tumbler.
        if let Ok((_, mut tumbler)) = tumblers.get_mut(tumbler_entity) {
            tumbler.order_num_entity = num_entity_id;
        }
    }


    // Lookup:
    // let tumbler_entity = tumbler_by_position[target_position as usize];
    //
    //
    // let mut position = 0;
    // for (entity, tumbler_chamber) in tumbler_chamber_query {
    //     // let mut tumbler_entity= None;
    //     // for (entity, tumbler) in tumblers.iter(){
    //     //     if number_lottery[position] == tumbler.position{
    //     //         tumbler_entity = Some(entity);
    //     //     }
    //     // }
    //     //
    //     // if tumbler_entity == None {
    //     //     panic!("NO ENTITY");
    //     // } else {
    // let mut num_entity_id = Entity::PLACEHOLDER; //Had to switch to this else i get the parent entity
    //     commands.entity(entity).with_children(|parent_node| {
    //     num_entity_id = parent_node.spawn((
    //         Sprite::from_image(asset_server.load(helper_get_number_handle(number_lottery[position]))),
    //         TumblerChamberNumberComponent,
    //         Transform {
    //             translation: Vec3::new(0.0, -350.0, 1.0),
    //             scale: Vec3::new(0.7, 0.7, 0.7),
    //             ..default()
    //         },
    //     )).id();
    // });
    //     for (mut tumbler_component) in tumblers.iter_mut(){
    //         if tumbler_component.position == number_lottery[i] {
    //             tumbler_component.order_num_entity = num_entity_id;
    //             break;
    //         }
    //
    //     }
    //
    //     position += 1;
    // }
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
            println!("each tumbler after position {} falls out", tumbler.position);

            // Set full opacity
            if let Ok(mut sprite) = tumbler_number_query.get_mut(tumbler.order_num_entity) {
                sprite.color = Color::srgba(1.0, 1.0, 1.0, 1.0);
            }

            tumbler.timer.reset();
            tumbler.timer.pause();
            tumbler.velocity.y = TUMBLER_SET_RELEASE_VELOCITY;
            commands.entity(entity).remove::<SetTumblerComponent>();
        }
    }

    // for (entity, mut tumbler) in tumbler_component_query.iter_mut() {
    //     if !set_tumbler_component.contains(entity) {
    //         println!("not set");
    //         continue;
    //     }
    //     if tumbler_order.order[(tumbler.position - 1) as usize] < tumbler_order.current_position {
    //         println!("below current position");
    //         continue;
    //     }
    //
    //     let Ok(mut sprite) = tumbler_number_query.get_mut(tumbler.order_num_entity) else {
    //         println!("sprite not got");
    //         continue;
    //     };
    //
    //     let secs = if tumbler.timer.is_finished() {
    //         0.0
    //     } else {
    //         tumbler.timer.remaining_secs()
    //     };
    //
    //     println!("half transparency color!");
    //     sprite.color = Color::srgba(1.0, 1.0, 1.0, 0.5);
    //
    //     if secs > 1.0 {
    //         println!("pos: {} bigger than cur: {}", tumbler.position, tumbler_order.current_position);
    //         println!("full transparency color!");
    //         sprite.color = Color::srgba(1.0, 1.0, 1.0, 1.0);
    //         tumbler.timer.finish();
    //     }
    // }
}

pub fn move_magic_arrow(
    mut magic_arrow: Query<(&mut Transform), With<MagicArrowMarker>>,
    mut anchor_point: Query<(&GlobalTransform), With<GameObjectAnchorMarker>>,
    mut magic_arrow_action: MessageReader<HexDirection>
){
    let Ok(mut magic_arrow_transform) = magic_arrow.single_mut() else {return};
    let Ok(anchor_point_transform) = anchor_point.single() else {return};

    let offset = vec3(300.0, 300.0, 0.0);

    if magic_arrow_transform.translation - offset != anchor_point_transform.translation(){
        magic_arrow_transform.translation = anchor_point_transform.translation() + offset;
    }

    for action in magic_arrow_action.read(){
        match action {
            HexDirection::Up => {
                magic_arrow_transform.rotation = Quat::from_rotation_z(-(PI/2.0) )
            }
            HexDirection::Down => {
                magic_arrow_transform.rotation = Quat::from_rotation_z((-(3.0*PI)/2.0) )}
            HexDirection::Left => {
                magic_arrow_transform.rotation = Quat::from_rotation_z(0.0)
            }
            HexDirection::Right => {
                magic_arrow_transform.rotation = Quat::from_rotation_z(-PI)
            }
        }
    }


}


pub fn charge_charge_bar(
    //mut commands: Commands,
    mut charge_bar_query: Query<(&mut Visibility, &mut Transform), With<ChargeBarMarker>>,
    mut charge_loading_bar_query: Query<(&mut Transform, &mut Sprite), (With<ChargeLoadingMarker>, Without<ChargeBarMarker>, Without<LockpickComponent>)>,
    mut charge_actions: MessageReader<ChargeLockpick>,
    lockpick_electric_charge: Res<LockpickElectricCharge>,
    lockpick_component: Query<&Transform, (With<LockpickComponent>, Without<ChargeBarMarker>)>
){
    let Ok(lockpick_transform) = lockpick_component.single() else {return};
    let Ok((mut charge_loading_bar_transform, mut charge_loading_bar_sprite)) = charge_loading_bar_query.single_mut() else {return};
    let Ok((mut charge_bar_visiblity, mut charge_bar_transform)) = charge_bar_query.single_mut() else {return};
    let charge_offset = vec3(LOCKPICK_HEAD_OFFSET - CHARGE_BAR_SPRITE_WIDTH - 30.0, CHARGE_BAR_SPRITE_HEIGHT + 30.0, 0.0);

    let scale = (CHARGE_BAR_SPRITE_WIDTH/CHARGE_LOADING_SPRITE_WIDTH)*(lockpick_electric_charge.current_charge/lockpick_electric_charge.max_charge);

    let mut is_ready = 1.0;
    if lockpick_electric_charge.current_charge < lockpick_electric_charge.max_charge /4.0 {
        is_ready = 0.0;
    }

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
    //Show red when not charged enough
    charge_loading_bar_sprite.color = Color::srgb(1.0, is_ready, is_ready);
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