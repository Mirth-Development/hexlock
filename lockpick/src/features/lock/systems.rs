use bevy::prelude::*;
use bevy::ui::debug::print_ui_layout_tree;
use rand::prelude::*;

use super::components::LockComponent;
use super::resource::{LockSpriteHandles, NumberOfTumblersToSpawn};
use super::tumblers;


//Hardcoded Sprite Sizes so that they don't have to be sought dynamically, async loading is a pain in the ass
const START_SPRITE_SIZE : f32= 669.0;
const TUMBLER_SPRITE_SIZE : f32= 77.0;
const END_SPRITE_SIZE : f32= 149.0;




//Load Resources
pub fn load_sprite_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    //Sanity code
    println!("Loading LockSprites!");

    let start_handle: Handle<Image> = asset_server.load("images/Test_for_Start_of_Lock.png");
    let tumbler_handle: Handle<Image> = asset_server.load("images/Test_for_Tumbler_Section.png");
    let end_handle: Handle<Image> = asset_server.load("images/Test_for_End_of_Lock.png");
    commands.insert_resource(LockSpriteHandles {
        start_sprite: start_handle,
        tumbler_sprite: tumbler_handle,
        end_sprite: end_handle
    });

}

pub fn load_game_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    //Sanity code
    println!("Loading GameResources!");

    //List all resources required for load on startup here
    commands.insert_resource(NumberOfTumblersToSpawn(4));
}

// //Spawn Command
// pub fn spawn_lock(mut commands: Commands) {
//     //Sanity Code
//     println!("Spawning Lock!");
//     commands.spawn(
//         (LockComponent{
//             num_of_tumblers: 4,
//         },
//          Transform::from_translation(Vec3::new(10.0, 0.0, 0.0)),
//         )
//     );
// }

//Spawn and Build Lock
pub fn spawn_lock(
    mut commands: Commands,
    // lock_query: Query<(&LockComponent, &Transform), With<LockComponent>>,
    lock_sprite_handles: Res<LockSpriteHandles>,
) {
    //Sanity code
    println!("Building Locks");
    let mut offset: f32 = 0.0;

    //Sprites are spawned centered on their spawn coords, so the offset calculates where to place them
    offset += (START_SPRITE_SIZE/2.0);

    let mut lock = LockComponent::default();

    commands.spawn(
        (
            lock,
            Transform::from_xyz(0.0,0.0,0.0),
            Visibility::default()
        )
    ).with_children(|parent_node| {
        parent_node.spawn(
            (
                Sprite::from_image(lock_sprite_handles.start_sprite.clone()),
                Transform::from_xyz(offset, 0.0, 0.0),

            )
        );
        offset += START_SPRITE_SIZE/2.0 + TUMBLER_SPRITE_SIZE/2.0;

        for x in 1..=lock.num_of_tumblers {
            parent_node.spawn(
                (
                    Sprite::from_image(lock_sprite_handles.tumbler_sprite.clone()),
                    Transform::from_xyz(offset, 0.0, 0.0),

                )
            );
            if x != lock.num_of_tumblers {
            offset += TUMBLER_SPRITE_SIZE;
            }
        };
        offset += TUMBLER_SPRITE_SIZE/2.0 + END_SPRITE_SIZE/2.0;

        parent_node.spawn(
            (
                Sprite::from_image(lock_sprite_handles.end_sprite.clone()),
                Transform{
                    //scale: Vec3::new(0.3, 0.3, 1.0),
                    translation: Vec3::new(offset, 0.0, 0.0),
                    ..Default::default()
                },

            )
        );
        offset += END_SPRITE_SIZE/2.0;

    })
        //Add the offset back into the entity by replacing the Transform of the parent
        .insert(
        Transform::from_xyz( -offset/2.0, 0.0,0.0)
    );

}