use bevy::prelude::*;
use crate::features::game_controller::tumbler_randomizer::systems::get_random_tumbler;
use crate::features::lock::messages::CatchTumbler;
use crate::features::lock::spring::systems::HEIGHT_OF_SPRING_SPRITE;
use crate::features::lock::tumblers::messages::TumblerTimerMessage;
use crate::features::lock::tumblers::resources::TumblerSize;
use crate::features::lock::tumblers::systems::{HEIGHT_OF_LARGE_TUMBLER_SPRITE, HEIGHT_OF_MEDIUM_TUMBLER_SPRITE, HEIGHT_OF_SMALL_TUMBLER_SPRITE, TUMBLER_DEFAULT_SET_TIME};
use crate::features::rand::resources::RandomSeed;
use super::components::{LockComponent, TumblerChamberComponent};
use super::resource::{LockSpriteHandles, TumblerSpringPairings};
use super::tumblers::components::{FocusedTumblerComponent, SetTumblerComponent, TumblerComponent};
use super::spring::components::SpringComponent;

//Hardcoded Sprite Sizes so that they don't have to be sought dynamically, async loading is a pain in the ass
pub const TOP_OF_CHAMBER: f32= 399.0;
const LOCK_START_SPRITE_WIDTH: f32= 875.0;
const TUMBLER_CHAMBER_SPRITE_WIDTH: f32= 150.0;
const LOCK_END_SPRITE_WIDTH: f32= 120.0;
const TUMBLER_SET_THRESHOLD: f32= 10.0;
const LOCK_START_OFFSET: f32 = -170.0;
const LOCK_END_OFFSET: f32 = -80.0;

//Load Resources
pub fn load_sprite_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    //Sanity code
    println!("Loading LockSprites!");

    let start_handle: Handle<Image> = asset_server.load("images/Lock_Start.png");
    let tumbler_section_handle: Handle<Image> = asset_server.load("images/Lock_Tumbler.png");
    let end_handle: Handle<Image> = asset_server.load("images/Lock_End.png");
    let spring_handle: Handle<Image> = asset_server.load("images/Spring.png");
    let tumbler_small_handle: Handle<Image> = asset_server.load("images/Head_Small.png");
    let tumbler_medium_handle: Handle<Image> = asset_server.load("images/Head_Medium.png");
    let tumbler_large_handle: Handle<Image> = asset_server.load("images/Head_Large.png");

    commands.insert_resource(LockSpriteHandles {
        start_sprite: start_handle,
        tumbler_chamber_sprite: tumbler_section_handle,
        end_sprite: end_handle,
        spring_sprite: spring_handle,
        tumbler_small_sprite: tumbler_small_handle,
        tumbler_medium_sprite: tumbler_medium_handle,
        tumbler_large_sprite: tumbler_large_handle,

    });
}

pub fn load_lock_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    //Sanity code
    println!("Loading GameResources!");

    //List all resources required for load on startup here
    commands.insert_resource(TumblerSpringPairings {
        array: Vec::new()
    });
}

//Spawn and Build Lock
pub fn spawn_lock(
    lock_sprite_handles: Res<LockSpriteHandles>,
    mut rng: ResMut<RandomSeed>,
    mut commands: Commands,
    mut tumbler_spring_pairings: ResMut<TumblerSpringPairings>
) {
    //Sanity code
    println!("Building Locks");
    let mut offset: f32 = 0.0;
    //
    let mut tumbler_set_timer = Timer::from_seconds(TUMBLER_DEFAULT_SET_TIME, TimerMode::Once);
    //Pause timer after creation
    tumbler_set_timer.pause();

    //Sprites are spawned centered on their spawn coords, so the offset calculates where to place them
    offset += (LOCK_START_SPRITE_WIDTH / 2.0);

    let mut lock = LockComponent {
        num_of_tumblers: 8, //Max amount for our purposes
        ..default()
    };

    commands.spawn((
        lock,
        Transform::from_xyz(0.0,0.0,0.0),
        Visibility::default()
    )).with_children(|parent_node| {

        //Start of Lock
        parent_node.spawn((
            Sprite::from_image(lock_sprite_handles.start_sprite.clone()),
            Transform::from_xyz(offset, LOCK_START_OFFSET, 0.0),
        ));
        offset += (LOCK_START_SPRITE_WIDTH / 2.0) + (TUMBLER_CHAMBER_SPRITE_WIDTH / 2.0);

        for x in 1..= lock.num_of_tumblers {

            //Spawn Tumbler Chamber
            parent_node.spawn((
                Sprite::from_image(lock_sprite_handles.tumbler_chamber_sprite.clone()),
                TumblerChamberComponent,
                Transform::from_xyz(offset, 0.0, 0.0),
            ));

            //Spawn Tumbler
            let tumbler;
            if x == 1 {
                tumbler = parent_node.spawn((
                    get_random_tumbler(x,tumbler_set_timer.clone(),&mut rng.RandomNumberGenerator,&lock_sprite_handles,),
                    FocusedTumblerComponent,
                    Transform::from_xyz(offset, TOP_OF_CHAMBER-(HEIGHT_OF_MEDIUM_TUMBLER_SPRITE /2.0)-(HEIGHT_OF_SPRING_SPRITE), 0.0),
                )).id();
            } else {
                tumbler = parent_node.spawn((
                    get_random_tumbler(x,tumbler_set_timer.clone(),&mut rng.RandomNumberGenerator,&lock_sprite_handles,),
                    Transform::from_xyz(offset, TOP_OF_CHAMBER - (HEIGHT_OF_MEDIUM_TUMBLER_SPRITE / 2.0) - (HEIGHT_OF_SPRING_SPRITE), 0.0),
                )).id();
            }

            //Spawn_Spring
            let spring = parent_node.spawn((
                Sprite::from_image(lock_sprite_handles.spring_sprite.clone()),
                SpringComponent{
                    position: x
                },
                Transform::from_xyz(offset, TOP_OF_CHAMBER - (HEIGHT_OF_SPRING_SPRITE / 2.0), 0.0),
            )).id();

            tumbler_spring_pairings.array.push((tumbler, spring));
            if x != lock.num_of_tumblers {
                offset += TUMBLER_CHAMBER_SPRITE_WIDTH;
            }
        };
        offset += (TUMBLER_CHAMBER_SPRITE_WIDTH / 2.0) + (LOCK_END_SPRITE_WIDTH / 2.0);

        //Spawn End of Lock
        parent_node.spawn((
            Sprite::from_image(lock_sprite_handles.end_sprite.clone()),
            Transform{
                //scale: Vec3::new(0.3, 0.3, 1.0),
                translation: Vec3::new(offset, -LOCK_END_OFFSET, 0.0),
                ..Default::default()
            },
        ));
        offset += LOCK_END_SPRITE_WIDTH /2.0;

    })

    //Add the offset back into the entity by replacing the Transform of the parent
    .insert(Transform::from_xyz(-offset / 2.0, 0.0, 0.0));
}

pub fn handle_catching_tumblers (
    mut commands: Commands,
    mut actions: MessageReader<CatchTumbler>,
    mut writer: MessageWriter<TumblerTimerMessage>,
    mut tumbler_query: Query<(Entity, &mut Transform, &mut TumblerComponent), With<FocusedTumblerComponent>>,
) {

    let Ok((focused_entity, mut focused_tumbler_transform, mut focused_tumbler)) = tumbler_query.single_mut() else {return};
    //You will see the following match function in a few different functions, I could have put the const in the Component.... but I didnt want to go undo everything :)
    let height = match focused_tumbler.size {
        TumblerSize::Small =>{
            HEIGHT_OF_SMALL_TUMBLER_SPRITE
        },
        TumblerSize::Medium =>{
            HEIGHT_OF_MEDIUM_TUMBLER_SPRITE
        },
        TumblerSize::Large =>{
            HEIGHT_OF_LARGE_TUMBLER_SPRITE
        }
    };
    for action in actions.read(){
        match action {
            CatchTumbler::Catch => {
                if focused_tumbler_transform.translation.y + (height / 2.0) >= (TOP_OF_CHAMBER - TUMBLER_SET_THRESHOLD){
                    writer.write(TumblerTimerMessage(focused_entity));
                } else {
                    //Add time/score reducing code here!
                    if focused_tumbler.velocity.y != (TOP_OF_CHAMBER - (height / 2.0) - (HEIGHT_OF_SPRING_SPRITE / 2.0)){
                        focused_tumbler.velocity.y = -600.0;
                    }
                }
            }
        }
    }
}
