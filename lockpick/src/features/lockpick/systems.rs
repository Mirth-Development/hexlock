use bevy::prelude::*;
use crate::features::lock::components::LockComponent;
use crate::features::lock::spring::systems::HEIGHT_OF_SPRING_SPRITE;
use crate::features::lock::systems::TOP_OF_CHAMBER;
use crate::features::lock::tumblers::components::{FocusedTumblerComponent, SetTumblerComponent, TumblerComponent};
use crate::features::lock::tumblers::resources::TumblerSize;
use crate::features::lock::tumblers::systems::HEIGHT_OF_MEDIUM_TUMBLER_SPRITE;
use crate::features::lockpick::component::LockpickComponent;
use crate::features::lockpick::messages::LockpickAction;
use crate::features::lockpick::resources::LockpickType;

const LOCKPICK_HEAD_OFFSET: f32 = 1041.0;
const LOCKPICK_MAX_HEIGHT: f32 = TOP_OF_CHAMBER - (HEIGHT_OF_MEDIUM_TUMBLER_SPRITE / 2.0 + HEIGHT_OF_SPRING_SPRITE);
const LOCKPICK_LOWER_BOUND: f32 = -200.0;

//Spawn Systems
pub fn spawn_lockpick (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Sprite{
            image: asset_server.load("images/Lockpick.png"),
            ..Default::default()
        },
        LockpickComponent::default(),
        Transform {
            translation: Vec3::new(0.0, LOCKPICK_LOWER_BOUND, 0.0),
            ..Default::default()
        }
    ));
}

//Movement Systems

///Automatically Moves pick to focused chamber
pub fn move_to_focused_tumbler(
    mut lockpick_query: Query<(&mut Transform, &LockpickComponent)>,
    tumbler_query: Query<(&GlobalTransform, &TumblerComponent)>,
) {
    let Ok((mut lockpick_transform, lockpick)) = lockpick_query.single_mut() else {
        println!("FAIL");
        return
    };

    for (global_position, tumbler) in &tumbler_query {
        if tumbler.position == lockpick.current_tumbler {
            if (lockpick_transform.translation.x + LOCKPICK_HEAD_OFFSET) != global_position.translation().x {
                println!("Moving Pick!");
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
    mut lockpick_query: Query<(&mut Sprite, &mut LockpickComponent)>,
    mut focused_tumbler_query: Query<(Entity, &mut TumblerComponent), With<FocusedTumblerComponent>>,

){
    //let Ok((tumbler_entity, mut tumbler)) = focused_tumbler_query.single_mut() else {return};
    let Ok((focused_entity, mut focused_tumbler)) = focused_tumbler_query.single_mut() else {
        return
    };
    let Ok((mut lockpick_sprite, mut lockpick)) = lockpick_query.single_mut() else {
        return
    };
    let Ok(lock) = lock_query.single() else {
        return
    };

    if !lockpick.is_moving {
        for action in actions.read(){
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
                    if !check_set.contains(focused_entity) {
                        println!("Picking!");
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

                        focused_tumbler.velocity.y = 400.0 + spring_speed + tumbler_speed;
                        lockpick.is_moving = true;
                        lockpick.velocity.y += 800.0;
                    }
                },
                // let tumbler_color: Color = match random_type {
                //     TumblerType::Normal=> {
                //     Color::default()
                //     },
                //     TumblerType::Magic => {
                //     Color::srgb(1.0, 0.0, 1.0)
                //     },
                //     TumblerType::Electric => {
                //     Color::srgb(1.0, 1.0, 0.0)
                //     }
                LockpickAction::SwitchNext => {
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
                },
                LockpickAction::SwitchLast => {
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
                },
            }
        }
    }

}

pub fn lockpick_movement(
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
