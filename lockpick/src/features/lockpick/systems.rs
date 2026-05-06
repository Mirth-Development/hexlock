use bevy::math::NormedVectorSpace;
use bevy::prelude::*;
use bevy::ui::debug::print_ui_layout_tree;
use rand::RngExt;
use crate::features::animation::components::{Animated, AnimationShake};
use crate::features::lock::components::LockComponent;
use crate::features::lock::spring::components::SpringComponent;
use crate::features::lock::spring::systems::HEIGHT_OF_SPRING_SPRITE;
use crate::features::lock::systems::TOP_OF_CHAMBER;
        use crate::features::lock::tumblers::components::{FocusedTumblerComponent, SetTumblerComponent, TumblerComponent};
use crate::features::lock::tumblers::resources::{TumblerSize, TumblerType};
use crate::features::lock::tumblers::systems::HEIGHT_OF_MEDIUM_TUMBLER_SPRITE;
use crate::features::lockpick::component::LockpickComponent;
use crate::features::lockpick::messages::LockpickAction;
use crate::features::lockpick::resources::LockpickType;
use crate::features::rand::resources::RandomSeed;

const LOCKPICK_HEAD_OFFSET: f32 = 1041.0;
const LOCKPICK_MAX_HEIGHT: f32 = (TOP_OF_CHAMBER - (HEIGHT_OF_MEDIUM_TUMBLER_SPRITE / 2.0 + HEIGHT_OF_SPRING_SPRITE));
const LOCKPICK_LOWER_BOUND: f32 = -200.0;

//Spawn Systems
pub fn spawn_lockpick (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        LockpickComponent::default(),
        Transform {
            translation: Vec3::new(0.0, LOCKPICK_LOWER_BOUND, 0.0),
            ..Default::default()
        }
    )).with_children(| parent_node| {
        parent_node.spawn(
            (
                Sprite{
                    image: asset_server.load("images/Lockpick.png"),
                    ..Default::default()
                },
                Transform::from_xyz(0.0,0.0,0.0),
                Animated,
                // AnimationShake::new(1.0, Vec3::splat(0.0,))
                )
        );

        
    });
}

//Movement Systems

///Automatically Moves pick to focused chamber
pub fn move_to_focused_tumbler(
    time: Res<Time>,
    mut lockpick_query: Query<(&mut Transform, &LockpickComponent)>,
    tumbler_query: Query<(&GlobalTransform, &TumblerComponent), With<FocusedTumblerComponent>>,
) {
    let Ok((mut lockpick_transform, lockpick)) = lockpick_query.single_mut() else {
        println!("FAIL");
        return
    };

    for (global_position, tumbler) in &tumbler_query {
        if tumbler.position == lockpick.current_tumbler {
            if (lockpick_transform.translation.x + LOCKPICK_HEAD_OFFSET) != global_position.translation().x {
                //println!("Moving Pick!");
                let target_point = global_position.translation() + Vec3::new(-LOCKPICK_HEAD_OFFSET, 0.0, 0.0);
                let distance = lockpick_transform.translation.distance(target_point);
                println!("distance {}", distance);
                lockpick_transform.translation.x = global_position.translation().x - LOCKPICK_HEAD_OFFSET
            }
        }
    }
}


//Handle Pick Event
pub fn handle_lockpick_message(
    check_set: Query<(), With<SetTumblerComponent>>, //Call all set elements
    lock_query: Query<&LockComponent>,
    tumblers: Query<(Entity, &TumblerComponent), Without<FocusedTumblerComponent>>,
    mut actions: MessageReader<LockpickAction>,
    mut commands: Commands,
    mut lockpick_query: Query<(&mut Children, &mut LockpickComponent)>,
    mut animated_sprite_query: Query<&mut Sprite, With<Animated>>,
    mut focused_tumbler_query: Query<(Entity, &mut TumblerComponent), With<FocusedTumblerComponent>>,

){
    //let Ok((tumbler_entity, mut tumbler)) = focused_tumbler_query.single_mut() else {return};
    let Ok((focused_entity, mut focused_tumbler)) = focused_tumbler_query.single_mut() else {
        println!("Tumbler check!");
        return
    };
    let Ok((mut lockpick_children, mut lockpick)) = lockpick_query.single_mut() else {
        println!("Lockpick check!");
        return
    };

    let Ok(lock) = lock_query.single() else {
        println!("Lock check!");
        return
    };

    if !lockpick.is_moving {
        let tumbler_speed = match focused_tumbler.size {
            TumblerSize::Small => {80.0}
            TumblerSize::Medium => {0.0}
            TumblerSize::Large => {-80.0}
        };

        let spring_speed = match focused_tumbler.size {
            TumblerSize::Small => {40.0}
            TumblerSize::Medium => {0.0}
            TumblerSize::Large => {-40.0}
        };
        //println!("Lockpick isnt moving!");
        for action in actions.read(){
            println!("Lockpick received Message");
            match action {
                LockpickAction::Left => {
                    if focused_tumbler.position > 1 {
                        for (tumbler_entity, tumbler_component) in &tumblers {
                            if tumbler_component.position == lockpick.current_tumbler - 1 {
                                commands.entity(focused_entity).remove::<FocusedTumblerComponent>();
                                commands.entity(tumbler_entity).insert(FocusedTumblerComponent);
                                lockpick.current_tumbler -= 1;
                                break;
                            }
                        }
                    }
                }
                LockpickAction::Right => {
                    if focused_tumbler.position < lock.num_of_tumblers {
                        for (tumbler_entity, tumbler_component) in &tumblers {
                            if tumbler_component.position == lockpick.current_tumbler + 1 {
                                commands.entity(focused_entity).remove::<FocusedTumblerComponent>();
                                commands.entity(tumbler_entity).insert(FocusedTumblerComponent);
                                lockpick.current_tumbler += 1;
                                break;
                            }
                        }
                    }
                }
                LockpickAction::Pick => {
                    match lockpick.lockpick_type {
                        LockpickType::Normal => {
                            if !check_set.contains(focused_entity) {
                                if focused_tumbler.tumbler_type == TumblerType::Normal {
                                    lockpick.is_moving = true;
                                    lockpick.velocity.y += 800.0;
                                    println!("Picking!");
                                    focused_tumbler.velocity.y = 400.0 + spring_speed + tumbler_speed;
                                } else {
                                    //NOT RIGHT TUMBLER TYPE, LOSE TIME!
                                }


                            }
                        }
                        LockpickType::Electric => {
                            if !check_set.contains(focused_entity){
                                if focused_tumbler.tumbler_type == TumblerType::Electric {
                                    lockpick.is_moving = true;
                                    lockpick.velocity.y += 800.0; //REMOVE LATER, MOVEMENT WILL HANDLE DIFFERENT FOR THIS PICK
                                    println!("Picking!");
                                    focused_tumbler.velocity.y = 400.0 + spring_speed + tumbler_speed;
                                } else {
                                    println!("WRONG TYPE!")
                                    //NOT RIGHT TUMBLER TYPE, LOSE TIME!
                                }
                            } else {
                                println!("STUN TUMBLER")
                                //STUN THE SET TUMBLER!
                            }
                        }
                        LockpickType::Magic => {
                            if !check_set.contains(focused_entity){
                                if focused_tumbler.tumbler_type == TumblerType::Magic {
                                    lockpick.is_moving = true;
                                    lockpick.velocity.y += 800.0; //REMOVE LATER, MOVEMENT WILL HANDLE DIFFERENT FOR THIS PICK
                                    println!("Picking!");
                                    focused_tumbler.velocity.y = 400.0 + spring_speed + tumbler_speed;
                                } else {
                                    println!("WRONG TYPE!")
                                    //NOT RIGHT TUMBLER TYPE, LOSE TIME! OR SLOW TUMBLER?
                                }
                            }
                        }
                    }

                },
                LockpickAction::SwitchNext => {
                    for child in lockpick_children.iter(){
                        if let Ok(mut lockpick_sprite) = animated_sprite_query.get_mut(child) {
                            lockpick.lockpick_type = match lockpick.lockpick_type {
                                LockpickType::Magic => {
                                    lockpick_sprite.color = Color::default();
                                    LockpickType::Normal

                                },
                                LockpickType::Normal => {
                                    lockpick_sprite.color = Color::srgb(1.0, 1.0, 0.0);
                                    LockpickType::Electric
                                },
                                LockpickType::Electric => {
                                    lockpick_sprite.color = Color::srgb(1.0, 0.0, 1.0);
                                    //     },
                                    LockpickType::Magic
                                }
                            };
                            break
                        } else {
                            panic!("Sprite Error!")
                        }
                    }

                },
                LockpickAction::SwitchLast => {

                    for child in lockpick_children.iter(){
                        if let Ok(mut lockpick_sprite) = animated_sprite_query.get_mut(child) {
                            lockpick.lockpick_type = match lockpick.lockpick_type {
                                LockpickType::Magic => {
                                    lockpick_sprite.color = Color::srgb(1.0, 1.0, 0.0);
                                    LockpickType::Electric
                                },
                                LockpickType::Normal => {
                                    lockpick_sprite.color = Color::srgb(1.0, 0.0, 1.0);
                                    LockpickType::Magic
                                },
                                LockpickType::Electric => {
                                    lockpick_sprite.color = Color::default();
                                    LockpickType::Normal
                                }
                            };
                            break
                        } else {
                            panic!("Sprite Error!")
                        }
                    }

                    // LockpickType::Magic => {
                        //     lockpick_sprite.color = Color::srgb(1.0, 1.0, 0.0);
                        //     LockpickType::Electric
                        // },
                        // LockpickType::Normal => {
                        //     lockpick_sprite.color = Color::srgb(1.0, 0.0, 1.0);
                        //     LockpickType::Magic
                        // },
                        // LockpickType::Electric => {
                        //     lockpick_sprite.color = Color::default();
                        //     LockpickType::Normal
                        // }

                },
            }
        }
    }

}

pub fn lockpick_movement( //NORMAL PICK ONLY?
    time: Res<Time>,
    mut lockpick_query: Query<(&mut Transform, &mut LockpickComponent)>
) {

    let Ok((mut lockpick_transform, mut lockpick)) = lockpick_query.single_mut() else {return};

    if lockpick_transform.translation.y > LOCKPICK_MAX_HEIGHT {
        lockpick_transform.translation.y = LOCKPICK_MAX_HEIGHT;
        lockpick.velocity.y *= -1.0;
    }
    else if lockpick_transform.translation.y < LOCKPICK_LOWER_BOUND {
        lockpick_transform.translation.y = LOCKPICK_LOWER_BOUND;
        lockpick.velocity.y = 0.0;
        lockpick.is_moving = false;
    }

    lockpick_transform.translation += lockpick.velocity * time.delta_secs();
}
