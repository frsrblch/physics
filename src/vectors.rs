use std::ops::*;
use std::fmt::{Display, Formatter, Result};
use crate::*;
use crate::units::*;
use crate::types::UnitVector;

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Vector<T> {
    pub x: Scalar<T>,
    pub y: Scalar<T>,
}

impl<T> Vector<T> {
    fn new<U: Into<Scalar<T>>>(x: U, y: U) -> Vector<T> {
        Vector { x: x.into(), y: y.into() }
    }

    pub fn zero() -> Vector<T> {
        Vector::new(0.0, 0.0)
    }

    pub fn magnitude(&self) -> Scalar<T> {
        self.magnitude_squared().sqrt().into()
    }

    pub fn magnitude_squared(&self) -> Float {
        self.x.value.powi(2) + self.y.value.powi(2)
    }
}

impl<T: Unit> Vector<T> {
    pub fn unit_vector(self) -> Option<UnitVector> {
        match self == Self::zero() {
            true => None,
            false => {
                let m = self.magnitude();
                Some(Vector::new(self.x / m, self.y / m))
            }
        }
    }

    pub fn rotate_cw(&self, angle: Angle) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        let x = cos * self.x - sin * self.y;
        let y = sin * self.x + cos * self.y;
        Vector::new(x, y)
    }
}

impl Vector<Meters> {
    pub fn in_meters<T: Into<Scalar<Meters>>>(x: T, y: T) -> Position {
        Vector::new(x, y)
    }
}

impl Vector<MetersPerSecond> {
    pub fn in_meters_per_second<T: Into<Scalar<MetersPerSecond>>>(x: T, y: T) -> Velocity {
        Vector::new(x, y)
    }
}

impl Vector<MetersPerSecondSquared> {
    pub fn in_meters_per_second_squared<T: Into<Scalar<MetersPerSecondSquared>>>(x: T, y: T) -> Acceleration {
        Vector::new(x, y)
    }
}

impl Vector<Pixels> {
    pub fn in_pixels<T: Into<Scalar<Pixels>>>(x: T, y: T) -> Resolution {
        Vector::new(x, y)
    }
}

impl<T: Unit> Display for Vector<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T, U: Into<Scalar<T>>> From<(U, U)> for Vector<T> {
    fn from((x, y): (U, U)) -> Vector<T> {
        Vector::new(x, y)
    }
}

impl<T> Neg for Vector<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl<T> Add for Vector<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> AddAssign for Vector<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub for Vector<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> SubAssign for Vector<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Mul<Vector<T>> for Float {
    type Output = Vector<T>;
    fn mul(self, rhs: Vector<T>) -> Vector<T> {
        Vector::new(self * rhs.x, self * rhs.y)
    }
}

impl<T> Mul<Float> for Vector<T> {
    type Output = Self;
    fn mul(self, rhs: Float) -> Self {
        Vector::new(self.x * rhs, self.y * rhs)
    }
}

impl<T> MulAssign<Float> for Vector<T> {
    fn mul_assign(&mut self, rhs: Float) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T> Div<Float> for Vector<T> {
    type Output = Self;
    fn div(self, rhs: Float) -> Self {
        Vector::new(self.x / rhs, self.y / rhs)
    }
}

impl<T> DivAssign<Float> for Vector<T> {
    fn div_assign(&mut self, rhs: Float) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T> Mul<UnitVector> for Scalar<T> {
    type Output = Vector<T>;
    fn mul(self, rhs: UnitVector) -> Self::Output {
        Vector::new(self.value * rhs.x.value, self.value * rhs.y.value)
    }
}

impl<T> Mul<Scalar<T>> for UnitVector {
    type Output = Vector<T>;
    fn mul(self, rhs: Scalar<T>) -> Self::Output {
        rhs * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Position;

    #[test]
    fn zero() {
        assert_eq!(Position::new(0.0, 0.0), Vector::zero());
    }

    #[test]
    fn display() {
        let position = Position::new(1.5, 2.5);

        assert_eq!("(1.5 m, 2.5 m)", position.to_string());
    }

    #[test]
    fn add() {
        let v1 = Position::new(2.0, 3.0);
        let v2 = Position::new(5.0, 7.0);

        assert_eq!(Position::new(7.0, 10.0), v1 + v2);
    }

    #[test]
    fn add_assign() {
        let mut v1 = Position::new(2.0, 3.0);
        let v2 = Position::new(5.0, 7.0);

        v1 += v2;

        assert_eq!(Position::new(7.0, 10.0), v1);
    }

    #[test]
    fn add_assign_borrow() {
        let mut v1 = Position::new(2.0, 3.0);
        let v2 = Position::new(5.0, 7.0);

        {
            let v = &mut v1;
            *v += v2;
        }

        assert_eq!(Position::new(7.0, 10.0), v1);
    }

    #[test]
    fn sub() {
        let v1 = Position::new(2.0, 3.0);
        let v2 = Position::new(5.0, 7.0);

        assert_eq!(Position::new(-3.0, -4.0), v1 - v2);
    }

    #[test]
    fn sub_assign() {
        let mut v1 = Position::new(2.0, 3.0);
        let v2 = Position::new(5.0, 7.0);

        v1 -= v2;

        assert_eq!(Position::new(-3.0, -4.0), v1);
    }

    #[test]
    fn sub_assign_borrow() {
        let mut v1 = Position::new(2.0, 3.0);
        let v2 = Position::new(5.0, 7.0);

        {
            let v = &mut v1;
            *v -= v2;
        }

        assert_eq!(Position::new(-3.0, -4.0), v1);
    }

    #[test]
    fn mul() {
        let v = Position::new(2.0, 3.0);
        let x = 5.0;

        assert_eq!(Position::new(10.0, 15.0), v * x);
        assert_eq!(Position::new(10.0, 15.0), x * v);
    }

    #[test]
    fn mul_assign() {
        let mut v = Position::new(2.0, 3.0);
        let x = 5.0;

        v *= x;

        assert_eq!(Position::new(10.0, 15.0), v);
    }

    #[test]
    fn mul_assign_borrowed() {
        let mut v = Position::new(2.0, 3.0);
        let x = 5.0;

        {
            let v1 = &mut v;
            *v1 *= x;
        }

        assert_eq!(Position::new(10.0, 15.0), v);
    }

    #[test]
    fn div() {
        let v = Position::new(2.0, 3.0);
        let x = 5.0;

        assert_eq!(Position::new(0.4, 0.6), v / x);
    }

    #[test]
    fn div_assign() {
        let mut v = Position::new(2.0, 3.0);
        let x = 5.0;

        v /= x;

        assert_eq!(Position::new(0.4, 0.6), v);
    }

    #[test]
    fn div_assign_borrowed() {
        let mut v = Position::new(2.0, 3.0);
        let x = 5.0;

        {
            let v1 = &mut v;
            *v1 /= x;
        }

        assert_eq!(Position::new(0.4, 0.6), v);
    }

    #[test]
    fn magnitude_squared() {
        let v = Position::new(3.0, 4.0);

        assert_eq!(25.0, v.magnitude_squared());
    }

    #[test]
    fn magnitude() {
        let v = Position::new(3.0, 4.0);

        assert_eq!(5.0, v.magnitude().value);
    }

    #[test]
    fn unit_vector_given_zero_vector_returns_none() {
        let v = Position::zero();

        assert_eq!(None, v.unit_vector());
    }

    #[test]
    fn unit_vector_given_vector_returns_unit_vector() {
        let v = Position::new(0.0, 2.1);

        assert_eq!(Some(UnitVector::new(0.0, 1.0)), v.unit_vector());
    }

    #[test]
    fn unit_vector_conversion() {
        let position = Position::in_meters(1.0, 2.0);
        let length = position.magnitude();
        let unit_vector = position.unit_vector().unwrap();

        assert_eq!(position, length * unit_vector);
        assert_eq!(position, unit_vector * length);
    }
}