#![no_std]
pub mod rotary_encoder;
pub mod switch;

pub enum UiEvent {
    RotaryEncoder(usize, i32),
    Switch(usize, switch::SwitchEvent),
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
