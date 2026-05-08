use bevy::prelude::*;
use crate::features::animation::components::{Animatable, Animated, AnimationShake};
use crate::features::game_controller::game_effects::events::{Magic, Zap};
use crate::features::lock::components::LockComponent;
use crate::features::lock::resource::{TumblerSpringPairings};
use crate::features::lock::spring::components::SpringComponent;
use crate::features::lock::spring::resources::SpringSize;
use crate::features::lock::spring::systems::HEIGHT_OF_SPRING_SPRITE;
use crate::features::lock::systems::{tumbler_size_helper_function, TOP_OF_CHAMBER};
use crate::features::lock::tumblers::components::{FocusedTumblerComponent, SetTumblerComponent, TumblerComponent, TumblerRustComponent};
use crate::features::lock::tumblers::event::BreakRust;
use crate::features::lock::tumblers::resources::{TumblerSize, TumblerType};
use crate::features::lock::tumblers::systems::HEIGHT_OF_MEDIUM_TUMBLER_SPRITE;
use crate::features::lockpick::component::LockpickComponent;
use crate::features::lockpick::messages::{ChargeLockpick, LockpickAction};
use crate::features::lockpick::resources::{LockpickElectricCharge, LockpickType};

pub const LOCKPICK_HEAD_OFFSET: f32 = 1041.0;
const LOCKPICK_MAX_HEIGHT: f32 = TOP_OF_CHAMBER - (HEIGHT_OF_MEDIUM_TUMBLER_SPRITE / 2.0 + HEIGHT_OF_SPRING_SPRITE);
const LOCKPICK_LOWER_BOUND: f32 = -200.0;


pub fn load_lockpick_resources(
    mut commands: Commands,)
{
        //Sanity code
        println!("Loading Lockpick Resources!");

        commands.insert_resource(LockpickElectricCharge {
            is_charging: false,
            max_charge: 2.0,
            charge_per_tick: 1.5, //ratio
            current_charge: 0.0,
        });
}

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
    mut lockpick_query: Query<(&mut Transform, &LockpickComponent)>,
    tumbler_query: Query<(&GlobalTransform, &TumblerComponent), With<FocusedTumblerComponent>>,
) {
    let Ok((mut lockpick_transform, lockpick)) = lockpick_query.single_mut() else {
        println!("FAIL");
        return
    };

    for (global_position, tumbler) in &tumbler_query {
        if tumbler.position == lockpick.current_tumbler {
            if (lockpick_transform.translation.x + LOCKPICK_HEAD_OFFSET ) != global_position.translation().x {
                //println!("Moving Pick!");
                let target_point = global_position.translation() + Vec3::new(-LOCKPICK_HEAD_OFFSET+ 10.0 , 0.0, 0.0);
                let distance = lockpick_transform.translation.distance(target_point);
                println!("distance {}", distance);
                lockpick_transform.translation.x = global_position.translation().x - (LOCKPICK_HEAD_OFFSET ) ;
            }
        }
    }
}


pub fn lockpick_movement( //NORMAL PICK ONLY?
    time: Res<Time>,
    mut lockpick_electric_charge: MessageReader<LockpickAction>,
    mut lockpick_query: Query<(&mut Transform, &mut LockpickComponent)>
) {

    let Ok((mut lockpick_transform, mut lockpick)) = lockpick_query.single_mut() else {return};

    match lockpick.lockpick_type{
        LockpickType::Normal => {
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
        LockpickType::Electric => {
            for action in lockpick_electric_charge.read(){
                match action {

                    LockpickAction::Charge => {lockpick.is_moving = true}
                    LockpickAction::Release => {lockpick.is_moving = false}
                    _ => {}
                }
            }
        }
        LockpickType::Magic => {

        }
    }

}

pub fn handle_lockpick_charge(

    time: Res<Time>,
    mut lockpick_query: Query<&mut LockpickComponent>,
    mut actions: MessageReader<ChargeLockpick>,
    mut lockpick_electric_charge: ResMut<LockpickElectricCharge>,
){
    let Ok(mut lockpick) = lockpick_query.single_mut() else {return};
    for action in actions.read() {
        match action {
            ChargeLockpick::Charge => {
                lockpick_electric_charge.is_charging = true;
                lockpick.charge_timer.unpause()
            }
            ChargeLockpick::Release => {
                lockpick_electric_charge.current_charge = 0.0;
                lockpick_electric_charge.is_charging = false;
                lockpick.charge_timer.reset();
                lockpick.charge_timer.pause()
            }
        }
    }
    if lockpick_electric_charge.is_charging {
        //println!("charge amount:{}", lockpick_electric_charge.current_charge);
        lockpick.charge_timer.tick(time.delta());
        if lockpick_electric_charge.current_charge > lockpick_electric_charge.max_charge{
            lockpick_electric_charge.current_charge = lockpick_electric_charge.max_charge
        } else {
            lockpick_electric_charge.current_charge += lockpick.charge_timer.elapsed_secs() * lockpick_electric_charge.charge_per_tick;
            lockpick.charge_timer.reset();
        }
    }

}


//Handle Pick Event
pub fn handle_lockpick_message(
    check_set: Query<(), With<SetTumblerComponent>>, //Call all set elements
    check_rust: Query<(Entity,&TumblerRustComponent)>,
    //tumbler_parent: Query<(&Children), With<TumblerComponent>>,
    lock_query: Query<&LockComponent>,
    springs: Query<&SpringComponent>,
    tumblers: Query<(Entity, &TumblerComponent), Without<FocusedTumblerComponent>>,
    tumbler_spring_pairings: Res<TumblerSpringPairings>,
    lockpick_electric_charge: ResMut<LockpickElectricCharge>,
    mut charge_lockpick_writer: MessageWriter<ChargeLockpick>,
    mut actions: MessageReader<LockpickAction>,
    mut commands: Commands,
    mut lockpick_query: Query<(&Children, &GlobalTransform, &mut LockpickComponent)>,
    mut animated_sprite_query: Query<&mut Sprite, With<Animated>>,
    mut focused_tumbler_query: Query<(Entity,&GlobalTransform, &mut TumblerComponent, &Children), With<FocusedTumblerComponent>>,

){

    //let Ok((tumbler_entity, mut tumbler)) = focused_tumbler_query.single_mut() else {return};
    let Ok((focused_tumbler_entity, focused_tumbler_transform, mut focused_tumbler, focused_children)) = focused_tumbler_query.single_mut() else {
        println!("Tumbler check!");
        return
    };
    let Ok((lockpick_children, lockpick_transform, mut lockpick)) = lockpick_query.single_mut() else {
        println!("Lockpick check!");
        return
    };

    let Ok(lock) = lock_query.single() else {
        println!("Lock check!");
        return
    };





    if !lockpick.is_moving {
        let mut spring = None;
        for (tumbler_index, spring_index) in tumbler_spring_pairings.array.iter(){
            if focused_tumbler_entity == *tumbler_index {
                let Ok(paired_spring) = springs.get(*spring_index) else {
                    panic!("Tumbler has no spring!")
                };
                spring = Some(paired_spring);
                break;
            }
        }
        let Some(spring)  = spring else {
            panic!("No pairing!")
        };


        //println!("Lockpick isnt moving!");
        for action in actions.read(){
            println!("Lockpick received Message");
            let tumbler_speed = match focused_tumbler.size {
                TumblerSize::Small => {80.0}
                TumblerSize::Medium => {0.0}
                TumblerSize::Large => {-80.0}
            };

            let spring_speed = match spring.size {
                SpringSize::Thin => {40.0}
                SpringSize::Regular => {0.0}
                SpringSize::Thick => {-40.0}
            };

            let variant_tumbler_spring_speed = spring_speed + tumbler_speed;
            match action {
                LockpickAction::Left => {
                    if focused_tumbler.position > 1 {
                        for (tumbler_entity, tumbler_component) in &tumblers {
                            if tumbler_component.position == lockpick.current_tumbler - 1 {
                                commands.entity(focused_tumbler_entity).remove::<FocusedTumblerComponent>();
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
                                commands.entity(focused_tumbler_entity).remove::<FocusedTumblerComponent>();
                                commands.entity(tumbler_entity).insert(FocusedTumblerComponent);
                                lockpick.current_tumbler += 1;
                                break;
                            }
                        }
                    }
                }

                LockpickAction::Charge => {
                    lockpick.is_moving = true;
                    charge_lockpick_writer.write(ChargeLockpick::Charge);
                    println!("Charging Pick!")


                }
                LockpickAction::Release => {
                    lockpick.is_moving = false;
                    if !check_set.contains(focused_tumbler_entity) {
                        if focused_tumbler.tumbler_type == TumblerType::Electric {
                            if lockpick_electric_charge.current_charge > lockpick_electric_charge.max_charge /4.0 {
                                commands.trigger(Zap{life_timer: Timer::from_seconds(0.4, TimerMode::Once),
                                    top: focused_tumbler_transform.translation().y,
                                    bottom: lockpick_transform.translation().y});
                                println!("Zapping! power:{}", lockpick_electric_charge.current_charge);
                                focused_tumbler.velocity.y = (200.0 + variant_tumbler_spring_speed)*lockpick_electric_charge.current_charge;
                            } else {
                                shake_tumbler_help_function(focused_children, &mut animated_sprite_query, &mut commands);
                                println!("Not enough force")
                            }

                        } else {
                            shake_tumbler_help_function(focused_children, &mut animated_sprite_query, &mut commands);
                            println!("lose time!")
                            //NOT RIGHT TUMBLER TYPE, LOSE TIME!

                        }

                    } else {
                        //This may be improper handling of this component but who cares
                        if lockpick_electric_charge.current_charge > lockpick_electric_charge.max_charge /4.0 {
                            commands.trigger(Zap{life_timer: Timer::from_seconds(0.4, TimerMode::Once),
                                top: focused_tumbler_transform.translation().y,
                                bottom: lockpick_transform.translation().y});
                            focused_tumbler.timer.reset();
                            println!("STUNNING TUMBLER!")
                        } else {
                            shake_tumbler_help_function(focused_children, &mut animated_sprite_query, &mut commands);
                            println!("Not enough force")
                        }



                    }
                    charge_lockpick_writer.write(ChargeLockpick::Release);


                }
                //May be for regular pick only?
                LockpickAction::Pick => {

                    match lockpick.lockpick_type {
                        LockpickType::Normal => {
                            if !check_set.contains(focused_tumbler_entity) {
                                let mut found_rust = None;
                                //check children for rust
                                for child in focused_children.iter() {
                                    if let Ok((rust_entity,_)) = check_rust.get(child) {
                                        found_rust = Some(rust_entity);

                                    }
                                }


                                if found_rust.is_some(){
                                    shake_tumbler_help_function(focused_children, &mut animated_sprite_query, &mut commands);
                                    commands.trigger(BreakRust{id:found_rust.unwrap()});


                                } else {
                                    if focused_tumbler.tumbler_type == TumblerType::Normal {
                                        lockpick.is_moving = true;
                                        lockpick.velocity.y += 800.0;
                                        println!("Picking!");
                                        focused_tumbler.velocity.y = 400.0 + variant_tumbler_spring_speed;
                                    } else {
                                        shake_tumbler_help_function(focused_children, &mut animated_sprite_query, &mut commands);

                                        println!("Lose time")
                                        //NOT RIGHT TUMBLER TYPE, LOSE TIME!
                                    }
                                }
                            } else {
                                shake_tumbler_help_function(focused_children, &mut animated_sprite_query, &mut commands);
                            }
                        }
                        _ => {println!("This type shouldn't pick!")}
                    }
                    lockpick.is_moving = true;
                    lockpick.velocity.y += 800.0;
                }

                LockpickAction::Hex => {
                    match lockpick.lockpick_type {
                        LockpickType::Magic => {
                            if !check_set.contains(focused_tumbler_entity) {
                                if focused_tumbler.tumbler_type == TumblerType::Magic {
                                    //lockpick.is_moving = true;
                                    // lockpick.velocity.y += 800.0;
                                    println!("Magicking!");
                                    //focused_tumbler.velocity.y = 400.0 + variant_tumbler_spring_speed;
                                    commands.trigger(Magic{
                                        life_timer: Timer::from_seconds(2.0, TimerMode::Once),
                                        top: focused_tumbler_transform.translation().y - (tumbler_size_helper_function(&focused_tumbler)/2.0),
                                        bottom: lockpick_transform.translation().y});
                                } else {
                                    shake_tumbler_help_function(focused_children, &mut animated_sprite_query, &mut commands);
                                    // for child in focused_children.iter() {
                                    //     if let Ok(_) = animated_sprite_query.get_mut(child) {
                                    //         println!("Animates");
                                    //         commands.entity(child).insert(AnimationShake::new(0.5, Vec3::splat(0.0)));
                                    //     }
                                    // }
                                    println!("Lose time")
                                    //NOT RIGHT TUMBLER TYPE, LOSE TIME!
                                    }
                                }
                        }
                        _ => {println!("This type shouldn't pick!")}
                    }
                    //lockpick.is_moving = true;
                    // lockpick.velocity.y += 800.0;
                }
                ,


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

                }
            }
        }
    }

}



//Helper Function

pub fn shake_tumbler_help_function(
    tumbler_children: &Children,
    sprite_query: &mut Query<&mut Sprite, With<Animated>>,
    commands: &mut Commands
)
{
    for child in tumbler_children.iter() {
        if let Ok(_) = sprite_query.get_mut(child) {
            println!("Animates");
            commands.entity(child).insert(AnimationShake::new(0.5, Vec3::splat(0.0), TimerMode::Once));
        }
    }
}
