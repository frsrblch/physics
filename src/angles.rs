use crate::Float;
use std::f64::consts::PI;

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Angle(Float);

impl Angle {
    pub fn in_radians(radians: Float) -> Self {
        if radians < Self::MIN {
            let mut radians = -radians;
            radians = radians % Self::MAX;
            return Self(-radians + Self::MAX)
        }

        if radians >= Self::MAX {
            return Self(radians % Self::MAX)
        }

        Self(radians)
    }

    pub fn in_degrees(degrees: Float) -> Self {
        Self::in_radians(Self::get_radians(degrees))
    }

    fn get_radians(degrees: Float) -> Float {
        degrees * (PI as Float) / 180.0
    }

    const MIN: Float = 0.0;
    const MAX: Float = 2.0 * (PI as Float);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_angle_wraps() {
        let angle = Angle::in_degrees(-721.0);

        assert_eq!((Angle::in_degrees(359.0).0 * MULT).floor(), (angle.0 * MULT).floor());
    }

    #[test]
    fn too_large_angle_reduced() {
        let angle = Angle::in_degrees(721.0);

        assert_eq!((Angle::in_degrees(1.0).0 * MULT).floor(), (angle.0 * MULT).floor());
    }

    const MULT: Float = 100_000.0;
}