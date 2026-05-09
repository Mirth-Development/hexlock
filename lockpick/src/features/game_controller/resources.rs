use bevy::prelude::*;
use crate::features::lock::tumblers::resources::Directions;

#[derive(Resource)]
pub struct GameResourceHandles {
    pub charge_bar: Handle<Image>,
    pub charge: Handle<Image>,
}

#[derive(Resource)]
pub struct NumberOfTumblers {
    pub number_of_tumblers: u32,
}

#[derive(Resource)]
pub struct TumblerOrdering {
    pub current_position: u32,
    pub order: Vec<u32>
}

# [derive(Resource)]
pub struct InputtedArrowCode{
    pub inputting : bool,
    pub entered_code: Vec<Directions>,
}


#[derive(Resource)]
pub struct PlayerAttributes{
    //Effects the player can choose at the end of each round
    //Bools should be checked only once
    //World effects
    pub additional_time: f32,
    pub additional_threshold: f32,
    pub see_threshold: bool, //Get a line that shows where the tumbler has to go past

    //Electric Pick
    pub charge_speed: f32,
    pub stun_duration: f32,

    //Magic Pick
    pub slow_duration: f32,
    pub spell_reduction: bool,

    //General Pick?
    pub picking_speed: f32,
    pub tumbler_additional_set_time: f32,

}