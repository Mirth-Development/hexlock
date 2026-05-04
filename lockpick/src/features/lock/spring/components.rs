use bevy::prelude::Component;

//Naming Component
#[derive(Component)]
pub struct SpringComponent{
    pub position: u32
}

impl Default for SpringComponent{
    fn default() -> Self {
        Self {
            position: 0
        }
    }
}
