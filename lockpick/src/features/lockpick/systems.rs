use bevy::prelude::*;

// fn lockpick_prefab() ->

//Spawn Systems
pub fn spawn_lockpick (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(
        (
            Node::DEFAULT,
            Sprite::from_image(asset_server.load("images/lockpick.png")),
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        )
    );
}

//Movement Systems