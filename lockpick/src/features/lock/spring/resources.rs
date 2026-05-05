use bevy::prelude::Resource;
#[derive(Resource, Copy, Clone)]
pub enum SpringSize{
    Thin,
    Regular,
    Thick
}