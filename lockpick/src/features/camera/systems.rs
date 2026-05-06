use bevy::prelude::*;
use bevy::camera::*;
use crate::features::game_controller::game_timer::definitions::*;

pub fn spawn_camera(
    mut commands: Commands,
    camera: Query<Entity, With<Camera>>,
) {
    //Code to check that camera already exists in the environment
    if let Ok(_) = camera.single() {
        println!("Camera already exists");
        return
    } else {
        commands.spawn((
            Camera2d::default(),
            Camera {
                clear_color: ClearColorConfig::Custom(Color::srgb(0.0, 0.0, 0.0)), //Change the background color per camera
                ..default()
            },
            Projection::from(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical {
                    viewport_height:1080.0, // Locks viewport height to x amount of pixels.
                },
                scale: 1.3,
                ..OrthographicProjection::default_2d()
            }),
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ));
    }

    // ####################################################################################################### //
    // CLOCK TESTS

    // Default Clock
    commands.spawn(Chronolog::new());

    // Tracking Specific Places Example
    // commands.spawn(Chronolog {
    //     ticker_for_one: Some(Ticker {
    //         number: Some(0),
    //         timer: Some(Timer::from_seconds(1.0, TimerMode::Repeating)),
    //     }),
    //     ticker_for_tenth: Some(Ticker {
    //         number: Some(0),
    //         timer: Some(Timer::from_seconds(0.1, TimerMode::Repeating)),
    //     }),
    //     ..Default::default()
    // });

    // Standalone Ticker Example
    // commands.spawn(Ticker {
    //     number: Some(0),
    //     timer: Some(Timer::from_seconds(1.0, TimerMode::Repeating)),
    // });

    // Preset Clock Example
    // commands.spawn(Chronolog {
    //     ticker_for_hundred: Some(Ticker { number: Some(1), timer: Some(Timer::from_seconds(100.0, TimerMode::Repeating)) }),
    //     ticker_for_ten:     Some(Ticker { number: Some(2), timer: Some(Timer::from_seconds(10.0,  TimerMode::Repeating)) }),
    //     ticker_for_one:     Some(Ticker { number: Some(5), timer: Some(Timer::from_seconds(1.0,   TimerMode::Repeating)) }),
    //     ticker_for_tenth:   Some(Ticker { number: Some(5), timer: Some(Timer::from_seconds(0.1,   TimerMode::Repeating)) }),
    //     ..Default::default()
    // });

    // ####################################################################################################### //

}
