use bevy::prelude::*;

//Naming Component
#[derive(Component)]
pub struct TumblerComponent{
    pub position: u32,
    pub velocity: Vec3,
}

#[derive(Component)]
pub struct FocusedTumblerComponent;



impl Default for TumblerComponent {
    fn default() -> Self {
        Self {
            position: 0,
            velocity: Vec3::splat(0.0)
        }
    }
}



