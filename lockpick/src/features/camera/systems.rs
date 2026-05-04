use bevy::prelude::*;
use bevy::camera::*;
use bevy::camera::ScalingMode::WindowSize;

pub fn spawn_camera(
    mut commands: Commands,
    camera: Query<Entity, With<Camera>>,
) {
    //Code to check that camera already exists in the environment
    if let Ok(_) = camera.single() {
        println!("Camera already exists");
        return
    } else {
        commands.spawn(
            (
                Camera2d::default(),
                Camera{
                    clear_color: ClearColorConfig::Custom(Color::srgb(0.0, 0.0, 0.0)), //Change the background color per camera
                    ..default()
                },
                Projection::from(OrthographicProjection{
                    scaling_mode: ScalingMode::FixedVertical {
                        viewport_height:1080.0,
                    },
                    scale: 1.3,
                    ..OrthographicProjection::default_2d()
                }),
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            )
        );
    }
}