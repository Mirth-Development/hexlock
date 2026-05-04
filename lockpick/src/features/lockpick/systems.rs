use bevy::prelude::*;
use crate::features::lock::components::LockComponent;
use crate::features::lock::tumblers::components::{FocusedTumblerComponent, TumblerComponent};
use crate::features::lockpick::component::LockpickComponent;
use crate::features::lockpick::events::LockpickAction;

const LOCKPICK_HEAD_OFFSET: f32 = 280.0;


//Spawn Systems
pub fn spawn_lockpick (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(
        (
            Sprite{
                image: asset_server.load("images/lockpick.png"),
                //custom_size: Option::from(Vec2::new(250.0, 280.0)),
                ..Default::default()
            },
            LockpickComponent{
                is_moving: false,
                current_tumbler: 1,
            },
            Transform {
                translation: Vec3::new(0.0,0.0,0.0),
                //scale: Vec3::new(0.3,0.3,1.0),
                ..Default::default()
            }
        )
    );
}

//Movement Systems

///Automatically Moves pick to focused chamber
pub fn move_to_focused_tumbler(
    time: Res<Time>,
    mut lockpick_query: Query<(&mut Transform, &LockpickComponent)>,
    tumbler_query: Query<(&GlobalTransform, &TumblerComponent)>,
    // tumbler_position_collection: Res<TumblerPositionCollection>,
) {
    let Ok((mut lockpick_pos, lockpick)) = lockpick_query.single_mut() else {
        println!("FAIL");
        return };

    for (global_position, tumbler) in &tumbler_query {
        if tumbler.position == lockpick.current_tumbler {
            // if tumbler.position == pair.0
            //     && (lockpick_pos.translation.x + LOCKPICK_HEAD_OFFSET) != pair.1
            // {
            //     println!("Moving Pick!");
            //     lockpick_pos.translation.x = pair.1;
            // }
            if (lockpick_pos.translation.x + LOCKPICK_HEAD_OFFSET) != global_position.translation().x{
                println!("Moving Pick!");
                    lockpick_pos.translation.x = global_position.translation().x - LOCKPICK_HEAD_OFFSET
            }
        }
    }

}


//Handle Pick Event
pub fn handle_lockpick_message(
    mut actions: MessageReader<LockpickAction>,
    mut commands: Commands,
    tumblers: Query<(Entity, &TumblerComponent), Without<FocusedTumblerComponent>>,
    focused_tumbler_component: Query<(Entity,&TumblerComponent), With<FocusedTumblerComponent>>,
    mut lockpick_query: Query<&mut LockpickComponent>,
    lock_query: Query<&LockComponent>,
) {
    let Ok((focused_entity ,focused_tumbler)) = focused_tumbler_component.single() else {
        println!("bail on focused");
        return};
    let Ok(mut lockpick) = lockpick_query.single_mut() else {
        println!("bail on lockpick");
        return};
    let Ok(lock) = lock_query.single() else {
        println!("bail on lock");
        return};

    for action in actions.read(){
        match action {
            LockpickAction::Left => {
                if focused_tumbler.position > 1 {
                    for (tumbler_entity, tumbler_component) in &tumblers{
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
                    for (tumbler_entity, tumbler_component) in &tumblers{
                        if tumbler_component.position == lockpick.current_tumbler + 1 {
                            commands.entity(focused_entity).remove::<FocusedTumblerComponent>();
                            commands.entity(tumbler_entity).insert(FocusedTumblerComponent);
                            lockpick.current_tumbler += 1;
                            break;
                        }
                    }
                }
            }
            _ => {
            }
        }
    }

}
