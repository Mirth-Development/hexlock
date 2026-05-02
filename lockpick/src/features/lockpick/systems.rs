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
            Sprite{
                image: asset_server.load("images/lockpick.png"),
                //custom_size: Option::from(Vec2::new(250.0, 280.0)),
                ..Default::default()
            },
            Transform {
                translation: Vec3::new(0.0,0.0,0.0),
                scale: Vec3::new(0.3,0.3,1.0),
                ..Default::default()
            }
        )
    );
}

//Movement Systems

