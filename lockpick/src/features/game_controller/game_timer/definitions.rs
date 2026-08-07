
// Imports
use bevy::prelude::*;

// Plugin
pub struct DefinitionsForGameTimer {}
impl Plugin for DefinitionsForGameTimer {
    fn build(&self, app: &mut App) {

        // Components
        app.register_type::<Ticker>();
        app.register_type::<Chronolog>();
        app.register_type::<Chronodigit>();

        // Resources
        app.init_resource::<TheTimer>();
    }
}



// #################################################################################################### //
// CHRONODIGIT DEFINITION

/// Using this as a way to mark digit spawns so that they can be deleted later.
#[derive(Component, Reflect)]
pub struct Chronodigit;
// #################################################################################################### //



// #################################################################################################### //
// GAME TIMER DEFINITION
#[derive(Resource)]
pub struct TheTimer {
    pub chronolog: Chronolog,
}

impl Default for TheTimer {
    fn default() -> Self {
        Self {
            chronolog: Chronolog::new(Some(125.0)),
        }
    }
}
// #################################################################################################### //



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
    pub fn tick(&mut self, delta: std::time::Duration) {
        if let (Some(timer), Some(number)) = (&mut self.timer, &mut self.number) {

            // Advance timer by the difference in time between frames.
            // This .tick is Bevy's tick method for their timers, this isn't a recursive action.
            timer.tick(delta);

            // Handling frame spiking.
            let ticks = timer.times_finished_this_tick();
            if ticks > 0 {
                // Don't get rid of my modulo!  It's what's allowing digits to be processed correctly
                // inside chronologs.  The "number" property only goes up to 9 intentionally to properly
                // maintain digits inside chronologs.
                *number = (*number + ticks) % 10;
            }
        }
    }

    /// Will return the current value of the number stored in Ticker.
    pub fn get_number(&self) -> u32 {
        self.number.unwrap_or(0)
    }

    /// Will return the current value of the number stored in Ticker as a string.
    pub fn get_string(&self) -> String {
        format!("{}", self.number.unwrap_or(0))
    }

    /// Pauses a timer within the ticker.
    pub fn pause(&mut self) {
        if let Some(timer) = &mut self.timer {
            // This .pause is Bevy's pause method for their timers, this isn't a recursive action.
            timer.pause();
        }
    }

    /// Unpauses a timer within a ticker.
    pub fn unpause(&mut self) {
        if let Some(timer) = &mut self.timer {
            // This .unpause is Bevy's unpause method for their timers, this isn't a recursive action.
            timer.unpause();
        }
    }
}
// #################################################################################################### //



// #################################################################################################### //
// CHRONOLOG DEFINITION

#[derive(PartialEq, Reflect, Debug)]
enum ChronologStates {
    Dormant,
    Paused,
    Ticking
}

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
    start_value: Option<f32>,
    countdown_value: f32,
    state_of_log: ChronologStates,
    pub ticker_for_hundreds: Option<Ticker>,
    pub ticker_for_tens: Option<Ticker>,
    pub ticker_for_ones: Option<Ticker>,
    pub ticker_for_tenths: Option<Ticker>,
    pub ticker_for_hundredths: Option<Ticker>,
    pub ticker_for_thousandths: Option<Ticker>,
}

impl Default for Chronolog {
    fn default() -> Self {
        Self {
            start_value: Some(0.0),
            countdown_value: 0.0,
            state_of_log: ChronologStates::Dormant,
            ticker_for_hundreds: Some(Ticker::default()),
            ticker_for_tens: Some(Ticker::default()),
            ticker_for_ones: Some(Ticker::default()),
            ticker_for_tenths: Some(Ticker::default()),
            ticker_for_hundredths: Some(Ticker::default()),
            ticker_for_thousandths: Some(Ticker::default()),
        }
    }
}

impl Chronolog {

    /// Requires an Option to be thrown into it for usage, the Option may be filled or None.
    ///
    /// Passing in None will set the start_value to 0.0.
    /// Passing in Some(INSERT_FLOATING_POINT_HERE) will set the start_value to INSERT_FLOATING_POINT_HERE.
    ///
    /// Does not work with negatives, so don't even try unless you'd like to cry like I have.
    pub fn new(starting_value: Option<f32>) -> Self {
        Self {

            start_value: Some(starting_value.unwrap_or(0.0)),

            countdown_value: starting_value.unwrap_or(0.0),

            state_of_log: ChronologStates::Ticking,

            ticker_for_hundreds: Some(Ticker{
                number: Some(0),
                timer: Some(Timer::from_seconds(100.0, TimerMode::Repeating)),
            }),

            ticker_for_tens: Some(Ticker{
                number: Some(0),
                timer: Some(Timer::from_seconds(10.0, TimerMode::Repeating)),
            }),

            ticker_for_ones: Some(Ticker{
                number: Some(0),
                timer: Some(Timer::from_seconds(1.0, TimerMode::Repeating)),
            }),

            ticker_for_tenths: Some(Ticker{
                number: Some(0),
                timer: Some(Timer::from_seconds(0.1, TimerMode::Repeating)),
            }),

            ticker_for_hundredths: Some(Ticker{
                number: Some(0),
                timer: Some(Timer::from_seconds(0.01, TimerMode::Repeating)),
            }),

            ticker_for_thousandths: Some(Ticker{
                number: Some(0),
                timer: Some(Timer::from_seconds(0.001, TimerMode::Repeating)),
            }),
        }
    }

    /// This type of reset will cause for the Chronolog to continue ticking immediately after reset
    /// in the same way that a Chronolog will tick when created using the "new" method.  It will cause
    /// a Chronolog to start off with the start_value it was initially assigned with.
    pub fn reset(&mut self) {
        *self = Chronolog::new(self.start_value);
    }

    /// Will wipe out all the tickers in the Chronolog.  Can be used to create a blank slate to add new
    /// tickers onto a Chronolog if you want to.
    pub fn blank(&mut self) {
        *self = Chronolog::default();
    }

    /// Pauses all tickers within the Chronolog.
    pub fn pause(&mut self) {
        if let Some(ticker) = &mut self.ticker_for_hundreds    { ticker.pause(); }
        if let Some(ticker) = &mut self.ticker_for_tens        { ticker.pause(); }
        if let Some(ticker) = &mut self.ticker_for_ones        { ticker.pause(); }
        if let Some(ticker) = &mut self.ticker_for_tenths      { ticker.pause(); }
        if let Some(ticker) = &mut self.ticker_for_hundredths  { ticker.pause(); }
        if let Some(ticker) = &mut self.ticker_for_thousandths { ticker.pause(); }

        self.state_of_log = ChronologStates::Paused;
    }

    /// Unpauses all tickers within the Chronolog.
    pub fn unpause(&mut self) {
        if let Some(ticker) = &mut self.ticker_for_hundreds    { ticker.unpause(); }
        if let Some(ticker) = &mut self.ticker_for_tens        { ticker.unpause(); }
        if let Some(ticker) = &mut self.ticker_for_ones        { ticker.unpause(); }
        if let Some(ticker) = &mut self.ticker_for_tenths      { ticker.unpause(); }
        if let Some(ticker) = &mut self.ticker_for_hundredths  { ticker.unpause(); }
        if let Some(ticker) = &mut self.ticker_for_thousandths { ticker.unpause(); }

        self.state_of_log = ChronologStates::Ticking;
    }

    /// This is for an update on frame system to adjust countdown_value with each frame's passing.
    /// Will only update if the chronolog is in the Ticking state.
    ///
    /// Not calling this inside an update system will prevent the countdown_value from ever moving.
    pub fn update_countdown(&mut self, delta: std::time::Duration) {

        if self.state_of_log == ChronologStates::Ticking {

            self.countdown_value -= delta.as_secs_f32();

            if self.countdown_value < 0.0 {
                self.countdown_value = 0.0;
            }
        }
    }

    /// Adds time in seconds to the countdown value.
    pub fn add_to_countdown(&mut self, seconds: f32) {
        self.countdown_value += seconds;
    }

    /// Returns the start value of the Chronolog.
    /// Will return 0.0 if no start value is set.
    pub fn get_start_value(&self) -> f32 {
        self.start_value.unwrap_or(0.0)
    }

    /// Returns a string for the current countdown value, the number of digits is based on how many
    /// whole places is desired and how many floating places is desired.
    ///
    /// It's important to note that a decimal is added into the string and should be accounted for
    /// if you're gonna try and convert the string into an indexable structure.
    pub fn get_countdown_string(
        &self,
        number_of_whole_places: usize,
        number_of_floating_places: usize
    ) -> String {

        let character_count = number_of_whole_places + number_of_floating_places + 1;

        // Left side of printout is dictated implicitly by (number_of_characters - floating).
        format!("{:0>number_of_characters$.floating$}",
                self.countdown_value,
                number_of_characters = character_count,
                floating = number_of_floating_places
        )
    }

    /// Returns the number that's in the hundreds' ticker.  Will return 0 if there is no ticker.
    ///
    /// Something to keep in mind is that a ticker could exist for the hundreds place and also have
    /// a value of 0 for its number.  This means that 0 doesn't ALWAYS mean that a ticker doesn't exist for
    /// the given digit.
    pub fn get_hundreds_digit(&self) -> u32 {
        self.ticker_for_hundreds.as_ref().and_then(|ticker| ticker.number).unwrap_or(0)
    }

    /// Returns the number that's in the tens' ticker.  Will return 0 if there is no ticker.
    ///
    /// Something to keep in mind is that a ticker could exist for the tens place and also have
    /// a value of 0 for its number.  This means that 0 doesn't ALWAYS mean that a ticker doesn't exist for
    /// the given digit.
    pub fn get_tens_digit(&self) -> u32 {
        self.ticker_for_tens.as_ref().and_then(|ticker| ticker.number).unwrap_or(0)
    }

    /// Returns the number that's in the ones' ticker.  Will return 0 if there is no ticker.
    ///
    /// Something to keep in mind is that a ticker could exist for the ones place and also have
    /// a value of 0 for its number.  This means that 0 doesn't ALWAYS mean that a ticker doesn't exist for
    /// the given digit.
    pub fn get_ones_digit(&self) -> u32 {
        self.ticker_for_ones.as_ref().and_then(|ticker| ticker.number).unwrap_or(0)
    }

    /// Returns the number that's in the tenths' ticker.  Will return 0 if there is no ticker.
    ///
    /// Something to keep in mind is that a ticker could exist for the tenths place and also have
    /// a value of 0 for its number.  This means that 0 doesn't ALWAYS mean that a ticker doesn't exist for
    /// the given digit.
    pub fn get_tenths_digit(&self) -> u32 {
        self.ticker_for_tenths.as_ref().and_then(|ticker| ticker.number).unwrap_or(0)
    }

    /// Returns the number that's in the hundredths' ticker.  Will return 0 if there is no ticker.
    ///
    /// Something to keep in mind is that a ticker could exist for the hundredths place and also have
    /// a value of 0 for its number.  This means that 0 doesn't ALWAYS mean that a ticker doesn't exist for
    /// the given digit.
    pub fn get_hundredths_digit(&self) -> u32 {
        self.ticker_for_hundredths.as_ref().and_then(|ticker| ticker.number).unwrap_or(0)
    }

    /// Returns the number that's in the thousandths' ticker.  Will return 0 if there is no ticker.
    ///
    /// Something to keep in mind is that a ticker could exist for the thousandths place and also have
    /// a value of 0 for its number.  This means that 0 doesn't ALWAYS mean that a ticker doesn't exist for
    /// the given digit.
    pub fn get_thousandths_digit(&self) -> u32 {
        self.ticker_for_thousandths.as_ref().and_then(|ticker| ticker.number).unwrap_or(0)
    }

    /// Returns the number that's in the hundreds' ticker as a string.  Will return 0 as a string if there is no ticker.
    ///
    /// Something to keep in mind is that a ticker could exist for the hundreds place and also have
    /// a value of 0 for its number.  This means that 0 doesn't ALWAYS mean that a ticker doesn't exist for
    /// the given digit.
    pub fn string_for_hundreds(&self) -> String {
        let hundreds = self.ticker_for_hundreds.as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        format!("{}", hundreds)
    }

    /// Returns the number that's in the tens' ticker as a string.  Will return 0 as a string if there is no ticker.
    ///
    /// Something to keep in mind is that a ticker could exist for the tens place and also have
    /// a value of 0 for its number.  This means that 0 doesn't ALWAYS mean that a ticker doesn't exist for
    /// the given digit.
    pub fn string_for_tens(&self) -> String {
        let tens = self.ticker_for_tens.as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        format!("{}", tens)
    }

    /// Returns the number that's in the ones' ticker as a string.  Will return 0 as a string if there is no ticker.
    ///
    /// Something to keep in mind is that a ticker could exist for the ones place and also have
    /// a value of 0 for its number.  This means that 0 doesn't ALWAYS mean that a ticker doesn't exist for
    /// the given digit.
    pub fn string_for_ones(&self) -> String {
        let ones = self.ticker_for_ones.as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        format!("{}", ones)
    }

    /// Returns the number that's in the tenths' ticker as a string.  Will return 0 as a string if there is no ticker.
    ///
    /// Something to keep in mind is that a ticker could exist for the tenths place and also have
    /// a value of 0 for its number.  This means that 0 doesn't ALWAYS mean that a ticker doesn't exist for
    /// the given digit.
    pub fn string_for_tenths(&self) -> String {
        let tenths = self.ticker_for_tenths.as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        format!("{}", tenths)
    }

    /// Returns the number that's in the hundredths' ticker as a string.  Will return 0 as a string if there is no ticker.
    ///
    /// Something to keep in mind is that a ticker could exist for the hundredths place and also have
    /// a value of 0 for its number.  This means that 0 doesn't ALWAYS mean that a ticker doesn't exist for
    /// the given digit.
    pub fn string_for_hundredths(&self) -> String {
        let hundredths = self.ticker_for_hundredths.as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        format!("{}", hundredths)
    }

    /// Returns the number that's in the thousandths' ticker as a string.  Will return 0 as a string if there is no ticker.
    ///
    /// Something to keep in mind is that a ticker could exist for the thousandths place and also have
    /// a value of 0 for its number.  This means that 0 doesn't ALWAYS mean that a ticker doesn't exist for
    /// the given digit.
    pub fn string_for_thousandths(&self) -> String {
        let thousandths = self.ticker_for_thousandths.as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        format!("{}", thousandths)
    }

    /// Will return the current value of the Chronolog.  Any unused digits will be labeled as 0.
    pub fn get_number(&self) -> f32 {

        let mut total_time: f32 = 0.0;

        // WHY I LOVE AND HATE RUST
        // 1. Borrow by using as_ref().
        // 2. Get rid of double Option results using and_then(#Anonymous).
        // 3. Get what's in number or set to 0 if nothing is there using unwrap_or(0).
        // 4. Typecast to f32 since tickers store u32 and the end result we're going for is a decimal.
        let hundreds    = (self.ticker_for_hundreds      .as_ref().and_then(|ticker| ticker.number).unwrap_or(0)) as f32 * 100.0;
        let tens        = (self.ticker_for_tens          .as_ref().and_then(|ticker| ticker.number).unwrap_or(0)) as f32 * 10.0;
        let ones        = (self.ticker_for_ones          .as_ref().and_then(|ticker| ticker.number).unwrap_or(0)) as f32 * 1.0;
        let tenths      = (self.ticker_for_tenths        .as_ref().and_then(|ticker| ticker.number).unwrap_or(0)) as f32 * 0.1;
        let hundredths  = (self.ticker_for_hundredths    .as_ref().and_then(|ticker| ticker.number).unwrap_or(0)) as f32 * 0.01;
        let thousandths = (self.ticker_for_thousandths   .as_ref().and_then(|ticker| ticker.number).unwrap_or(0)) as f32 * 0.001;

        total_time += hundreds;
        total_time += tens;
        total_time += ones;
        total_time += tenths;
        total_time += hundredths;
        total_time += thousandths;

        total_time
    }

    /// Will return the current value of the Chronolog as a string.  Any unused digits will be labeled as 0.
    pub fn get_string(&self) -> String {

        // WHY I LOVE AND HATE RUST
        // 1. Borrow by using as_ref().
        // 2. Get rid of double Option results using and_then(#Anonymous).
        // 3. Get what's in number or set to 0 if nothing is there using unwrap_or(0).
        let hundreds    = self.ticker_for_hundreds      .as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        let tens        = self.ticker_for_tens          .as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        let ones        = self.ticker_for_ones          .as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        let tenths      = self.ticker_for_tenths        .as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        let hundredths  = self.ticker_for_hundredths    .as_ref().and_then(|ticker| ticker.number).unwrap_or(0);
        let thousandths = self.ticker_for_thousandths   .as_ref().and_then(|ticker| ticker.number).unwrap_or(0);

        format!("{}{}{}.{}{}{}", hundreds, tens, ones, tenths, hundredths, thousandths)
    }
}
// #################################################################################################### //
