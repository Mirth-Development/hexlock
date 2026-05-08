use bevy::prelude::*;
use rand::RngExt;
use super::components::{GameObjectAnchorMarker, LockComponent, TumblerChamberComponent};
use super::resource::{LockSpriteHandles, TumblerSpringPairings};
use super::tumblers::components::{FocusedTumblerComponent, TumblerComponent, TumblerMagicComponent};
use super::tumblers::resources::Directions;
use super::super::game_controller::rust_randomizer::systems::chance_to_add_rust;
use super::tumblers::components::{SetTumblerComponent};
use crate::features::animation::components::{Animatable, Animated, AnimationShake};
use crate::features::game_controller::tumbler_randomizer::systems::gen_random_tumbler;
use crate::features::lock::messages::CatchTumbler;
use crate::features::lock::spring::systems::{HEIGHT_OF_SPRING_SPRITE, gen_random_spring};
use crate::features::lock::tumblers::messages::TumblerTimerMessage;
use crate::features::lock::tumblers::resources::{TumblerSize, TumblerType};
use crate::features::lock::tumblers::systems::{
    HEIGHT_OF_LARGE_TUMBLER_SPRITE, HEIGHT_OF_MEDIUM_TUMBLER_SPRITE,
    HEIGHT_OF_SMALL_TUMBLER_SPRITE, TUMBLER_DEFAULT_SET_TIME,
};
use crate::features::rand::resources::RandomSeed;
use crate::features::game_controller::game_effects::resources::EffectsSpriteHandles;

//Hardcoded Sprite Sizes so that they don't have to be sought dynamically, async loading is a pain in the ass
pub const TOP_OF_CHAMBER: f32 = 399.0;
const LOCK_START_SPRITE_WIDTH: f32 = 875.0;
const TUMBLER_CHAMBER_SPRITE_WIDTH: f32 = 150.0;
const LOCK_END_SPRITE_WIDTH: f32 = 120.0;
const TUMBLER_SET_THRESHOLD: f32 = 10.0;
const LOCK_START_OFFSET: f32 = -170.0;
const LOCK_END_OFFSET: f32 = -80.0;

//Load Resources
pub fn load_lock_sprite_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
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
) {
    //Sanity code
    println!("Loading GameResources!");

    //List all resources required for load on startup here
    commands.insert_resource(TumblerSpringPairings { array: Vec::new() });
}

//Spawn and Build Lock
pub fn spawn_lock(
    lock_sprite_handles: Res<LockSpriteHandles>,
    effects_sprite_handles: Res<EffectsSpriteHandles>,
    mut rng: ResMut<RandomSeed>,
    mut commands: Commands,
    mut tumbler_spring_pairings: ResMut<TumblerSpringPairings>,

) {
    //Sanity code
    println!("Building Locks");
    let mut offset: f32 = 0.0;
    //
    let mut tumbler_set_timer = Timer::from_seconds(TUMBLER_DEFAULT_SET_TIME, TimerMode::Once);
    //Pause timer after creation
    tumbler_set_timer.pause();

    //Sprites are spawned centered on their spawn coords, so the offset calculates where to place them
    offset += (LOCK_START_SPRITE_WIDTH / 2.0) + 10.0; //Extra pixel gap

    let lock = LockComponent {
        num_of_tumblers: 8, //Max amount for our purposes
        ..default()
    };

    commands
        .spawn((
            lock,
            Transform::from_xyz(0.0, 0.0, 0.0),
            Visibility::default(),
        ))
        .with_children(|parent_node| {
            //Start of Lock
            parent_node.spawn((
                Sprite::from_image(lock_sprite_handles.start_sprite.clone()),
                GameObjectAnchorMarker,
                Transform::from_xyz(offset, LOCK_START_OFFSET, 0.0),
            ));
            offset += (LOCK_START_SPRITE_WIDTH / 2.0) + (TUMBLER_CHAMBER_SPRITE_WIDTH / 2.0) + 10.0; //Extra pixel gap

            for x in 1..=lock.num_of_tumblers {
                //Spawn Tumbler Chamber
                parent_node.spawn((
                    Sprite::from_image(lock_sprite_handles.tumbler_chamber_sprite.clone()),
                    TumblerChamberComponent,
                    Transform::from_xyz(offset, 0.0, 0.0),
                ));

                //Spawn Tumbler
                let mut tumbler_entity_commands: EntityCommands;
                let tumbler_entity_id: Entity;
                //Generate random tumbler attributes
                let (mut tumbler, sprites) = gen_random_tumbler(
                    x,
                    tumbler_set_timer.clone(),
                    &mut rng.random_number_generator,
                    &lock_sprite_handles,
                );
                //get translation
                let tumbler_translation = vec3(
                    offset,
                    TOP_OF_CHAMBER
                        - (HEIGHT_OF_MEDIUM_TUMBLER_SPRITE / 2.0)
                        - (HEIGHT_OF_SPRING_SPRITE),
                    0.0,
                );


                if x == 1 {
                    tumbler_entity_commands = parent_node
                        .spawn((
                            // gen_random_tumbler(tumbler_translation,x, tumbler_set_timer.clone(), &mut rng.RandomNumberGenerator, &lock_sprite_handles,),
                            //Spawn sprite as a child with its own transform
                            children![(spawn_animatable_sprite_child(
                                sprites.0.clone(), //image
                                sprites.1, //color
                            ),)],
                            tumbler.clone(),
                            FocusedTumblerComponent,
                            Transform {
                                translation: tumbler_translation,
                                ..default()
                            },
                        ));
                    tumbler_entity_id = tumbler_entity_commands.id();
                } else {
                    tumbler_entity_commands = parent_node
                        .spawn((
                            //Spawn sprite as a child with its own transform
                            children![(spawn_animatable_sprite_child(
                                sprites.0.clone(),
                                sprites.1,
                            ),)],
                            tumbler.clone(),
                            Transform {
                                translation: tumbler_translation,
                                ..default()
                            },
                        ));

                    tumbler_entity_id = tumbler_entity_commands.id();
                }

                //Randomly generate rust on regular components
                if tumbler.tumbler_type == TumblerType::Normal {
                    let height = tumbler_size_helper_function(&tumbler);

                    chance_to_add_rust(
                        &mut rng.random_number_generator,
                        &mut tumbler_entity_commands,
                        &effects_sprite_handles,
                        height



                    )
                }
                
                if tumbler.tumbler_type == TumblerType::Magic{
                    let mut magic_code: Vec<Directions> = Vec::new();
                    for x in 0..=3 {
                        let rand_dir = match rng.random_number_generator.random_range(0..4){
                            0 => {
                                Directions::Left
                            }
                            1 => {
                                Directions::Up
                            }
                            2 => {
                                Directions::Right
                            }
                            _ => {
                                Directions::Down
                            }
                        };
                        magic_code.push(rand_dir);

                    }
                    println!("{:?}",magic_code);
                    tumbler_entity_commands.insert(TumblerMagicComponent {
                        arrow_code: magic_code
                    });

                }

                //Spawn_Spring
                let spring = parent_node
                    .spawn((
                        gen_random_spring(x, &mut rng.random_number_generator, &lock_sprite_handles),
                        Transform::from_xyz(
                            offset,
                            TOP_OF_CHAMBER - (HEIGHT_OF_SPRING_SPRITE / 2.0),
                            0.0,
                        ),
                    ))
                    .id();

                tumbler_spring_pairings
                    .array
                    .push((tumbler_entity_id, spring));
                if x != lock.num_of_tumblers {
                    offset += TUMBLER_CHAMBER_SPRITE_WIDTH + 10.0; //Extra pixel gap
                }
            }
            offset += (TUMBLER_CHAMBER_SPRITE_WIDTH / 2.0) + (LOCK_END_SPRITE_WIDTH / 2.0) + 10.0; //Extra pixel gap

            //Spawn End of Lock
            parent_node.spawn((
                Sprite::from_image(lock_sprite_handles.end_sprite.clone()),
                Transform {
                    //scale: Vec3::new(0.3, 0.3, 1.0),
                    translation: Vec3::new(offset, -LOCK_END_OFFSET, 0.0),
                    ..Default::default()
                },
            ));
            offset += LOCK_END_SPRITE_WIDTH / 2.0 + 10.0; //Extra pixel gap
        })
        //Add the offset back into the entity by replacing the Transform of the parent
        .insert(Transform::from_xyz(-offset / 2.0, 0.0, 0.0));
}


pub fn handle_catching_tumblers(
    check_set: Query<(), With<SetTumblerComponent>>, //Call all set elements
    mut commands: Commands,
    mut actions: MessageReader<CatchTumbler>,
    mut writer: MessageWriter<TumblerTimerMessage>,
    mut tumbler_query: Query<(Entity, &mut Transform, &mut TumblerComponent, &Children), With<FocusedTumblerComponent> >,
    mut animated_sprite_query: Query<&mut Sprite, With<Animated>>,
) {

    let Ok((focused_entity, focused_tumbler_transform, mut focused_tumbler, focused_tumbler_children)) =
        tumbler_query.single_mut()
    else {
        return;
    };

    //You will see the following match function in a few different functions, I could have put the const in the Component.... but I didnt want to go undo everything :)
    //Edit its now a helper function
    let height = tumbler_size_helper_function(&focused_tumbler);
    for action in actions.read() {
        match action {
            CatchTumbler::Catch => {
                if focused_tumbler_transform.translation.y + (height / 2.0) >= (TOP_OF_CHAMBER - TUMBLER_SET_THRESHOLD){
                    writer.write(TumblerTimerMessage(focused_entity));
                } else {
                    //Add time/score reducing code here!
                    for child in focused_tumbler_children.iter(){
                        if let Ok(_) = animated_sprite_query.get_mut(child) {
                            //commands.entity(child).remove::<Sprite>(); //test - works
                            commands.entity(child).insert(AnimationShake::new(0.5, Vec3::splat(0.0), TimerMode::Once));
                        }
                    }
                    if (focused_tumbler.velocity.y != (TOP_OF_CHAMBER - (height / 2.0) - (HEIGHT_OF_SPRING_SPRITE / 2.0))) && !check_set.contains(focused_entity)
                    {
                        focused_tumbler.velocity.y = -600.0;
                    }
                }
            }
        }
    }
}

pub fn spawn_animatable_sprite_child(
    tumbler_sprite: Handle<Image>,
    tumbler_color: Color,
) -> (Sprite, Animated, Transform) {
    (
        Sprite {
            image: tumbler_sprite,
            color: tumbler_color,
            ..default()
        },
        Animated,
        //AnimationShake::new(1.0, tumbler_translation),
        Transform::from_xyz(0.0,0.0,0.0)
        // {
        //     translation: tumbler_translation,
        //     ..default()
        // },
    )
}

pub fn tumbler_size_helper_function(tumbler: &TumblerComponent) -> f32{
    let height = match tumbler.size {
        TumblerSize::Small => HEIGHT_OF_SMALL_TUMBLER_SPRITE,
        TumblerSize::Medium => HEIGHT_OF_MEDIUM_TUMBLER_SPRITE,
        TumblerSize::Large => HEIGHT_OF_LARGE_TUMBLER_SPRITE,
    };
    height
}