use bevy::ecs::component::Mutable;
use bevy::prelude::*;
use crate::features::animation::components::AnimationShake;
use crate::features::animation::systems::animation_controller;
use crate::features::interface::definitions::*;
use crate::features::interface::systems_for_interface_spawns::*;
use crate::features::lockpick::systems::*;
use crate::features::lockpick::component::*;
use crate::features::lock::components::*;
use crate::features::lock::systems::*;
use crate::features::lock::tumblers::systems::*;
use crate::features::lock::spring::systems::*;
use crate::features::controls::systems::*;
use crate::features::game_controller::game_effects::systems::handle_lifetime_timers;
use crate::features::game_controller::systems::{charge_chargebar, check_game_state, handle_game_state, spawn_charge_bar};

pub struct Interfaces {}
impl Plugin for Interfaces {
    fn build(&self, app: &mut App) {

        app.add_systems(OnEnter(InterfaceStates::StartMenu), setup_start_menu);
        app.add_systems(OnExit(InterfaceStates::StartMenu), (record_start_menu_exit, cleanup_entities).chain());

        app.add_systems(OnEnter(InterfaceStates::Level1), (setup_level_1, spawn_lockpick, load_lock_resources, spawn_lock, spawn_charge_bar).chain());
        app.add_systems(OnExit(InterfaceStates::Level1), (record_level_1_exit, cleanup_entities).chain());

        app.add_systems(OnEnter(InterfaceStates::Level2), (setup_level_2, spawn_lockpick, load_lock_resources, spawn_lock, spawn_charge_bar).chain());
        app.add_systems(OnExit(InterfaceStates::Level2), (record_level_2_exit, cleanup_entities).chain());

        app.add_systems(OnEnter(InterfaceStates::Level3), (setup_level_3, spawn_lockpick, load_lock_resources, spawn_lock, spawn_charge_bar).chain());
        app.add_systems(OnExit(InterfaceStates::Level3), (record_level_3_exit, cleanup_entities).chain());

        app.add_systems(OnEnter(InterfaceStates::Level4), (setup_level_4, spawn_lockpick, load_lock_resources, spawn_lock, spawn_charge_bar).chain());
        app.add_systems(OnExit(InterfaceStates::Level4), (record_level_4_exit, cleanup_entities).chain());

        app.add_systems(OnEnter(InterfaceStates::Level5), (setup_level_5, spawn_lockpick, load_lock_resources, spawn_lock, spawn_charge_bar).chain());
        app.add_systems(OnExit(InterfaceStates::Level5), (record_level_5_exit, cleanup_entities).chain());

        app.add_systems(OnEnter(InterfaceStates::WinScreen), (setup_win_screen).chain());
        app.add_systems(OnExit(InterfaceStates::WinScreen), (record_lose_screen_exit, cleanup_entities).chain());

        app.add_systems(OnEnter(InterfaceStates::LoseScreen), (setup_lose_screen).chain());
        app.add_systems(OnExit(InterfaceStates::LoseScreen), (record_win_screen_exit, cleanup_entities).chain());

        app.add_systems(Update, (
            move_to_focused_tumbler,
            tumbler_movement,
            lockpick_movement,
            user_control_system,
            timer_tumbler_finished,
            stretch_to_tumbler,
            charge_chargebar,
            handle_lockpick_message,
            handle_catching_tumblers,
            handle_tumbler_set,
            handle_escape_message,
            check_game_state,
            handle_game_state,
            handle_lockpick_charge,
            handle_lifetime_timers,

        )
            .chain()
            .run_if(in_level_state)
        );
    }
}

// UI SETUPS
fn setup_start_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>
) -> Result<()> {

    // Defining variables for UI elements.
    let window = window_query.single()?;
    let path_for_image: Option<&'static str> = Some("images/Button.png");
    let path_for_font = "fonts/Cinzel_Decorative.ttf";
    let color_of_text = Color::WHITE;
    let x_anchor = 50.0;
    let layer = 1.0;

    let button_width = 30.0;
    let button_aspect_ratio: Option<f32> = Some(120.0 / 20.0);
    let button_font_size = 0.02;

    let title_width = 55.0;
    let title_aspect_ratio: Option<f32> = Some(80.0 / 20.0);
    let title_font_size = 0.06;

    // Title Label
    spawn_ui_element(
        &mut commands, &asset_server, window,
        None,
        None,
        Some(Labels::Title),
        None,
        Vec3::new(x_anchor, 25.0, layer),
        title_width,
        title_aspect_ratio,
        Some(TextSpawn {
            content: "Hexlock",
            font_path: path_for_font,
            font_size_scale: title_font_size,
            color: color_of_text,
        })
    );

    // Play Button
    spawn_ui_element(
        &mut commands, &asset_server, window,
        Some(Buttons::Play),
        None,
        None,
        path_for_image,
        Vec3::new(x_anchor, 45.0, layer),
        button_width,
        button_aspect_ratio,
        Some(TextSpawn {
            content: "Play",
            font_path: path_for_font,
            font_size_scale: button_font_size,
            color: color_of_text,
        })
    );

    // Exit Level Button
    spawn_ui_element(
        &mut commands, &asset_server, window,
        Some(Buttons::ExitGame),
        None,
        None,
        path_for_image,
        Vec3::new(x_anchor, 60.0, layer),
        button_width,
        button_aspect_ratio,
        Some(TextSpawn {
            content: "Exit Game",
            font_path: path_for_font,
            font_size_scale: button_font_size,
            color: color_of_text,
        })
    );

    Ok(())
}

fn setup_level_1(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>
) -> Result<()> {

    let window = window_query.single()?;

    // Label for Level #
    spawn_ui_element(
        &mut commands, &asset_server, window,
        None,
        None,
        Some(Labels::Level),
        None,
        Vec3::new(10.0, 5.0, 1.0),
        10.0,
        None,
        Some(TextSpawn {
            content: "LEVEL 1",
            font_path: "fonts/Cinzel_Decorative.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );

    // Button for Next Level
    spawn_ui_element(
        &mut commands, &asset_server, window,
        Some(Buttons::GoToLevel2),
        None,
        None,
        Some("images/Button.png"),
        Vec3::new(10.0, 10.0, 2.0),
        10.0,
        Some(120.0 / 20.0),
        Some(TextSpawn {
            content: "Next Level",
            font_path: "fonts/Spectral.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );

    Ok(())
}

fn setup_level_2(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>
) -> Result<()> {

    let window = window_query.single()?;

    // Label for Level #
    spawn_ui_element(
        &mut commands, &asset_server, window,
        None,
        None,
        Some(Labels::Level),
        None,
        Vec3::new(10.0, 5.0, 1.0),
        10.0,
        None,
        Some(TextSpawn {
            content: "LEVEL 2",
            font_path: "fonts/Cinzel_Decorative.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );

    // Button for Previous Level
    spawn_ui_element(
        &mut commands, &asset_server, window,
        Some(Buttons::GoToLevel1),
        None,
        None,
        Some("images/Button.png"),
        Vec3::new(10.0, 10.0, 2.0),
        10.0,
        Some(120.0 / 20.0),
        Some(TextSpawn {
            content: "Previous Level",
            font_path: "fonts/Spectral.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );

    // Button for Next Level
    spawn_ui_element(
        &mut commands, &asset_server, window,
        Some(Buttons::GoToLevel3),
        None,
        None,
        Some("images/Button.png"),
        Vec3::new(10.0, 15.0, 2.0),
        10.0,
        Some(120.0 / 20.0),
        Some(TextSpawn {
            content: "Next Level",
            font_path: "fonts/Spectral.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );

    Ok(())
}

fn setup_level_3(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>
) -> Result<()> {

    let window = window_query.single()?;

    // Label for Level #
    spawn_ui_element(
        &mut commands, &asset_server, window,
        None,
        None,
        Some(Labels::Level),
        None,
        Vec3::new(10.0, 5.0, 1.0),
        10.0,
        None,
        Some(TextSpawn {
            content: "LEVEL 3",
            font_path: "fonts/Cinzel_Decorative.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );

    // Button for Previous Level
    spawn_ui_element(
        &mut commands, &asset_server, window,
        Some(Buttons::GoToLevel2),
        None,
        None,
        Some("images/Button.png"),
        Vec3::new(10.0, 10.0, 2.0),
        10.0,
        Some(120.0 / 20.0),
        Some(TextSpawn {
            content: "Previous Level",
            font_path: "fonts/Spectral.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );

    // Button for Next Level
    spawn_ui_element(
        &mut commands, &asset_server, window,
        Some(Buttons::GoToLevel4),
        None,
        None,
        Some("images/Button.png"),
        Vec3::new(10.0, 15.0, 2.0),
        10.0,
        Some(120.0 / 20.0),
        Some(TextSpawn {
            content: "Next Level",
            font_path: "fonts/Spectral.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );

    Ok(())
}

fn setup_level_4(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>
) -> Result<()> {

    let window = window_query.single()?;

    // Label for Level #
    spawn_ui_element(
        &mut commands, &asset_server, window,
        None,
        None,
        Some(Labels::Level),
        None,
        Vec3::new(10.0, 5.0, 1.0),
        10.0,
        None,
        Some(TextSpawn {
            content: "LEVEL 4",
            font_path: "fonts/Cinzel_Decorative.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );

    // Button for Previous Level
    spawn_ui_element(
        &mut commands, &asset_server, window,
        Some(Buttons::GoToLevel3),
        None,
        None,
        Some("images/Button.png"),
        Vec3::new(10.0, 10.0, 2.0),
        10.0,
        Some(120.0 / 20.0),
        Some(TextSpawn {
            content: "Previous Level",
            font_path: "fonts/Spectral.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );

    // Button for Next Level
    spawn_ui_element(
        &mut commands, &asset_server, window,
        Some(Buttons::GoToLevel5),
        None,
        None,
        Some("images/Button.png"),
        Vec3::new(10.0, 15.0, 2.0),
        10.0,
        Some(120.0 / 20.0),
        Some(TextSpawn {
            content: "Next Level",
            font_path: "fonts/Spectral.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );

    Ok(())
}

fn setup_level_5(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>
) -> Result<()> {

    let window = window_query.single()?;

    // Label for Level #
    spawn_ui_element(
        &mut commands, &asset_server, window,
        None,
        None,
        Some(Labels::Level),
        None,
        Vec3::new(10.0, 5.0, 1.0),
        10.0,
        None,
        Some(TextSpawn {
            content: "LEVEL 5",
            font_path: "fonts/Cinzel_Decorative.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );

    // Button for Previous Level
    spawn_ui_element(
        &mut commands, &asset_server, window,
        Some(Buttons::GoToLevel4),
        None,
        None,
        Some("images/Button.png"),
        Vec3::new(10.0, 10.0, 2.0),
        10.0,
        Some(120.0 / 20.0),
        Some(TextSpawn {
            content: "Previous Level",
            font_path: "fonts/Spectral.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );

    Ok(())
}

fn setup_win_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>
) -> Result<()> {

    let window = window_query.single()?;

    // Label for Level #
    spawn_ui_element(
        &mut commands, &asset_server, window,
        None,
        None,
        Some(Labels::Level),
        None,
        Vec3::new(10.0, 5.0, 1.0),
        10.0,
        None,
        Some(TextSpawn {
            content: "WIN",
            font_path: "fonts/Cinzel_Decorative.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );

    Ok(())
}

fn setup_lose_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>
) -> Result<()> {

    let window = window_query.single()?;

    // Label for Level #
    spawn_ui_element(
        &mut commands, &asset_server, window,
        None,
        None,
        Some(Labels::Level),
        None,
        Vec3::new(10.0, 5.0, 1.0),
        10.0,
        None,
        Some(TextSpawn {
            content: "WIN",
            font_path: "fonts/Cinzel_Decorative.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );

    Ok(())
}




// UI STATE RECORDERS
fn record_start_menu_exit(mut history: ResMut<StateHistory>) { history.push(InterfaceStates::StartMenu); }
fn record_level_1_exit(mut history: ResMut<StateHistory>) { history.push(InterfaceStates::Level1); }
fn record_level_2_exit(mut history: ResMut<StateHistory>) { history.push(InterfaceStates::Level2); }
fn record_level_3_exit(mut history: ResMut<StateHistory>) { history.push(InterfaceStates::Level3); }
fn record_level_4_exit(mut history: ResMut<StateHistory>) { history.push(InterfaceStates::Level4); }
fn record_level_5_exit(mut history: ResMut<StateHistory>) { history.push(InterfaceStates::Level5); }

fn record_lose_screen_exit(mut history: ResMut<StateHistory>) { history.push(InterfaceStates::LoseScreen); } //Are these required?
fn record_win_screen_exit(mut history: ResMut<StateHistory>) { history.push(InterfaceStates::WinScreen); }

// TRASH COLLECTOR
fn cleanup_entities(
    mut commands: Commands,
    button_query: Query<Entity, With<Buttons>>,
    container_query: Query<Entity, With<Containers>>,
    label_query: Query<Entity, With<Labels>>,
    lock_query: Query<Entity, With<LockComponent>>,
    lockpick_query: Query<Entity, With<LockpickComponent>>,
)
{
    for entity in button_query.iter()    { commands.entity(entity).despawn(); }
    for entity in container_query.iter() { commands.entity(entity).despawn(); }
    for entity in label_query.iter()     { commands.entity(entity).despawn(); }
    for entity in lock_query.iter()      { commands.entity(entity).despawn(); }
    for entity in lockpick_query.iter()  { commands.entity(entity).despawn(); }
}

// CHECKING IF IN-LEVEL
fn in_level_state(
    state: Res<State<InterfaceStates>>
) -> bool
{
    matches!(
        state.get(),
        InterfaceStates::Level1 |
        InterfaceStates::Level2 |
        InterfaceStates::Level3 |
        InterfaceStates::Level4 |
        InterfaceStates::Level5
    )
}

