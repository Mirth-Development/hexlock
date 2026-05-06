
// Imports
use bevy::prelude::*;

// Plugin
pub struct DefinitionsForGameTimer {}
impl Plugin for DefinitionsForGameTimer {
    fn build(&self, app: &mut App) {
        app.register_type::<Ticker>();
        app.register_type::<Chronolog>();
    }
}

// #################################################################################################### //
// TICKER DEFINITION

/// By themselves, tickers can be used to create simple timers.  Although they are best used in conjunction
/// with a Chronolog to create some wicked tickety-tocking.
///
/// Non-chronolog tickers are only capable of the range of 0 to 9.  But with a repeating timer, good maffffs,
/// and an accumulator that's paired with a condition or two then you can go up to or below any number you want
/// with a non-chronolog ticker.
#[derive(Component, Reflect, Debug)]
pub struct Ticker {
    pub number: Option<u32>,
    pub timer: Option<Timer>,
}

impl Default for Ticker {
    fn default() -> Self {
        Self {
            number: None,
            timer: None,
        }
    }
}

impl Ticker {
    pub fn tick_tock(&mut self, delta: std::time::Duration) {
        if let (Some(timer), Some(number)) = (&mut self.timer, &mut self.number) {

            // Advance timer by the difference in time between frames.
            timer.tick(delta);

            // This and the if-condition is handling frame spiking.
            let ticks = timer.times_finished_this_tick();
            if ticks > 0 {
                // Don't get rid of my modulo!  It's what's allowing digits to be processed correctly
                // inside chronologs.  Tickers a
                *number = (*number + ticks) % 10;
            }
        }
    }
}

// #################################################################################################### //



// #################################################################################################### //
// CHRONOLOG DEFINITION

/// Used to create timers that can optionally store digits from the hundreds place to the thousandths
/// place.  Digits can be preset by assigning values to digit properties and timers for each digit
/// can be assigned independently for all the insanity that comes with declaring fancy clocks.
///
/// Digits are declared with the datatype u32 and not u8 because the tick system is set up
/// to handle frame spikes.  Frame spikes with timers inside Bevy can be dealt with using the times_finished_this_tick()
/// method on a timer and counting the number of ticks that occurred during the time it took to pass through the
/// delta happening mid-spike.  I'm being a bit insane by applying them to all digits.  Realistically speaking,
/// applying u32 to the thousandth and hundredth place is absurd over going with the just_finished() option that Bevy provides on timers
/// (nobody is gonna have a 10+ or 100+ second frame spike).  But uh...  CODE SYMMETRY (AKA OCD) CALLS FOR INEFFICIENCY!
///
/// By default, Tickers within a Chronolog have nothing in them.
/// By using the new method, Tickers will count up and repeat once they hit their max value.
/// You can still do countdown logic with tickers moving up in time, just takes a little more work.  But
/// if you'd like you can give a default Ticker custom timers to do countdown effects more intuitively.
#[derive(Component, Reflect, Debug)]
pub struct Chronolog {
    pub ticker_for_hundred: Option<Ticker>,
    pub ticker_for_ten: Option<Ticker>,
    pub ticker_for_one: Option<Ticker>,
    pub ticker_for_tenth: Option<Ticker>,
    pub ticker_for_hundredth: Option<Ticker>,
    pub ticker_for_thousandth: Option<Ticker>,
}

impl Default for Chronolog {
    fn default() -> Self {
        Self {
            ticker_for_hundred: Some(Ticker::default()),
            ticker_for_ten: Some(Ticker::default()),
            ticker_for_one: Some(Ticker::default()),
            ticker_for_tenth: Some(Ticker::default()),
            ticker_for_hundredth: Some(Ticker::default()),
            ticker_for_thousandth: Some(Ticker::default()),
        }
    }
}

impl Chronolog {
    pub fn new() -> Self {
        Self {
            ticker_for_hundred: Some(Ticker{
                number: Some(0),
                timer: Some(Timer::from_seconds(100.0, TimerMode::Repeating)),
            }),

            ticker_for_ten: Some(Ticker{
                number: Some(0),
                timer: Some(Timer::from_seconds(10.0, TimerMode::Repeating)),
            }),

            ticker_for_one: Some(Ticker{
                number: Some(0),
                timer: Some(Timer::from_seconds(1.0, TimerMode::Repeating)),
            }),

            ticker_for_tenth: Some(Ticker{
                number: Some(0),
                timer: Some(Timer::from_seconds(0.1, TimerMode::Repeating)),
            }),

            ticker_for_hundredth: Some(Ticker{
                number: Some(0),
                timer: Some(Timer::from_seconds(0.01, TimerMode::Repeating)),
            }),

            ticker_for_thousandth: Some(Ticker{
                number: Some(0),
                timer: Some(Timer::from_seconds(0.001, TimerMode::Repeating)),
            }),
        }
    }

    pub fn display(&self) -> String {

        // WHY I LOVE AND HATE RUST
        // 1. Borrow by using as_ref().
        // 2. Get rid of double Option results using and_then(#Anonymous).
        // 3. Get what's in number or set to 0 if nothing is there using unwrap_or(0).
        let hundreds    = self.ticker_for_hundred.as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        let tens        = self.ticker_for_ten.as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        let ones        = self.ticker_for_one.as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        let tenths      = self.ticker_for_tenth.as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        let hundredths  = self.ticker_for_hundredth.as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        let thousandths = self.ticker_for_thousandth.as_ref().and_then(|ticker| ticker.number).unwrap_or(0);

        format!("{}{}{}.{}{}{}", hundreds, tens, ones, tenths, hundredths, thousandths)
    }
}
// #################################################################################################### //
