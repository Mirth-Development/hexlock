use bevy::prelude::*;
use crate::features::interface::definitions::*;
use crate::features::interface::systems_for_spawns::*;
use crate::features::lockpick::systems::*;
use crate::features::lockpick::component::*;
use crate::features::lock::components::*;
use crate::features::lock::systems::*;
use crate::features::lock::tumblers::systems::*;
use crate::features::lock::spring::systems::*;
use crate::features::controls::systems::*;
use crate::features::game_controller::components::{ChargeBarMarker};
use crate::features::game_controller::game_effects::systems::handle_lifetime_timers;
use crate::features::game_controller::game_timer::definitions::TheTimer;
use crate::features::game_controller::systems::{charge_charge_bar, check_game_state, check_tumbler_order, enter_arrow_code, handle_game_state, spawn_charge_bar, spawn_lock_order};



pub struct SystemsForUserInterfaceStates {}
impl Plugin for SystemsForUserInterfaceStates {
    fn build(&self, app: &mut App) {

        app.add_systems(OnEnter(Interfaces::StartMenu), setup_start_menu);
        app.add_systems(OnExit(Interfaces::StartMenu), (record_start_menu_exit, cleanup_entities).chain());

        app.add_systems(OnEnter(Interfaces::Level1), (setup_level_1, load_lock_resources, spawn_lock, spawn_lockpick, spawn_charge_bar,spawn_lock_order).chain());
        app.add_systems(OnExit(Interfaces::Level1), (record_level_1_exit, cleanup_entities).chain());

        app.add_systems(OnEnter(Interfaces::Level2), (setup_level_2, load_lock_resources, spawn_lock, spawn_lockpick, spawn_charge_bar,spawn_lock_order).chain());
        app.add_systems(OnExit(Interfaces::Level2), (record_level_2_exit, cleanup_entities).chain());

        app.add_systems(OnEnter(Interfaces::Level3), (setup_level_3, load_lock_resources, spawn_lock, spawn_lockpick, spawn_charge_bar,spawn_lock_order).chain());
        app.add_systems(OnExit(Interfaces::Level3), (record_level_3_exit, cleanup_entities).chain());

        app.add_systems(OnEnter(Interfaces::Level4), (setup_level_4, load_lock_resources, spawn_lock, spawn_lockpick, spawn_charge_bar,spawn_lock_order).chain());
        app.add_systems(OnExit(Interfaces::Level4), (record_level_4_exit, cleanup_entities).chain());

        app.add_systems(OnEnter(Interfaces::Level5), (setup_level_5, load_lock_resources, spawn_lock, spawn_lockpick, spawn_charge_bar,spawn_lock_order).chain());
        app.add_systems(OnExit(Interfaces::Level5), (record_level_5_exit, cleanup_entities).chain());

        app.add_systems(OnEnter(Interfaces::Won), setup_won);
        app.add_systems(OnExit(Interfaces::Won), (record_won_exit, cleanup_entities).chain());

        app.add_systems(OnEnter(Interfaces::Lost), setup_lost);
        app.add_systems(OnExit(Interfaces::Lost), (record_lost_exit, cleanup_entities).chain());

        app.add_systems(Update, (
            move_to_focused_tumbler,
            tumbler_movement,
            lockpick_movement,
            user_control_system,
            timer_tumbler_finished,
            stretch_to_tumbler,
            charge_charge_bar,
            enter_arrow_code,
            handle_lockpick_message,
            handle_catching_tumblers,
            handle_tumbler_set,
            check_game_state,
            check_tumbler_order,
            handle_game_state,
            handle_lockpick_charge,
            handle_lifetime_timers,
        ).chain().run_if(in_level_state)
        );
    }
}

// UI SETUPS
fn setup_start_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>,
    images: Res<InterfaceImages>,
) -> Result<()> {

    // Defining variables for UI elements.
    let window = window_query.single()?;
    let path_for_font = "fonts/Cinzel_Decorative.ttf";
    let color_of_text = Color::WHITE;
    let x_anchor = 50.0;
    let layer = 1.0;

    let button_width = 30.0;
    let button_aspect_ratio: Option<f32> = Some(1115.0 / 200.0);
    let button_font_size = 0.02;

    let title_width = 55.0;
    let title_aspect_ratio: Option<f32> = Some(80.0 / 20.0);
    let title_font_size = 0.09;

    // Spawning background visual.
    spawn_background(&mut commands, window, Some(&images.background_start));

    // Title Label
    spawn_ui_element(
        &mut commands, &asset_server, window,
        None,
        None,
        Some(Labels::Title),
        None,
        None,
        Vec3::new(x_anchor, 30.0, layer),
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
        None,
        Some(&images.button),
        Vec3::new(x_anchor, 50.0, layer),
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
        None,
        Some(&images.button),
        Vec3::new(x_anchor, 65.0, layer),
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
    window_query: Query<&Window>,
    images: Res<InterfaceImages>,
) -> Result<()> {

    let window = window_query.single()?;

    // Spawning background visual.
    spawn_background(&mut commands, window, Some(&images.background_level));

    // Spawning timer related visuals.
    spawn_countdown(&mut commands, &asset_server, window, &images);

    // Spawning title related visuals and buttons.
    spawn_level_title(&mut commands, &asset_server, window, &images, "LEVEL 1");

    Ok(())
}

fn setup_level_2(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>,
    images: Res<InterfaceImages>,
    mut game_timer: ResMut<TheTimer>,
) -> Result<()> {

    let window = window_query.single()?;

    // Spawning card visuals.
    spawn_cards(&mut commands, &asset_server, window, &images, &mut game_timer);

    // Spawning background visual.
    spawn_background(&mut commands, window, Some(&images.background_level));

    // Spawning timer related visuals.
    spawn_countdown(&mut commands, &asset_server, window, &images);

    // Spawning title related visuals and buttons.
    spawn_level_title(&mut commands, &asset_server, window, &images, "LEVEL 2");

    Ok(())
}

fn setup_level_3(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>,
    images: Res<InterfaceImages>,
    mut game_timer: ResMut<TheTimer>,
) -> Result<()> {

    let window = window_query.single()?;

    // Spawning card visuals.
    spawn_cards(&mut commands, &asset_server, window, &images, &mut game_timer);

    // Spawning background visual.
    spawn_background(&mut commands, window, Some(&images.background_level));

    // Spawning timer related visuals.
    spawn_countdown(&mut commands, &asset_server, window, &images);

    // Spawning title related visuals and buttons.
    spawn_level_title(&mut commands, &asset_server, window, &images, "LEVEL 3");

    Ok(())
}

fn setup_level_4(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>,
    images: Res<InterfaceImages>,
    mut game_timer: ResMut<TheTimer>,
) -> Result<()> {

    let window = window_query.single()?;

    // Spawning card visuals.
    spawn_cards(&mut commands, &asset_server, window, &images, &mut game_timer);

    // Spawning background visual.
    spawn_background(&mut commands, window, Some(&images.background_level));

    // Spawning timer related visuals.
    spawn_countdown(&mut commands, &asset_server, window, &images);

    // Spawning title related visuals and buttons.
    spawn_level_title(&mut commands, &asset_server, window, &images, "LEVEL 4");

    Ok(())
}

fn setup_level_5(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>,
    images: Res<InterfaceImages>,
    mut game_timer: ResMut<TheTimer>,
) -> Result<()> {

    let window = window_query.single()?;

    // Spawning card visuals.
    spawn_cards(&mut commands, &asset_server, window, &images, &mut game_timer);

    // Spawning background visual.
    spawn_background(&mut commands, window, Some(&images.background_level));

    // Spawning timer related visuals.
    spawn_countdown(&mut commands, &asset_server, window, &images);

    // Spawning title related visuals and buttons.
    spawn_level_title(&mut commands, &asset_server, window, &images, "LEVEL 5");

    Ok(())
}

fn setup_won(
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
        None,
        Vec3::new(10.0, 5.0, 1.0),
        10.0,
        None,
        Some(TextSpawn {
            content: "WON",
            font_path: "fonts/Cinzel_Decorative.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );

    Ok(())
}

fn setup_lost(
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
        None,
        Vec3::new(10.0, 5.0, 1.0),
        10.0,
        None,
        Some(TextSpawn {
            content: "LOST",
            font_path: "fonts/Cinzel_Decorative.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );

    Ok(())
}

// UI STATE RECORDERS
fn record_start_menu_exit(mut history: ResMut<StateHistory>) { history.push(Interfaces::StartMenu); }
fn record_level_1_exit(mut history: ResMut<StateHistory>) { history.push(Interfaces::Level1); }
fn record_level_2_exit(mut history: ResMut<StateHistory>) { history.push(Interfaces::Level2); }
fn record_level_3_exit(mut history: ResMut<StateHistory>) { history.push(Interfaces::Level3); }
fn record_level_4_exit(mut history: ResMut<StateHistory>) { history.push(Interfaces::Level4); }
fn record_level_5_exit(mut history: ResMut<StateHistory>) { history.push(Interfaces::Level5); }
fn record_lost_exit(mut history: ResMut<StateHistory>) { history.push(Interfaces::Lost); }
fn record_won_exit(mut history: ResMut<StateHistory>) { history.push(Interfaces::Won); }

// TRASH COLLECTOR
fn cleanup_entities(
    mut commands: Commands,
    button_query: Query<Entity, With<Buttons>>,
    container_query: Query<Entity, With<Containers>>,
    label_query: Query<Entity, With<Labels>>,
    lock_query: Query<Entity, With<LockComponent>>,
    lockpick_query: Query<Entity, With<LockpickComponent>>,
    charge_bar_marker: Query<Entity, With<ChargeBarMarker>>,
)
{
    for entity in button_query.iter()    { commands.entity(entity).despawn(); }
    for entity in container_query.iter() { commands.entity(entity).despawn(); }
    for entity in label_query.iter()     { commands.entity(entity).despawn(); }
    for entity in lock_query.iter()      { commands.entity(entity).despawn(); }
    for entity in lockpick_query.iter()  { commands.entity(entity).despawn(); }
    for entity in charge_bar_marker.iter()  { commands.entity(entity).despawn(); }
}

// CHECKING IF IN-LEVEL
pub fn in_level_state(
    state: Res<State<Interfaces>>
) -> bool
{
    matches!(
        state.get(),
        Interfaces::Level1 |
        Interfaces::Level2 |
        Interfaces::Level3 |
        Interfaces::Level4 |
        Interfaces::Level5
    )
}
