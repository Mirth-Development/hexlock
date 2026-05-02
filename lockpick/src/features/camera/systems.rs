use bevy::prelude::*;

pub fn spawn_camera(
    mut commands: Commands,
    camera: Query<Entity, With<Camera>>,
) {
    //Code to check that camera already exists in the environment
    if let Ok(camera) = camera.single() {
        println!("Camera already exists");
        return
    } else {
        commands.spawn(
            (
                Camera2d::default(),
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            )
        );
    }
}