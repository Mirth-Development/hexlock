use bevy::prelude::*;


//send message to timer
#[derive(Message)]
pub enum HitWrongTumbler{ReduceTime}

#[derive(Message)]
pub enum TumblerTimerMessage{
    Stun,
    Finished
}