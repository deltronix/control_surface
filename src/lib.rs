#![no_std]

//! A library
use embedded_hal::digital::InputPin;

use crate::rotary_encoder::RotaryEncoder;
pub mod button;
pub mod fader;
pub mod rotary_encoder;

#[cfg(feature = "midi")]
pub mod midi;

pub enum UiEvent {
    Fader,
    RotaryEncoder(usize, i32),
    Button(usize, button::ButtonState),
}

pub struct Bank<S: SurfaceElement, const N: usize> {
    elements: [S; N],
}

type RotaryEncoderBank = Bank<RotaryEncoder<dyn InputPin<Error = ()>, 4>, 4>;

pub trait SurfaceElement {
    type Input;
    type Feedback;
    type Output;

    fn set_get(&mut self, value: Self::Input) -> Option<Self::Output>;
    fn feedback(&mut self, value: Self::Feedback);
}

pub trait ControlSurface {
    type SurfaceEvent;
    fn update(&mut self) -> Option<Self::SurfaceEvent>;
}

#[cfg(test)]
mod tests {
    use crate::rotary_encoder::EncoderVelocityMap;

    #[test]
    fn encoder_velocity() {
        let mut map: EncoderVelocityMap<4> = EncoderVelocityMap::new()
            .with((1_000_000, 1.0))
            .with((500_000, 2.0))
            .with((250_000, 4.0))
            .with((100_000, 8.0));

        let mut inst = 0;
        assert_eq!(map.map(inst, Some(1)), Some(1));
        inst += 300_000;
        assert_eq!(map.map(inst, Some(1)), Some(2));
        inst += 400_000;
        assert_eq!(map.map(inst, Some(-1)), Some(-2));
        inst += 90_000;
        assert_eq!(map.map(inst, Some(-1)), Some(-8));
    }
}
