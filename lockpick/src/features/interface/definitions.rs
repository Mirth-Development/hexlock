
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

#[derive(Component, Reflect, PartialEq)]
#[reflect(Component)]
pub enum Containers {
    Confirmation,
    Card,
    Timer,
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
