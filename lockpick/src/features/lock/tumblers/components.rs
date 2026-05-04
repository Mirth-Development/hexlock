use bevy::prelude::*;

//Naming Component
#[derive(Component)]
pub struct TumblerComponent{
    pub position: u32,
    pub velocity: Vec3,
}

#[derive(Component)]
pub struct FocusedTumblerComponent;

#[derive(Component)]
pub struct SetTumblerComponent;


impl Default for TumblerComponent {
    fn default() -> Self {
        Self {
            position: 0,
            velocity: Vec3::splat(0.0),
            //set: false //Handle this with a component
        }
    }
}



