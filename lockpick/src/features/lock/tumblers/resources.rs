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
