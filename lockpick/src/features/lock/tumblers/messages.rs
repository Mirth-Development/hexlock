use bevy::prelude::*;


//send message to timer
#[derive(Message)]
pub enum HitWrongTumbler{ReduceTime}

#[derive(Message)]
pub struct TumblerTimerMessage(pub Entity); //Position