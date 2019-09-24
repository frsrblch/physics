use crate::Float;
use std::ops::*;
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

    pub fn get_degrees(&self) -> Float {
        self.0 * 180.0 / (PI as Float)
    }

    const MIN: Float = 0.0;
    const MAX: Float = 2.0 * (PI as Float);

    pub fn sin(&self) -> Float {
        self.0.sin()
    }

    pub fn cos(&self) -> Float {
        self.0.cos()
    }

    pub fn tan(&self) -> Float {
        self.0.tan()
    }
}

impl Mul<Float> for Angle {
    type Output = Self;
    fn mul(self, rhs: Float) -> Self::Output {
        Self::in_radians(self.0 * rhs)
    }
}

impl MulAssign<Float> for Angle {
    fn mul_assign(&mut self, rhs: Float) {
        self.0 *= rhs;
    }
}

impl Div<Float> for Angle {
    type Output = Self;
    fn div(self, rhs: Float) -> Self::Output {
        Self::in_radians(self.0 / rhs)
    }
}

impl DivAssign<Float> for Angle {
    fn div_assign(&mut self, rhs: Float) {
        self.0 /= rhs;
    }
}

impl Add for Angle {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::in_radians(self.0 + rhs.0)
    }
}

impl Sub for Angle {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::in_radians(self.0 - rhs.0)
    }
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

    #[test]
    fn mul() {
        let angle = Angle::in_degrees(20.0);
        let expected = Angle::in_degrees(40.0);

        assert_eq!(expected, angle * 2.0);
    }

    #[test]
    fn mul_assign() {
        let mut angle = Angle::in_degrees(20.0);
        angle *= 2.0;
        let expected = Angle::in_degrees(40.0);

        assert_eq!(expected, angle);
    }

    #[test]
    fn div() {
        let angle = Angle::in_degrees(20.0);
        let expected = Angle::in_degrees(10.0);

        assert_eq!(expected, angle / 2.0);
    }

    #[test]
    fn div_assign() {
        let mut angle = Angle::in_degrees(20.0);
        angle /= 2.0;
        let expected = Angle::in_degrees(10.0);

        assert_eq!(expected, angle);
    }
}