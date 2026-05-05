
// Imports
use bevy::prelude::*;
use std::time::Duration;

pub struct Systems {}
impl Plugin for Systems {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick);
    }
}

// Timer
fn tick(
    time: Res<Time>,
    mut timers: Query<&mut Timer>
)
{
    for mut ticker in &mut timers {
        ticker.timer.tick(time.delta());
        println!("{:?}", ticker.timer)
    }
}
