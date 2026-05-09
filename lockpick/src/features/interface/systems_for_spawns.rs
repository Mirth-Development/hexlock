
use bevy::prelude::*;
use bevy::window::WindowResized;
use crate::features::interface::definitions::*;
use crate::features::game_controller::game_timer::definitions::*;
use crate::features::game_controller::resources::InputtedArrowCode;
use crate::features::interface::systems_for_states::*;
use crate::features::lock::tumblers::components::*;
use crate::features::lock::tumblers::resources::*;

pub struct SystemsForUserInterfaceSpawns {}
impl Plugin for SystemsForUserInterfaceSpawns {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (resize, resize_background, handle_button_interactions).chain());
        app.add_systems(Update, (
            despawn_combo,
            spawn_combo,
            despawn_countdown_digits,
            spawn_countdown_digits
        ).chain().run_if(in_level_state));
    }
}

pub fn spawn_ui_element(
    commands: &mut Commands,
    asset_server: &AssetServer,
    window: &Window,
    ui_button: Option<Buttons>,
    ui_container: Option<Containers>,
    ui_label: Option<Labels>,
    image_color: Option<Color>,
    image_handle: Option<&Handle<Image>>,    // IMAGE_HANDLE : Takes in a handle for an image that has been loaded into the asset server.
    position: Vec3,                         // POSITION : Percentage based with origin centered at the top left of the window.  Z values should be discrete.
    size_of_element: f32,                   // SIZE_OF_ELEMENT : Size is based on the width of the window and is percentage based.
                                            //      A value of 20.0 equals 20% of the window's width.  You use this value to
                                            //      determine the overall image size of the UI element.
    aspect_ratio: Option<f32>,              // ASPECT_RATIO : Can manipulate the ratio dimensions of an element.  Best to throw in calculated values
                                            //      16 (width) / 9 (height) so that one can understand the difference between the width and height.
    text: Option<TextSpawn>,                // TEXT : This is an optional element, by using it text can be placed onto a UI element.
                                            //      Position of the text is relative to the image that the UI element uses.  You can
                                            //      pass None into a call of this function if an element isn't supposed to contain text.
) -> Entity
{

    // Calculating UI component size (relative to width of window and aspect_ratio).
    let width_px = window.width() * (size_of_element / 100.0);
    let height_px = width_px / aspect_ratio.unwrap_or(1.0);
    let width_half_size = size_of_element / 2.0;
    let height_half_size = (height_px / window.height()) * 100.0 / 2.0;

    // Assigning UI attributes - image, position, layer, and size.
    let mut entity = commands.spawn((
        Button,
        ZIndex(position.z as i32),
        Visibility::default(),
        Sizer {
            position,
            size_of_element,
            aspect_ratio,
        },
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(position.x - width_half_size),
            top: Val::Percent(position.y - height_half_size),
            width: Val::Percent(size_of_element),
            height: Val::Px(height_px),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
    ));

    // Applying an image to the UI element if one was passed in.
    if let Some(handle) = image_handle {
        entity.insert(ImageNode {
            image: handle.clone(),
            color: image_color.unwrap_or(Color::WHITE),
            ..default()
        });
    }

    // Declaring Types for Entity (If Any Were Provided)
    if let Some(button) = ui_button {
        entity.insert(button);
    }
    if let Some(container) = ui_container {
        entity.insert(container);
    }
    if let Some(label) = ui_label {
        entity.insert(label);
    }

    // Assigning text to the UI element if any text was provided.
    entity.with_children(|parent| {
        if let Some(text_spawn) = text {
            parent.spawn((
                Visibility::default(),
                Text::new(text_spawn.content),
                TextColor(text_spawn.color),
                TextLayout::new_with_justify(Justify::Center),
                TextFont {
                    font: asset_server.load(text_spawn.font_path),
                    font_size: window.width() * text_spawn.font_size_scale,
                    ..default()
                },
                text_spawn,
            ));
        }
    });

    entity.id()
}

// Doesn't use UI spawning since it needs to be behind the world elements.  UI elements layer over
// world elements no matter what.
pub fn spawn_background(
    commands: &mut Commands,
    window: &Window,
    image: Option<&Handle<Image>>,
)
{
    let aspect_ratio = window.width() / window.height();
    let world_height = 1080.0 * 1.3;
    let world_width = world_height * aspect_ratio;

    if let Some(handle) = image {
        commands.spawn((
            Sprite {
                image: handle.clone(),
                custom_size: Some(Vec2::new(world_width, world_height)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, -100.0),
            Containers::Background,
        ));
    }
}

/// Used to create confirmation dialogs that can have different text within them based on what's
/// passed into dialog_text.
pub fn spawn_confirmation(
    commands: &mut Commands,
    asset_server: &AssetServer,
    window: &Window,
    images: &InterfaceImages,
    dialog_text: &'static str,
)
{
    // Container
    spawn_ui_element(
        commands,
        asset_server,
        window,
        None,
        Some(Containers::Confirmation),
        None,
        None,
        Some(&images.background_panel),
        Vec3::new(50.0, 40.0, 3.0),
        35.0,
        Some(530.0 / 230.0),
        None
    );

    // Label
    spawn_ui_element(
        commands,
        asset_server,
        window,
        None,
        Some(Containers::Confirmation),
        Some(Labels::Confirmation),
        None,
        None,
        Vec3::new(50.0, 35.0, 4.0),
        28.0,
        Some(100.0 / 20.0),
        Some(TextSpawn {
            content: dialog_text,
            font_path: "fonts/Spectral.ttf",
            font_size_scale: 0.013,
            color: Color::WHITE,
        })
    );

    // Yes Button
    spawn_ui_element(
        commands,
        asset_server,
        window,
        Some(Buttons::Yes),
        Some(Containers::Confirmation),
        None,
        None,
        Some(&images.button),
        Vec3::new(42.5, 45.0, 4.0),
        12.5,
        Some(1115.0 / 200.0),
        Some(TextSpawn {
            content: "YES",
            font_path: "fonts/Cinzel.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );

    // No Button
    spawn_ui_element(
        commands,
        asset_server,
        window,
        Some(Buttons::No),
        Some(Containers::Confirmation),
        None,
        None,
        Some(&images.button),
        Vec3::new(57.5, 45.0, 4.0),
        12.5,
        Some(1115.0 / 200.0),
        Some(TextSpawn {
            content: "NO",
            font_path: "fonts/Cinzel.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );
}


pub fn spawn_level_title(
    commands: &mut Commands,
    asset_server: &AssetServer,
    window: &Window,
    images: &InterfaceImages,
    title_text: &'static str,
    previous_level: Option<Buttons>,
    next_level: Option<Buttons>,
)
{
    // Label for Level #
    spawn_ui_element(
        commands, &asset_server, window,
        None,
        None,
        Some(Labels::Level),
        None,
        None,
        Vec3::new(21.5, 16.0, 1.0),
        30.0,
        None,
        Some(TextSpawn {
            content: title_text,
            font_path: "fonts/Cinzel_Decorative.ttf",
            font_size_scale: 0.055,
            color: Color::WHITE,
        })
    );

    // Button for Previous Level
    spawn_ui_element(
        commands, &asset_server, window,
        previous_level,
        None,
        None,
        None,
        Some(&images.button),
        Vec3::new(16.0, 5.0, 2.0),
        10.0,
        Some(1115.0 / 200.0),
        Some(TextSpawn {
            content: "Previous",
            font_path: "fonts/Spectral.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );

    // Button for Next Level
    spawn_ui_element(
        commands, &asset_server, window,
        next_level,
        None,
        None,
        None,
        Some(&images.button),
        Vec3::new(27.0, 5.0, 2.0),
        10.0,
        Some(1115.0 / 200.0),
        Some(TextSpawn {
            content: "Next",
            font_path: "fonts/Spectral.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );
}

pub fn spawn_countdown(
    commands: &mut Commands,
    asset_server: &AssetServer,
    window: &Window,
    images: &InterfaceImages,
)
{
    // Background
    spawn_ui_element(
        commands,
        asset_server,
        window,
        None,
        Some(Containers::Timer),
        None,
        None,
        Some(&images.background_panel),
        Vec3::new(89.0, 10.0, 3.0),
        20.0,
        Some(500.0 / 230.0),
        None
    );

    // Label
    spawn_ui_element(
        commands,
        asset_server,
        window,
        None,
        Some(Containers::Timer),
        None,
        None,
        None,
        Vec3::new(89.0, 6.0, 4.0),
        15.0,
        Some(100.0 / 20.0),
        Some(TextSpawn {
            content: "Time Until You Lose",
            font_path: "fonts/Cinzel_Decorative.ttf",
            font_size_scale: 0.01,
            color: Color::WHITE,
        })
    );
}

pub fn spawn_countdown_digits(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>,
    the_timer: Res<TheTimer>,
    images: Res<InterfaceImages>,
) -> Result<()>
{
    // Had to throw down this error catcher over the usual question mark operator usage.  As for why...?
    // I don't really understand it.  All I can say is that a panic event would occur when the program
    // would close via forced exit (close button) and the usual result handler wouldn't work.  My best
    // guess is that this system is being used extensively (each frame) and it's rather large (lot to process) so it's possible
    // that the pre-check for window wasn't enough for the entirety of how long it takes to process this function
    // for each and every frame.  I kind of doubt that's the problem, but I can't think of anything else.
    let Ok(window) = window_query.single()
    else { return Ok(()); };

    let digit_images: [&Handle<Image>; 10] = [
        &images.digit_zero,
        &images.digit_one,
        &images.digit_two,
        &images.digit_three,
        &images.digit_four,
        &images.digit_five,
        &images.digit_six,
        &images.digit_seven,
        &images.digit_eight,
        &images.digit_nine,
    ];

    // Obtaining current digit values.  Have to cast to usize because Rust arrays can't take u32?
    // Did not know that.  Doesn't usize account for u32?  ME DON'T UNDERSTAND!
    // EDIT: You can't even use index access on an array?!  WTF?! I'm sure there's some brilliant reason
    // but that drives me insane.
    let countdown: Vec<char> = the_timer.chronolog.get_countdown_string(3, 3).chars().collect();
    let hundreds = countdown[0].to_digit(10).unwrap_or(0) as usize;
    let tens     = countdown[1].to_digit(10).unwrap_or(0) as usize;
    let ones     = countdown[2].to_digit(10).unwrap_or(0) as usize;

    // Digit for Hundreds Place
    let digit_hundreds = spawn_ui_element(
        &mut commands,
        &asset_server,
        window,
        None,
        Some(Containers::Timer),
        None,
        None,
        Some(digit_images[hundreds]),
        Vec3::new(86.0, 12.0, 4.0),
        2.5,
        Some(85.0 / 135.0),
        None
    );

    // Digit for Tens Place
    let digit_tens = spawn_ui_element(
        &mut commands,
        &asset_server,
        window,
        None,
        Some(Containers::Timer),
        None,
        None,
        Some(digit_images[tens]),
        Vec3::new(89.0, 12.0, 4.0),
        2.5,
        Some(85.0 / 135.0),
        None
    );

    // Digit for Ones Place
    let digit_ones = spawn_ui_element(
        &mut commands,
        &asset_server,
        window,
        None,
        Some(Containers::Timer),
        None,
        None,
        Some(digit_images[ones]),
        Vec3::new(92.0, 12.0, 4.0),
        2.5,
        Some(85.0 / 135.0),
        None
    );

    // Marking digit entities to delete them each frame.
    commands.entity(digit_hundreds).insert(Chronodigit);
    commands.entity(digit_tens).insert(Chronodigit);
    commands.entity(digit_ones).insert(Chronodigit);

    Ok(())
}

/// Spawns timer cards that players can select to get a buff before moving onto the next level.
pub fn spawn_cards(
    commands: &mut Commands,
    asset_server: &AssetServer,
    window: &Window,
    images: &InterfaceImages,
)
{
    // Card for Timer Increase
    {
        // Container
        spawn_ui_element(
            commands,
            asset_server,
            window,
            Some(Buttons::CardTimerIncrease),
            Some(Containers::Card),
            None,
            None,
            Some(&images.card_increase_time),
            Vec3::new(35.0, 50.0, 3.0),
            25.0,
            Some(560.0 / 920.0),
            None
        );

        // Label for Title
        spawn_ui_element(
            commands,
            asset_server,
            window,
            Some(Buttons::CardTimerIncrease),
            Some(Containers::Card),
            Some(Labels::Card),
            None,
            None,
            Vec3::new(35.25, 50.0, 4.0),
            20.0,
            Some(100.0 / 20.0),
            Some(TextSpawn {
                content: "Increase Timer",
                font_path: "fonts/Cinzel_Decorative.ttf",
                font_size_scale: 0.015,
                color: Color::WHITE,
            })
        );

        // Label for Description
        spawn_ui_element(
            commands,
            asset_server,
            window,
            Some(Buttons::CardTimerIncrease),
            Some(Containers::Card),
            Some(Labels::Card),
            None,
            None,
            Vec3::new(35.25, 57.0, 4.0),
            20.0,
            Some(100.0 / 20.0),
            Some(TextSpawn {
                content: "Clicking this card will add X amount of seconds to your timer.",
                font_path: "fonts/Spectral.ttf",
                font_size_scale: 0.01,
                color: Color::WHITE,
            })
        );
    }

    // Card for Set Timer Increase
    {
        // Container
        spawn_ui_element(
            commands,
            asset_server,
            window,
            Some(Buttons::CardSetTimeIncrease),
            Some(Containers::Card),
            None,
            None,
            Some(&images.card_increase_set_time),
            Vec3::new(65.0, 50.0, 3.0),
            25.0,
            Some(560.0 / 920.0),
            None
        );

        // Label for Title
        spawn_ui_element(
            commands,
            asset_server,
            window,
            Some(Buttons::CardSetTimeIncrease),
            Some(Containers::Card),
            Some(Labels::Card),
            None,
            None,
            Vec3::new(65.25, 50.0, 4.0),
            20.0,
            Some(100.0 / 20.0),
            Some(TextSpawn {
                content: "Increase Set Duration",
                font_path: "fonts/Cinzel_Decorative.ttf",
                font_size_scale: 0.015,
                color: Color::WHITE,
            })
        );

        // Label for Description
        spawn_ui_element(
            commands,
            asset_server,
            window,
            Some(Buttons::CardSetTimeIncrease),
            Some(Containers::Card),
            Some(Labels::Card),
            None,
            None,
            Vec3::new(65.25, 59.0, 4.0),
            20.0,
            Some(100.0 / 20.0),
            Some(TextSpawn {
                content: "Clicking this card will add X amount of seconds to how long your tumblers stay in the set position.",
                font_path: "fonts/Spectral.ttf",
                font_size_scale: 0.01,
                color: Color::WHITE,
            })
        );
    }
}

pub fn spawn_combo(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut inputted_arrow_code: ResMut<InputtedArrowCode>,
    window_query: Query<&Window>,
    tumbler_query: Query<(Entity, &TumblerMagicComponent),( With<FocusedTumblerComponent>)>,
    images: Res<InterfaceImages>,
) -> Result<()>
{

    // Had to throw down this error catcher over the usual question mark operator usage.  As for why...?
    // I don't really understand it.  All I can say is that a panic event would occur when the program
    // would close via forced exit (close button) and the usual result handler wouldn't work.  My best
    // guess is that this system is being used extensively (each frame) and it's rather large (lot to process) so it's possible
    // that the pre-check for window wasn't enough for the entirety of how long it takes to process this function
    // for each and every frame.  I kind of doubt that's the problem, but I can't think of anything else.
    let Ok(window) = window_query.single()
    else { return Ok(()); };
    let Ok((entity, tumbler)) = tumbler_query.single()
    else { return Ok(()); };

    // Creating a list of 4 images off of the queried tumbler's arrow codes.
    let mut list_of_images: Vec<&Handle<Image>> = Vec::new();
    for code in &tumbler.arrow_code {
        match code {
            Directions::Up      => list_of_images.push(&images.arrow_up),
            Directions::Down    => list_of_images.push(&images.arrow_down),
            Directions::Left    => list_of_images.push(&images.arrow_left),
            Directions::Right   => list_of_images.push(&images.arrow_right),
        }
    }

    let mut num_of_matched_arrows = 0;
    for (x, dir) in inputted_arrow_code.entered_code.iter().enumerate(){
        if x < tumbler.arrow_code.len() && *dir == tumbler.arrow_code[x] {
            num_of_matched_arrows += 1;
        } else {
            break
        }
    }

    // Container
    let container = spawn_ui_element(
        &mut commands,
        &asset_server,
        window,
        None,
        Some(Containers::Combo),
        None,
        None,
        Some(&images.background_panel),
        Vec3::new(27.5, 42.0, 3.0),
        23.0,
        Some(550.0 / 200.0),
        None
    );

    // Label for Title
    let label = spawn_ui_element(
        &mut commands,
        &asset_server,
        window,
        None,
        Some(Containers::Combo),
        None,
        None,
        None,
        Vec3::new(27.5, 37.0, 4.0),
        20.0,
        Some(100.0 / 20.0),
        Some(TextSpawn {
            content: "Combo for Tumbler",
            font_path: "fonts/Cinzel_Decorative.ttf",
            font_size_scale: 0.0075,
            color: Color::WHITE,
        })
    );

    // Arrow #1
    let arrow_1 = spawn_ui_element(
        &mut commands,
        &asset_server,
        window,
        None,
        Some(Containers::Combo),
        None,
        if 1 <= num_of_matched_arrows {Some(Color::srgb(0.0,1.0,0.0))} else {None},
        Some(list_of_images[0]),
        Vec3::new(20.0, 43.0, 4.0),
        4.0,
        Some(150.0 / 150.0),
        None
    );

    // Arrow #2
    let arrow_2 = spawn_ui_element(
        &mut commands,
        &asset_server,
        window,
        None,
        Some(Containers::Combo),
        None,
        if 2 <= num_of_matched_arrows {Some(Color::srgb(0.0,1.0,0.0))} else {None},
        Some(list_of_images[1]),
        Vec3::new(25.0, 43.0, 4.0),
        4.0,
        Some(150.0 / 150.0),
        None
    );

    // Arrow #3
    let arrow_3 = spawn_ui_element(
        &mut commands,
        &asset_server,
        window,
        None,
        Some(Containers::Combo),
        None,
        if 3 <= num_of_matched_arrows {Some(Color::srgb(0.0,1.0,0.0))} else {None},
        Some(list_of_images[2]),
        Vec3::new(30.0, 43.0, 4.0),
        4.0,
        Some(150.0 / 150.0),
        None
    );

    // Arrow #4
    let arrow_4 = spawn_ui_element(
        &mut commands,
        &asset_server,
        window,
        None,
        Some(Containers::Combo),
        None,
        if 4 <= num_of_matched_arrows {Some(Color::srgb(0.0,1.0,0.0))} else {None},
        Some(list_of_images[3]),
        Vec3::new(35.0, 43.0, 4.0),
        4.0,
        Some(150.0 / 150.0),
        None
    );

    // Marking combo UI so that they can be deleted by their despawner.
    // commands.entity(container).insert(ComboArrow);
    // commands.entity(label).insert(ComboArrow);
    commands.entity(arrow_1).insert(ComboArrow{position: 1, code: tumbler.arrow_code[0]});
    commands.entity(arrow_2).insert(ComboArrow{position: 2, code: tumbler.arrow_code[1]});
    commands.entity(arrow_3).insert(ComboArrow{position: 3, code: tumbler.arrow_code[2]});
    commands.entity(arrow_4).insert(ComboArrow{position: 4, code: tumbler.arrow_code[3]});

    Ok(())
}

/// Used to obliterate arrow spawns when the focused tumbler marker changes.
pub fn despawn_combo(
    mut commands: Commands,
    arrow_query: Query<Entity, With<ComboArrow>>,
) {
    for arrow in arrow_query.iter() {
        commands.entity(arrow).despawn();
    }
}

/// Used to annihilate the infinite number of asset spawns that are occurring each frame.
pub fn despawn_countdown_digits(
    mut commands: Commands,
    digit_query: Query<Entity, With<Chronodigit>>,
) {
    for digit in digit_query.iter() {
        commands.entity(digit).despawn();
    }
}

/// Used to close UI panels that have all of their elements associated with a specified container.
/// If a UI component has a specific container type attached to it then you can delete it by using
/// this function.
pub fn despawn_container(
    commands: &mut Commands,
    container: Containers,
    container_query: &Query<(Entity, &Containers)>,
)
{
    for (entity, ui_container) in container_query.iter() {
        if *ui_container == container {
            commands.entity(entity).despawn();
        }
    }
}

/// This will resize text to always be relative to a text's set scaling factor and the
/// window's current width.  I say "current" since this system is running every frame but its
/// code will only trigger when the window gets resized.  This will also resize UI elements
/// according to the window width and the aspect ratio of the element's Sizer component.
pub fn resize(
    window_query: Query<&Window>,
    mut text_query: Query<(&mut TextFont, &TextSpawn)>,
    mut node_query: Query<(&mut Node, &Sizer)>,
    mut resize_reader: MessageReader<WindowResized>,
) -> Result<()>
{
    for _ in resize_reader.read() {

        let window = window_query.single()?;

        // Reposition and resize all UI elements.
        for (mut node, element) in node_query.iter_mut() {
            let width_px = window.width() * (element.size_of_element / 100.0);
            let height_px = width_px / element.aspect_ratio.unwrap_or(1.0);
            let height_half_size = (height_px / window.height()) * 100.0 / 2.0;
            let width_half_size = element.size_of_element / 2.0;

            node.left = Val::Percent(element.position.x - width_half_size);
            node.top  = Val::Percent(element.position.y - height_half_size);
            node.width = Val::Percent(element.size_of_element);
            node.height = Val::Px(height_px);
        }

        // Resize all text.
        for (mut text_font, text_spawn) in text_query.iter_mut() {
            text_font.font_size = window.width() * text_spawn.font_size_scale;
        }
    }

    Ok(())
}

pub fn resize_background(
    window_query: Query<&Window>,
    mut background_query: Query<(&mut Sprite, &Containers)>,
    mut resize_reader: MessageReader<WindowResized>,
) {
    for _ in resize_reader.read() {

        let Ok(window) = window_query.single()
        else { return; };

        let aspect_ratio = window.width() / window.height();
        let world_height = 1080.0 * 1.3;
        let world_width = world_height * aspect_ratio;

        for (mut sprite, container) in background_query.iter_mut() {
            if *container == Containers::Background {
                sprite.custom_size = Some(Vec2::new(world_width, world_height));
            }
        }
    }
}

/// Buttons are programmed out based on enum type and will direct state transitions and trigger confirmation dialogs where appropriate.
pub fn handle_button_interactions(
    asset_server: Res<AssetServer>,
    window_query: Query<&Window>,
    container_query: Query<(Entity, &Containers)>,
    interaction_query: Query<(&Interaction, &Buttons), Changed<Interaction>>,
    images: Res<InterfaceImages>,
    mut commands: Commands,
    mut button_chain: ResMut<ButtonChain>,
    mut next_state: ResMut<NextState<Interfaces>>,
    mut state_history: ResMut<StateHistory>,
    mut app_exit: MessageWriter<AppExit>,
    mut the_timer: ResMut<TheTimer>,
) -> Result<()>
{
    for (interaction, button) in interaction_query.iter() {
        println!("Button clicked: {:?}", button);
        if *interaction == Interaction::Pressed {

            match (button_chain.as_slice(), button) {

                ([Buttons::StartMenu], Buttons::Yes) => {
                    button_chain.clear();
                    state_history.clear();
                    despawn_container(&mut commands, Containers::Confirmation, &container_query);
                    next_state.set(Interfaces::StartMenu);
                },

                ([Buttons::ExitGame], Buttons::Yes) => {
                    button_chain.clear();
                    despawn_container(&mut commands, Containers::Confirmation, &container_query);
                    app_exit.write(AppExit::Success);
                },

                ([], Buttons::StartMenu) => {
                    let window = window_query.single()?;
                    spawn_confirmation(&mut commands, &asset_server, &window, &images, "Are you sure you wish to navigate to the Start Menu?");
                    button_chain.push(Buttons::StartMenu);
                },

                ([], Buttons::ExitGame) => {
                    let window = window_query.single()?;
                    spawn_confirmation(&mut commands, &asset_server, &window, &images, "Close the program and exit the game?");
                    button_chain.push(Buttons::ExitGame);
                },

                (_, Buttons::No) => {
                    button_chain.clear();
                    despawn_container(&mut commands, Containers::Confirmation, &container_query);
                },

                (_, Buttons::Play) => {
                    button_chain.clear();
                    next_state.set(Interfaces::Level1);
                    the_timer.chronolog.reset();
                    the_timer.chronolog.start_value = Some(111.0);
                },

                (_, Buttons::GoToLevel1) => {
                    button_chain.clear();
                    next_state.set(Interfaces::Level1);
                },

                (_, Buttons::GoToLevel2) => {
                    button_chain.clear();
                    next_state.set(Interfaces::Level2);
                },

                (_, Buttons::GoToLevel3) => {
                    button_chain.clear();
                    next_state.set(Interfaces::Level3);
                },

                (_, Buttons::GoToLevel4) => {
                    button_chain.clear();
                    next_state.set(Interfaces::Level4);
                },

                (_, Buttons::GoToLevel5) => {
                    button_chain.clear();
                    next_state.set(Interfaces::Level5);
                },

                _ => { button_chain.clear(); }
            }
        }
    }

    Ok(())
}
