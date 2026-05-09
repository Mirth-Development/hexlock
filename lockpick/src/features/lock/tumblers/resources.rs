use bevy::prelude::*;

#[derive(Resource, Copy, Clone, PartialEq)]
pub enum TumblerType{
    Normal,
    Electric,
    Magic
}

#[derive(Resource, Copy, Clone)]
pub enum TumblerSize{
    Small,
    Medium,
    Large
}

# [derive(Copy, Clone, Debug)]
pub enum Directions{
    Up,
    Down,
    Left,
    Right,
}

#[derive(Resource)]
pub struct TumblerTime {
    pub set_time: f32,
}

impl Default for TumblerTime {
    fn default() -> Self {
        Self {
            set_time: 20.0,
        }
    }
}
