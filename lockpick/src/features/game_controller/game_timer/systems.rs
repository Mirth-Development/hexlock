
// Imports
use bevy::prelude::*;
use crate::features::game_controller::game_timer::definitions::*;

pub struct SystemsForGameTimer {}
impl Plugin for SystemsForGameTimer {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (chronolog_ticking, ticker_ticking));
    }
}

/// Will loop through queried chronologs to see if they have tickers within them that need to be tick-tocked.
pub fn chronolog_ticking(
    time: Res<Time>,
    mut logs: Query<&mut Chronolog>,
) {

    let delta = time.delta();

    for mut log in &mut logs {
        if let Some(ticker) = &mut log.ticker_for_hundred    { ticker.tick_tock(delta); }
        if let Some(ticker) = &mut log.ticker_for_ten        { ticker.tick_tock(delta); }
        if let Some(ticker) = &mut log.ticker_for_one        { ticker.tick_tock(delta); }
        if let Some(ticker) = &mut log.ticker_for_tenth      { ticker.tick_tock(delta); }
        if let Some(ticker) = &mut log.ticker_for_hundredth  { ticker.tick_tock(delta); }
        if let Some(ticker) = &mut log.ticker_for_thousandth { ticker.tick_tock(delta); }

        println!("{}", log.display());
    }
}

/// Will loop through queried tickers to initiate tick-tocking.  Chronolog tickers are bound to their chronolog
/// and IN THEORY should act as separate entity.  I think...?
pub fn ticker_ticking(
    time: Res<Time>,
    mut tickers: Query<&mut Ticker>,
) {

    let delta = time.delta();

    for mut ticker in &mut tickers {
        ticker.tick_tock(delta);
        println!("{}", ticker.number.unwrap_or(0));
    }
}
