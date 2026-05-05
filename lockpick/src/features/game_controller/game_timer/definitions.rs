
// Imports
use bevy::prelude::*;

// Plugin
pub struct Definitions {}
impl Plugin for Definitions {
    fn build(&self, app: &mut App) {

        // States


        // Resources


        // Components
        app.register_type::<Ticker>();

    }
}

#[derive(Component, Debug)]
pub struct Ticker {
    digit_for_hundred: Option<u8>,
    timer_for_hundred: Option<Timer>,

    digit_for_ten: Option<u8>,
    timer_for_ten: Option<Timer>,

    digit_for_one: Option<u8>,
    timer_for_one: Option<Timer>,

    digit_for_tenth: Option<u8>,
    timer_for_tenth: Option<Timer>,

    digit_for_hundredth: Option<u8>,
    timer_for_hundredth: Option<Timer>,
}

impl Default for Ticker {
    fn default() -> Self {
        Self {
            digit_for_hundred: None,
            timer_for_hundred: None,

            digit_for_ten: None,
            timer_for_ten: None,

            digit_for_one: None,
            timer_for_one: None,

            digit_for_tenth: None,
            timer_for_tenth: None,

            digit_for_hundredth: None,
            timer_for_hundredth: None,
        }
    }
}

impl Ticker {
    pub fn new() -> Self {
        Self {
            digit_for_hundred: Some(0),
            timer_for_hundred: Some(Timer::from_seconds(100.0, TimerMode::Repeating)),

            digit_for_ten: Some(0),
            timer_for_ten: Some(Timer::from_seconds(10.0, TimerMode::Repeating)),

            digit_for_one: Some(0),
            timer_for_one: Some(Timer::from_seconds(1.0, TimerMode::Repeating)),

            digit_for_tenth: Some(0),
            timer_for_tenth: Some(Timer::from_seconds(0.1, TimerMode::Repeating)),

            digit_for_hundredth: Some(0),
            timer_for_hundredth: Some(Timer::from_seconds(0.01, TimerMode::Repeating)),
        }
    }

    pub fn display(&self) -> String {
        let hundreds    = self.digit_for_hundred.unwrap_or(0);
        let tens        = self.digit_for_ten.unwrap_or(0);
        let ones        = self.digit_for_one.unwrap_or(0);
        let tenths      = self.digit_for_tenth.unwrap_or(0);
        let hundredths  = self.digit_for_hundredth.unwrap_or(0);

        format!("{}{}{}.{}{}", hundreds, tens, ones, tenths, hundredths)
    }
}
