use std::ops::*;
use std::marker::PhantomData;
use std::fmt::{Display, Formatter, Result};
use specs::{Component, VecStorage};
use crate::*;
use crate::units::*;

#[derive(Debug, Default, Copy, Clone, PartialOrd)]
pub struct Scalar<T> {
    pub value: Float,
    marker: PhantomData<T>,
}

impl<T: Unit> Display for Scalar<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match T::symbol() {
            Some(symbol) => write!(f, "{} {}", self.value, symbol),
            None => write!(f, "{}", self.value),
        }
    }
}

impl<T> Scalar<T> {
    fn new(value: Float) -> Self {
        Scalar { value, marker: PhantomData }
    }

    pub fn zero() -> Self {
        Self::new(0.0)
    }
}

impl<T> Neg for Scalar<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl<T> From<Float> for Scalar<T> {
    fn from(value: Float) -> Self {
        Self::new(value)
    }
}

impl<T> PartialEq for Scalar<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.value == rhs.value
    }
}

impl<T> Add for Scalar<T> {
    type Output = Scalar<T>;
    fn add(self, rhs: Self) -> Self {
        Scalar::from(self.value + rhs.value)
    }
}

impl<T> AddAssign for Scalar<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value
    }
}

impl<T> Sub for Scalar<T> {
    type Output = Scalar<T>;
    fn sub(self, rhs: Self) -> Self {
        Scalar::from(self.value - rhs.value)
    }
}

impl<T> SubAssign for Scalar<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value
    }
}

impl<T> Mul<Float> for Scalar<T> {
    type Output = Scalar<T>;
    fn mul(self, rhs: Float) -> Self {
        Scalar::from(self.value * rhs)
    }
}

impl<T> Mul<Scalar<T>> for Float {
    type Output = Scalar<T>;
    fn mul(self, rhs: Scalar<T>) -> Scalar<T> {
        rhs * self
    }
}

impl<T> MulAssign<Float> for Scalar<T> {
    fn mul_assign(&mut self, rhs: Float) {
        self.value *= rhs;
    }
}

impl<T> Div<Float> for Scalar<T> {
    type Output = Scalar<T>;
    fn div(self, rhs: Float) -> Self {
        Scalar::from(self.value / rhs)
    }
}

impl<T> Div<Self> for Scalar<T> {
    type Output = Float;
    fn div(self, rhs: Self) -> Self::Output {
        self.value / rhs.value
    }
}

impl<T> DivAssign<Float> for Scalar<T> {
    fn div_assign(&mut self, rhs: Float) {
        self.value /= rhs;
    }
}

impl Scalar<Seconds> {
    pub fn in_seconds(value: Float) -> Self {
        Self::new(value)
    }

    pub fn in_minutes(value: Float) -> Self {
        Time::in_seconds(value * 60.0)
    }

    pub fn in_hours(value: Float) -> Self {
        Time::in_minutes(value * 60.0)
    }

    pub fn in_days(value: Float) -> Self {
        Time::in_hours(value * 24.0)
    }

    pub fn in_years(value: Float) -> Self {
        Time::in_days(value * 365.25)
    }
}

impl Scalar<Kilograms> {
    pub fn in_kilograms(value: Float) -> Self {
        value.into()
    }
}

impl Scalar<Meters> {
    pub fn in_meters(value: Float) -> Self {
        value.into()
    }
}

impl Scalar<MetersPerSecond> {
    pub fn in_meters_per_second(value: Float) -> Self {
        value.into()
    }
}

impl Scalar<Kelvin> {
    pub fn in_kelvin(value: Float) -> Self {
        value.into()
    }
}

impl Scalar<Newtons> {
    pub fn in_newtons(value: Float) -> Self {
        value.into()
    }
}

impl Scalar<Joules> {
    pub fn in_joules(value: Float) -> Self {
        value.into()
    }

    pub fn in_kilocalories(value: Float) -> Self {
        (value * 4184.0).into()
    }
}

impl Scalar<JoulesPerKilogram> {
    pub fn in_joules_per_kilogram(value: Float) -> Self {
        value.into()
    }
}

impl Scalar<JoulesPerSecond> {
    pub fn in_joules_per_second(value: Float) -> Self {
        value.into()
    }
}

impl Scalar<MetersSquared> {
    pub fn in_meters_squared(value: Float) -> Self { value.into() }
}

impl Scalar<MetersCubed> {
    pub fn in_meters_cubed(value: Float) -> Self { value.into() }
}

impl Scalar<KilogramsPerMeterCubed> {
    pub fn in_kilograms_per_meter_cubed(value: Float) -> Self {
        value.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Time;

    type NoDimension = Scalar<crate::units::Unitless>;

    #[test]
    fn display() {
        let time = Time::from(1.25);
        assert_eq!("1.25 s", time.to_string());

        let dimensionless = NoDimension::from(1.25);
        assert_eq!("1.25", dimensionless.to_string());
    }

    #[test]
    fn size() {
        use std::mem::size_of;
        assert_eq!(size_of::<Float>(), size_of::<Time>());
    }

    #[test]
    fn comparison() {
        let lhs = Time::from(2.0);
        let rhs = Time::from(3.0);

        assert!(lhs < rhs);
    }

    #[test]
    fn add() {
        let lhs = Time::from(2.0);
        let rhs = Time::from(3.0);

        let expected = Time::from(5.0);

        assert_eq!(expected, lhs + rhs);
    }

    #[test]
    fn add_assign() {
        let mut lhs = Time::from(2.0);
        let rhs = Time::from(3.0);

        lhs += rhs;
        let expected = Time::from(5.0);

        assert_eq!(expected, lhs);
    }

    #[test]
    fn sub() {
        let lhs = Time::from(5.0);
        let rhs = Time::from(2.0);

        let expected = Time::from(3.0);

        assert_eq!(expected, lhs - rhs);
    }

    #[test]
    fn sub_assign() {
        let mut lhs = Time::from(5.0);
        let rhs = Time::from(2.0);

        lhs -= rhs;
        let expected = Time::from(3.0);

        assert_eq!(expected, lhs);
    }

    #[test]
    fn mul_0() {
        let lhs = Time::from(2.0);
        let rhs = 3.0;

        let expected = Time::from(6.0);

        assert_eq!(expected, lhs * rhs)
    }

    #[test]
    fn mul_1() {
        let lhs = 3.0;
        let rhs = Time::from(2.0);

        let expected = Time::from(6.0);

        assert_eq!(expected, lhs * rhs)
    }

    #[test]
    fn mul_assign() {
        let mut lhs = Time::from(2.0);
        let rhs = 3.0;

        lhs *= rhs;
        let expected = Time::from(6.0);

        assert_eq!(expected, lhs);
    }

    #[test]
    fn div() {
        let lhs = Time::from(3.0);
        let rhs = 2.0;

        let expected = Time::from(1.5);

        assert_eq!(expected, lhs / rhs);
    }

    #[test]
    fn div_assign() {
        let mut lhs = Time::from(6.0);
        let rhs = 2.0;

        lhs /= rhs;
        let expected = Time::from(3.0);

        assert_eq!(expected, lhs);
    }

    #[test]
    fn div_self() {
        let t1 = Time::in_seconds(1.0);
        let t2 = Time::in_seconds(2.0);

        assert_eq!(0.5, t1 / t2);
    }

    #[test]
    fn time_tests() {
        assert_eq!(Time::in_seconds(60.0), Time::in_minutes(1.0));
        assert_eq!(Time::in_seconds(60.0 * 60.0), Time::in_hours(1.0));
        assert_eq!(Time::in_seconds(60.0 * 60.0 * 24.0), Time::in_days(1.0));
        assert_eq!(Time::in_seconds(60.0 * 60.0 * 24.0 * 365.25), Time::in_years(1.0));
    }

    #[test]
    fn energy_values() {
        assert_eq!(Energy::in_joules(4184.0), Energy::in_kilocalories(1.0));
    }
}