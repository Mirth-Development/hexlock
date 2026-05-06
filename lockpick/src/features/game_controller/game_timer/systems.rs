
// Imports
use bevy::prelude::*;
use crate::features::game_controller::game_timer::definitions::*;

pub struct SystemsForGameTimer {}
impl Plugin for SystemsForGameTimer {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick);
    }
}

// Timer
fn tick(
    time: Res<Time>,
    mut tickers: Query<&mut Ticker>
)
{
    // for mut ticker in &mut tickers {
    //     ticker.timer.tick(time.delta());
    //     println!("{:?}", ticker.timer)
    // }
}
