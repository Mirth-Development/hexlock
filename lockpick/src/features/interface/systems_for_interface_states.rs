
use bevy::prelude::*;
use crate::features::interface::definitions::*;
use crate::features::interface::systems_for_interface_spawns::*;

pub struct Interfaces {}
impl Plugin for Interfaces {
    fn build(&self, app: &mut App) {

        app.add_systems(OnEnter(InterfaceStates::StartMenu), setup_start_menu);
        app.add_systems(OnExit(InterfaceStates::StartMenu), (record_start_menu_exit, cleanup_ui_entities).chain());

        app.add_systems(OnEnter(InterfaceStates::Game), setup_game);
        app.add_systems(OnExit(InterfaceStates::Game), (record_game_exit, cleanup_ui_entities).chain());
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

    // Exit Game Button
    spawn_ui_element(
        &mut commands, &asset_server, window,
        Some(Buttons::ExitGame),
        None,
        None,
        path_for_image,
        Vec3::new(x_anchor, 85.0, layer),
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


fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>
) -> Result<()> {

    let window = window_query.single()?;

    Ok(())
}

// UI STATE RECORDERS
fn record_start_menu_exit(mut history: ResMut<StateHistory>) { history.push(InterfaceStates::StartMenu); }
fn record_game_exit(mut history: ResMut<StateHistory>) { history.push(InterfaceStates::Game); }

// TRASH COLLECTOR
fn cleanup_ui_entities(
    mut commands: Commands,
    button_query: Query<Entity, With<Buttons>>,
    container_query: Query<Entity, With<Containers>>,
    label_query: Query<Entity, With<Labels>>,
)
{
    for entity in button_query.iter()       { commands.entity(entity).despawn(); }
    for entity in container_query.iter()    { commands.entity(entity).despawn(); }
    for entity in label_query.iter()        { commands.entity(entity).despawn(); }
}
