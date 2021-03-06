use std::ops::*;
use std::fmt::{Display, Formatter, Result, LowerExp};
use crate::*;
use crate::types::UnitVector;

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct Vector<T> {
    pub x: Scalar<T>,
    pub y: Scalar<T>,
}

impl<T> Vector<T> {
    #[inline]
    fn new<U: Into<Scalar<T>>>(x: U, y: U) -> Vector<T> {
        Vector { x: x.into(), y: y.into() }
    }

    #[inline]
    pub const fn zero() -> Vector<T> {
        Vector { x: Scalar::zero(), y: Scalar::zero() }
    }

    #[inline]
    pub fn magnitude(&self) -> Scalar<T> {
        self.magnitude_squared().sqrt().into()
    }

    #[inline]
    pub fn magnitude_squared(&self) -> Float {
        self.x.value.powi(2) + self.y.value.powi(2)
    }
}

impl<T: Unit> Vector<T> {
    #[inline]
    pub fn unit_vector(self) -> Option<UnitVector> {
        if self == Self::zero() {
            None
        } else {
            let m = self.magnitude();
            Some(Vector::new(self.x / m, self.y / m))
        }
    }

    #[inline]
    pub fn rotate_cw(&self, angle: Angle) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        let x = cos * self.x - sin * self.y;
        let y = sin * self.x + cos * self.y;
        Vector::new(x, y)
    }

    #[inline]
    pub fn from_magnitude_and_angle(magnitude: Scalar<T>, angle: Angle) -> Self {
        let angle = -angle;
        let cos = angle.cos();
        let sin = angle.sin();
        let x = -sin * magnitude;
        let y = cos * magnitude;
        Vector::new(x, y)
    }

    #[inline]
    pub fn get_angle(self) -> Option<Angle> {
        if self == Self::zero() {
            return None;
        }

        match (self.x.value, self.y.value) {
            (x, y) if x < 0.0 && y < 0.0 => Some(self.atan() - Angle::in_degrees(180.0)),
            (_, y) if y < 0.0 => Some(Angle::in_degrees(180.0) + self.atan()),
            _ => Some(self.atan())
        }
    }

    fn atan(self) -> Angle {
        Angle::in_radians((self.x / self.y).atan())
    }
}

impl Vector<Meters> {
    #[inline]
    pub fn in_meters<T: Into<Scalar<Meters>>>(x: T, y: T) -> Position {
        Vector::new(x, y)
    }
}

impl Vector<MetersPerSecond> {
    #[inline]
    pub fn in_meters_per_second<T: Into<Scalar<MetersPerSecond>>>(x: T, y: T) -> Velocity {
        Vector::new(x, y)
    }
}

impl Vector<MetersPerSecondSquared> {
    #[inline]
    pub fn in_meters_per_second_squared<T: Into<Scalar<MetersPerSecondSquared>>>(x: T, y: T) -> Acceleration {
        Vector::new(x, y)
    }
}

impl Vector<Pixels> {
    #[inline]
    pub fn in_pixels<T: Into<Scalar<Pixels>>>(x: T, y: T) -> Resolution {
        Vector::new(x, y)
    }
}

impl<T: Unit> Display for Vector<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        let precision = f.precision().unwrap_or(2);
        write!(f, "({:.p$}, {:.p$})", self.x, self.y, p=precision)
    }
}

impl<T: Unit> LowerExp for Vector<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        let precision = f.precision().unwrap_or(2);
        write!(f, "({:.p$e}, {:.p$e})", self.x, self.y, p=precision)
    }
}

impl<T, U: Into<Scalar<T>>> From<(U, U)> for Vector<T> {
    #[inline]
    fn from((x, y): (U, U)) -> Vector<T> {
        Vector::new(x, y)
    }
}

impl<T> Neg for Vector<T> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl<T> Add for Vector<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> AddAssign for Vector<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub for Vector<T> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> SubAssign for Vector<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Mul<Vector<T>> for Float {
    type Output = Vector<T>;
    #[inline]
    fn mul(self, rhs: Vector<T>) -> Vector<T> {
        Vector::new(self * rhs.x, self * rhs.y)
    }
}

impl<T> Mul<Float> for Vector<T> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Float) -> Self {
        Vector::new(self.x * rhs, self.y * rhs)
    }
}

impl<T> MulAssign<Float> for Vector<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: Float) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T> Div<Float> for Vector<T> {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Float) -> Self {
        Vector::new(self.x / rhs, self.y / rhs)
    }
}

impl<T> Div<Scalar<T>> for Vector<T> {
    type Output = Vector<Float>;
    #[inline]
    fn div(self, rhs: Scalar<T>) -> Self::Output {
        Vector::from((self.x.value / rhs.value, self.y.value / rhs.value))
    }
}

impl<T> DivAssign<Float> for Vector<T> {
    #[inline]
    fn div_assign(&mut self, rhs: Float) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T> Mul<UnitVector> for Scalar<T> {
    type Output = Vector<T>;
    #[inline]
    fn mul(self, rhs: UnitVector) -> Self::Output {
        Vector::new(self.value * rhs.x.value, self.value * rhs.y.value)
    }
}

impl<T> Mul<Scalar<T>> for UnitVector {
    type Output = Vector<T>;
    #[inline]
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

        assert_eq!("(1.50 m, 2.50 m)", position.to_string());
    }

    #[test]
    fn display_precision() {
        let position = Position::new(1.44444, 2.555555);

        assert_eq!("(1.4 m, 2.6 m)", format!("{:.1}", position));
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
    fn div_self() {
        let position = Meters::get_vector(3.0, 5.0);
        let length = Meters::get_scalar(2.0);

        assert_eq!(position / length, Float::get_vector(1.5, 2.5));
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

    #[test]
    fn get_angle_from_zero_returns_none() {
        assert_eq!(None, Position::in_meters(0.0, 0.0).get_angle());
    }

    #[test]
    fn get_angle_0_1() {
        assert_eq!(Some(Angle::in_degrees(0.0)), Position::in_meters(0.0, 1.0).get_angle());
    }

    #[test]
    fn get_angle_1_1() {
        assert_eq!(Some(Angle::in_degrees(45.0)), Position::in_meters(1.0, 1.0).get_angle());
    }

    #[test]
    fn get_angle_1_0() {
        assert_eq!(Some(Angle::in_degrees(90.0)), Position::in_meters(1.0, 0.0).get_angle());
    }

    #[test]
    fn get_angle_1_n1() {
        assert_eq!(Some(Angle::in_degrees(135.0)), Position::in_meters(1.0, -1.0).get_angle());
    }

    #[test]
    fn get_angle_0_n1() {
        assert_eq!(Some(Angle::in_degrees(180.0)), Position::in_meters(0.0, -1.0).get_angle());
    }

    #[test]
    fn get_angle_n1_n1() {
        assert_eq!(Some(Angle::in_degrees(-135.0)), Position::in_meters(-1.0, -1.0).get_angle());
    }

    #[test]
    fn get_angle_n1_0() {
        assert_eq!(Some(Angle::in_degrees(-90.0)), Position::in_meters(-1.0, 0.0).get_angle());
    }

    #[test]
    fn get_angle_n1_1() {
        assert_eq!(Some(Angle::in_degrees(-45.0)), Position::in_meters(-1.0, 1.0).get_angle());
    }

    #[test]
    fn vector_from_angle_has_expected_angle() {
        let angle = Angle::in_degrees(35.0);
        let pos = Position::from_magnitude_and_angle(Length::in_meters(3.0), angle);

        assert_eq!(pos.get_angle(), Some(angle));
    }
}