use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerAttributes{
    //Effects the player can choose at the end of each round
    //Bools should be checked only once
    //World effects
    additional_time: f32,
    additional_threshold: f32,
    see_threshold: bool, //Get a line that shows where the tumbler has to go past

    //Electric Pick
    charge_speed: f32,
    stun_duration: f32,

    //Magic Pick
    slow_duration: f32,
    spell_reduction: bool,

    //General Pick?
    picking_speed: f32,
    tumbler_additional_set_time: f32,

}