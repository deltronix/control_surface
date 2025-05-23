use defmt::Format;
use embedded_hal::digital::InputPin;
use embedded_hal_async::digital::Wait;

#[derive(Format, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum SwitchEvent {
    Pressed,
    Released,
}

pub struct Switch<I, const FILTER_SIZE: usize> {
    pin_sw: I,
    state: bool,
    debounce: u8,
    index: u8,
    pull_up: bool,
    mask: u8,
}

/// Switch with a predefined debounce filter size should be between 2 and 8
impl<I: InputPin, const FILTER_SIZE: usize> Switch<I, FILTER_SIZE> {
    pub fn new(pin_sw: I, pull_up: bool) -> Self {
        let mut mask: u8 = 0;
        defmt::assert!(FILTER_SIZE < 8);
        for i in 0..FILTER_SIZE {
            mask |= 1 << i;
        }
        Self {
            pin_sw,
            state: false,
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
        self.state
    }
    /// Returns the internal state of the Switch, if it is released returns true
    #[inline]
    pub fn is_released(&self) -> bool {
        !self.state
    }

    /// Call periodically to read switch pin and update state, returns Some(SwitchState) if a state
    /// change was detected, else returns None.
    pub fn poll(&mut self) -> Option<SwitchEvent> {
        let sw = match self.pull_up {
            true => self.pin_sw.is_low().unwrap(),
            false => self.pin_sw.is_high().unwrap(),
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
            self.state = true;
            Some(SwitchEvent::Pressed)
        } else if self.debounce == 0 && self.state {
            self.state = false;
            Some(SwitchEvent::Released)
        } else {
            None
        }
    }
}
