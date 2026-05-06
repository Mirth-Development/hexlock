use bevy::prelude::*;

#[derive(Message)]
pub enum GameStateMessage{
    Win,
    Lose
}