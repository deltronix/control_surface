use core::ops::Not;

use defmt::Format;
use embedded_hal::digital::{Error, InputPin, PinState};
use embedded_hal_async::digital::Wait;

use crate::SurfaceElement;

#[derive(Default, Format, Debug, Copy, Clone, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ButtonState {
    #[default]
    Released,
    Pressed,
}

impl Not for ButtonState {
    type Output = bool;

    fn not(self) -> Self::Output {
        match self {
            ButtonState::Pressed => false,
            ButtonState::Released => true,
        }
    }
}
impl From<ButtonState> for bool {
    fn from(value: ButtonState) -> Self {
        match value {
            ButtonState::Released => false,
            ButtonState::Pressed => true,
        }
    }
}

pub struct DebouncedButton<I, const FILTER_SIZE: usize> {
    pin_sw: I,
    state: ButtonState,
    debounce: u8,
    index: u8,
    pull_up: bool,
    mask: u8,
}

/// Switch with a predefined debounce filter size should be between 2 and 8
impl<I, const FILTER_SIZE: usize> DebouncedButton<I, FILTER_SIZE>
where
    I: Into<bool>,
{
    pub fn new(pin_sw: I, pull_up: bool) -> Self {
        let mut mask: u8 = 0;
        defmt::assert!(FILTER_SIZE <= 8);
        defmt::assert!(FILTER_SIZE >= 2);
        for i in 0..FILTER_SIZE {
            mask |= 1 << i;
        }
        Self {
            pin_sw,
            state: ButtonState::default(),
            debounce: 0,
            index: 0,
            pull_up,
            mask,
        }
    }

    pub fn get_pin(&mut self) -> &mut I
    where
        I: Wait,
    {
        &mut self.pin_sw
    }

    /// Returns the internal state of the Switch, if it is held down returns true
    #[inline]
    pub fn is_pressed(&self) -> bool {
        self.state.into()
    }
    /// Returns the internal state of the Switch, if it is released returns true
    #[inline]
    pub fn is_released(&self) -> bool {
        !self.state
    }

    /// Call periodically to read switch pin and update state, returns Some(SwitchState) if a state
    /// change was detected, else returns None.
    pub fn debounce(&mut self, input: I) -> Option<ButtonState> {
        let sw = match self.pull_up {
            // TODO: reimplement
            true => !input.into(),
            false => input.into(),
        };

        if sw {
            self.debounce |= 1 << self.index;
        } else {
            self.debounce &= !(1 << self.index);
        }
        self.index += 1;
        if self.index >= FILTER_SIZE as u8 {
            self.index = 0;
        }

        if self.debounce == self.mask && !self.state {
            self.state = ButtonState::Pressed;
            Some(ButtonState::Pressed)
        } else if self.debounce == 0 && self.state.into() {
            self.state = ButtonState::Released;
            Some(ButtonState::Released)
        } else {
            None
        }
    }
}

impl<I> SurfaceElement for DebouncedButton<I, 4>
where
    I: Into<bool>,
{
    type Input = I;
    type Feedback = ();
    type Output = ButtonState;

    fn set_get(&mut self, value: Self::Input) -> Option<Self::Output> {
        self.debounce(value)
    }
    fn feedback(&mut self, value: Self::Feedback) {}
}
