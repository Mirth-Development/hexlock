use bevy::prelude::*;
use rand::RngExt;
use crate::features::lock::components::LockComponent;
use crate::features::lock::spring::systems::HEIGHT_OF_SPRING_SPRITE;
use crate::features::lock::systems::TOP_OF_CHAMBER;
use crate::features::lock::tumblers::components::{FocusedTumblerComponent, SetTumblerComponent, TumblerComponent};
use crate::features::lock::tumblers::systems::HEIGHT_OF_TUMBLER_SPRITE;
use crate::features::lockpick::component::LockpickComponent;
use crate::features::lockpick::messages::LockpickAction;
use crate::features::rand::resources::RandomSeed;

const LOCKPICK_HEAD_OFFSET: f32 = 1041.0;
const LOCKPICK_MAX_HEIGHT: f32 = (TOP_OF_CHAMBER - (HEIGHT_OF_TUMBLER_SPRITE / 2.0 + HEIGHT_OF_SPRING_SPRITE));
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
    time: Res<Time>,
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
    mut lockpick_query: Query<&mut LockpickComponent>,
    mut random: ResMut<RandomSeed>,
    mut focused_tumbler_query: Query<(Entity, &mut TumblerComponent), With<FocusedTumblerComponent>>,

){
    //let Ok((tumbler_entity, mut tumbler)) = focused_tumbler_query.single_mut() else {return};
    let Ok((focused_entity, mut focused_tumbler)) = focused_tumbler_query.single_mut() else {
        return
    };
    let Ok(mut lockpick) = lockpick_query.single_mut() else {
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
                        focused_tumbler.velocity.y = 250.0;
                        lockpick.is_moving = true;
                        lockpick.velocity.y += 800.0;
                    }
                }
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
        lockpick.velocity.y = 0.0;
        lockpick_transform.translation.y = LOCKPICK_LOWER_BOUND;
        lockpick.is_moving = false;
    }

    lockpick_transform.translation += lockpick.velocity * time.delta_secs();
}
