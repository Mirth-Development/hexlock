
// Imports
use bevy::prelude::*;

// Plugin
pub struct DefinitionsForUserInterface {}
impl Plugin for DefinitionsForUserInterface {
    fn build(&self, app: &mut App) {

        // States
        app.init_state::<Interfaces>();
        app.register_type::<Interfaces>();

        // Resources
        app.init_resource::<ButtonChain>();
        app.init_resource::<StateHistory>();
        app.init_resource::<InterfaceImages>();
        app.register_type::<ButtonChain>();
        app.register_type::<StateHistory>();

        // Components
        app.register_type::<ButtonChain>();
        app.register_type::<StateHistory>();
        app.register_type::<Containers>();
        app.register_type::<Buttons>();
        app.register_type::<Labels>();
        app.register_type::<TextSpawn>();
        app.register_type::<Sizer>();
    }
}



// ---------------------------------------------------------------------------------------------- //
// STATES

#[derive(Reflect, Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum Interfaces {
    #[default]
    StartMenu,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Cards,
    Won,
    Lost
}
// ---------------------------------------------------------------------------------------------- //



// ---------------------------------------------------------------------------------------------- //
// COMPONENTS

/// Using this as a way to mark combo arrow spawns so that they can be deleted later.
#[derive(Component, Reflect)]
pub struct ComboArrow;

#[derive(Component, Reflect, PartialEq)]
#[reflect(Component)]
pub enum Containers {
    Confirmation,
    Background,
    Card,
    Timer,
    Combo
}

#[derive(Component, Debug, Reflect, PartialEq, Clone)]
#[reflect(Component)]
pub enum Buttons {
    Play,
    ExitGame,
    StartMenu,
    GoToLevel1,
    GoToLevel2,
    GoToLevel3,
    GoToLevel4,
    GoToLevel5,
    CardTimerIncrease,
    CardSetTimeIncrease,
    Yes,
    No,
}

#[derive(Component, Reflect, PartialEq)]
#[reflect(Component)]
pub enum Labels {
    Title,
    Confirmation,
    Card,
    Level,
}

// This component is always built into other elements - or at least it should be, using it
// on its own will make cleanup features not work appropriately.  Text made from this element
// will be deleted when its corresponding parent is deleted.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TextSpawn {
    pub content: &'static str,      // content is always known at compile time, hence static lifetime.
    pub font_path: &'static str,    // font_path is always known at compile time, hence static lifetime.
    pub font_size_scale: f32,       // font_size_scale uses the window width as it's factor (use values below 1.0).
    pub color: Color,
}

// This is the component that is configured when the window size is changed (or if some function
// wants to use it to change the size of UI elements at runtime).  Position is part of this since
// the pixel position of something is changed when the window gets messed with.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Sizer {
    pub position: Vec3,
    pub size_of_element: f32,
    pub aspect_ratio: Option<f32>,
}
// ---------------------------------------------------------------------------------------------- //



// ---------------------------------------------------------------------------------------------- //
// RESOURCES

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct InterfaceImages {
    pub digit_zero: Handle<Image>,
    pub digit_one: Handle<Image>,
    pub digit_two: Handle<Image>,
    pub digit_three: Handle<Image>,
    pub digit_four: Handle<Image>,
    pub digit_five: Handle<Image>,
    pub digit_six: Handle<Image>,
    pub digit_seven: Handle<Image>,
    pub digit_eight: Handle<Image>,
    pub digit_nine: Handle<Image>,
    pub background_panel: Handle<Image>,
    pub background_level: Handle<Image>,
    pub background_start: Handle<Image>,
    pub arrow_up: Handle<Image>,
    pub arrow_down: Handle<Image>,
    pub arrow_left: Handle<Image>,
    pub arrow_right: Handle<Image>,
    pub card_increase_time: Handle<Image>,
    pub card_increase_set_time: Handle<Image>,
    pub button: Handle<Image>,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct ButtonChain {
    chain: Vec<Buttons>,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct StateHistory {
    stack: Vec<Interfaces>,
}

// Have to forcibly shove InterfaceImages into the world the gnarly way since the interface states
// get loaded before everything due to the Start Menu being the first thing required for the game
// to appear.
impl FromWorld for InterfaceImages {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            digit_zero:             asset_server.load("images/0.png"),
            digit_one:              asset_server.load("images/1.png"),
            digit_two:              asset_server.load("images/2.png"),
            digit_three:            asset_server.load("images/3.png"),
            digit_four:             asset_server.load("images/4.png"),
            digit_five:             asset_server.load("images/5.png"),
            digit_six:              asset_server.load("images/6.png"),
            digit_seven:            asset_server.load("images/7.png"),
            digit_eight:            asset_server.load("images/8.png"),
            digit_nine:             asset_server.load("images/9.png"),
            background_panel:       asset_server.load("images/Background_for_Panel.png"),
            background_start:       asset_server.load("images/Background_for_Start.png"),
            background_level:       asset_server.load("images/Background_for_Level.png"),
            arrow_up:               asset_server.load("images/Direction_Up.png"),
            arrow_down:             asset_server.load("images/Direction_Down.png"),
            arrow_left:             asset_server.load("images/Direction_Left.png"),
            arrow_right:            asset_server.load("images/Direction_Right.png"),
            card_increase_time:     asset_server.load("images/Direction_Left.png"),
            card_increase_set_time: asset_server.load("images/Direction_Right.png"),
            button:                 asset_server.load("images/Button.png"),
        }
    }
}

impl ButtonChain {

    // Add button to the chain.
    pub fn push(&mut self, button: Buttons) {
        self.chain.push(button);
    }

    // Clear the entire chain.
    pub fn clear(&mut self) {
        self.chain.clear();
    }

    // Return the chain as a slice so that it can be utilized in match statements.
    pub fn as_slice(&self) -> &[Buttons] {
        self.chain.as_slice()
    }
}

impl StateHistory {

    const HISTORY_CAP: usize = 10;

    // Adding to history.  Will remove oldest state in history when cap has been reached to allow
    // for new additions to the state history.
    pub fn push(&mut self, state: Interfaces) {
        if self.stack.len() >= Self::HISTORY_CAP {
            self.stack.remove(0);
        }
        self.stack.push(state);
    }

    // Removing/getting latest history.
    // Because we're working with a stack we must return an option for the scenario that the stack
    // could be empty.  Realistically speaking, I don't think this would ever happen since players
    // start on the main menu and always move into another UI.
    pub fn pop(&mut self) -> Option<Interfaces> {
        self.stack.pop()
    }

    // Wiping the full UI state history.
    pub fn clear(&mut self) {
        self.stack.clear();
    }
}
// ---------------------------------------------------------------------------------------------- //
