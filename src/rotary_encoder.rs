use embedded_hal::digital::InputPin;
use heapless::Vec;

/// Time driver agnostic velocity mapping for a RotaryEncoder.
pub struct EncoderVelocityMap<const MAP_SIZE: usize> {
    last_instant: Option<u64>,
    map: Vec<(u64, f32), MAP_SIZE>,
}
impl<const MAP_SIZE: usize> Default for EncoderVelocityMap<MAP_SIZE> {
    fn default() -> Self {
        Self {
            last_instant: None,
            map: Default::default(),
        }
    }
}

impl<const MAP_SIZE: usize> EncoderVelocityMap<MAP_SIZE> {
    pub fn new() -> Self {
        Self {
            last_instant: None,
            map: Vec::new(),
        }
    }
    pub fn with(mut self, map: (u64, f32)) -> Self {
        match self.map.push(map) {
            Ok(_) => {
                self.map.sort_unstable_by(|a, b| a.0.cmp(&b.0));
                self
            }
            Err(_) => panic!("No space for velocity map"),
        }
    }
    pub fn map(&mut self, instant: u64, ticks: Option<i32>) -> Option<i32> {
        match ticks {
            Some(ticks) => {
                if self.map.is_empty() {
                    return Some(ticks);
                }

                if let Some(inst) = self.last_instant {
                    if let Some(dur) = instant.checked_sub(inst) {
                        if let Some(map) = self.map.iter().find(|m| dur < m.0) {
                            self.last_instant = Some(instant);
                            Some((ticks as f32 * map.1) as i32)
                        } else {
                            self.last_instant = Some(instant);
                            Some(ticks)
                        }
                    } else {
                        None
                    }
                } else {
                    self.last_instant = Some(instant);
                    Some(ticks)
                }
            }
            None => None,
        }
    }
}

pub enum EncoderEvent {
    Ticks(usize, i32),
}

pub struct RotaryEncoder<I: InputPin, const FILTER_SIZE: usize> {
    pin_a: I,
    pin_b: I,
    state_a: bool,
    state_b: bool,
    debounce_a: u8,
    debounce_b: u8,
    index: u8,
    pull_up: bool,
    mask: u8,
    ticks: i32,
}

/// Rotary Encoder with a predefined debounce filter size should be between 2 and 8
impl<I: InputPin, const FILTER_SIZE: usize> RotaryEncoder<I, FILTER_SIZE> {
    pub fn new(pin_a: I, pin_b: I, pull_up: bool) -> Self {
        let mut mask: u8 = 0;
        defmt::assert!(FILTER_SIZE < 8);
        for i in 0..FILTER_SIZE {
            mask |= 1 << i;
        }
        Self {
            pin_a,
            pin_b,
            state_a: false,
            state_b: false,
            debounce_a: 0,
            debounce_b: 0,
            index: 0,
            pull_up,
            mask,
            ticks: 0,
        }
    }
    pub fn ticks(&self) -> i32 {
        self.ticks
    }
    pub fn reset_ticks(&mut self) {
        self.ticks = 0;
    }

    /// Call periodically to read encoder pins and update state
    pub fn poll(&mut self) -> Option<i32> {
        let (a, b) = match self.pull_up {
            true => (self.pin_a.is_low().unwrap(), self.pin_b.is_low().unwrap()),
            false => (self.pin_a.is_high().unwrap(), self.pin_b.is_high().unwrap()),
        };

        if a {
            self.debounce_a |= 1 << self.index;
        } else {
            self.debounce_a &= !(1 << self.index);
        }
        if b {
            self.debounce_b |= 1 << self.index;
        } else {
            self.debounce_b &= !(1 << self.index);
        }

        self.index += 1;
        if self.index >= FILTER_SIZE as u8 {
            self.index = 0;
        }

        let (old_a, old_b) = (self.state_a, self.state_b);
        if self.debounce_a == self.mask {
            self.state_a = true;
        } else if self.debounce_a == 0 {
            self.state_a = false;
        }
        if self.debounce_b == self.mask {
            self.state_b = true
        } else if self.debounce_b == 0 {
            self.state_b = false;
        }

        if self.state_a ^ self.state_b {
            if self.state_a && self.state_a != old_a {
                return Some(-1);
            }
            if self.state_b && self.state_b != old_b {
                return Some(1);
            }
        }
        None
    }
}
