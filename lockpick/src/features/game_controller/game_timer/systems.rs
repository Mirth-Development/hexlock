
// Imports
use bevy::prelude::*;
use crate::features::game_controller::game_timer::definitions::*;

pub struct SystemsForGameTimer {}
impl Plugin for SystemsForGameTimer {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (the_timer_ticking, chronolog_ticking, ticker_ticking));
    }
}

/// Resources can't be queried, which is basically the only reason why this exists (and because I can't think
/// of another way).  Would have preferred to have just used my chronolog_ticking for ALL chronologs...
pub fn the_timer_ticking(
    time: Res<Time>,
    mut the_timer: ResMut<TheTimer>,
) {

    let delta = time.delta();
    let log = &mut the_timer.chronolog;

    if let Some(ticker) = &mut log.ticker_for_hundreds    { ticker.tick(delta); }
    if let Some(ticker) = &mut log.ticker_for_tens        { ticker.tick(delta); }
    if let Some(ticker) = &mut log.ticker_for_ones        { ticker.tick(delta); }
    if let Some(ticker) = &mut log.ticker_for_tenths      { ticker.tick(delta); }
    if let Some(ticker) = &mut log.ticker_for_hundredths  { ticker.tick(delta); }
    if let Some(ticker) = &mut log.ticker_for_thousandths { ticker.tick(delta); }

    println!("{}", log.get_number());

}

/// Will loop through queried chronologs to see if they have tickers within them that need to be tick-tocked.
pub fn chronolog_ticking(
    time: Res<Time>,
    mut logs: Query<&mut Chronolog>,
) {

    let delta = time.delta();

    for mut log in &mut logs {
        if let Some(ticker) = &mut log.ticker_for_hundreds    { ticker.tick(delta); }
        if let Some(ticker) = &mut log.ticker_for_tens        { ticker.tick(delta); }
        if let Some(ticker) = &mut log.ticker_for_ones        { ticker.tick(delta); }
        if let Some(ticker) = &mut log.ticker_for_tenths      { ticker.tick(delta); }
        if let Some(ticker) = &mut log.ticker_for_hundredths  { ticker.tick(delta); }
        if let Some(ticker) = &mut log.ticker_for_thousandths { ticker.tick(delta); }

        println!("{}", log.get_number());
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
        ticker.tick(delta);
        println!("{}", ticker.get_number());
    }
}
