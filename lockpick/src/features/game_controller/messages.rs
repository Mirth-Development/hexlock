use bevy::prelude::*;

#[derive(Message)]
///Message sent on completion of a "Level" to trigger progression to the next, or to the loss screen.
pub enum GameStateMessage{
    Win,
    Lose
}