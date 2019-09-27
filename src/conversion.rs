use std::ops::*;
use crate::scalars::Scalar;
use crate::vectors::Vector;
use crate::units::*;

macro_rules! divide_convert_scalars {
    ($numerator:ty, $denominator:ty, $result:ty) => (
        impl Div<Scalar<$denominator>> for Scalar<$numerator> {
            type Output = Scalar<$result>;
            fn div(self, rhs: Scalar<$denominator>) -> Self::Output {
                Self::Output::from(self.value / rhs.value)
            }
        }

        impl Div<Scalar<$result>> for Scalar<$numerator> {
            type Output = Scalar<$denominator>;
            fn div(self, rhs: Scalar<$result>) -> Self::Output {
                Self::Output::from(self.value / rhs.value)
            }
        }

        impl Mul<Scalar<$denominator>> for Scalar<$result> {
            type Output = Scalar<$numerator>;
            fn mul(self, rhs: Scalar<$denominator>) -> Self::Output {
                Self::Output::from(self.value * rhs.value)
            }
        }

        impl Mul<Scalar<$result>> for Scalar<$denominator> {
            type Output = Scalar<$numerator>;
            fn mul(self, rhs: Scalar<$result>) -> Self::Output {
                Self::Output::from(self.value * rhs.value)
            }
        }
    );
}

macro_rules! divide_convert_vectors {
    ($numerator:ty, $denominator:ty, $result:ty) => (
        impl Div<Scalar<$denominator>> for Vector<$numerator> {
            type Output = Vector<$result>;
            fn div(self, rhs: Scalar<$denominator>) -> Self::Output {
                Self::Output::from((self.x / rhs, self.y / rhs))
            }
        }

        impl Mul<Scalar<$denominator>> for Vector<$result> {
            type Output = Vector<$numerator>;
            fn mul(self, rhs: Scalar<$denominator>) -> Self::Output {
                Self::Output::from((self.x * rhs, self.y * rhs))
            }
        }

        impl Mul<Vector<$result>> for Scalar<$denominator> {
            type Output = Vector<$numerator>;
            fn mul(self, rhs: Vector<$result>) -> Self::Output {
                Self::Output::from((self * rhs.x, self * rhs.y))
            }
        }
    );
}

macro_rules! divide_convert {
    ($numerator:ty, $denominator:ty, $result:ty) => (
        divide_convert_scalars!($numerator, $denominator, $result);
        divide_convert_vectors!($numerator, $denominator, $result);
    );
}

macro_rules! squares_scalar {
    ($normal:ty, $squared:ty) => (
        impl Mul for Scalar<$normal> {
            type Output = Scalar<$squared>;
            fn mul(self, rhs: Self) -> Self::Output {
                Self::Output::from(self.value * rhs.value)
            }
        }

        impl Div<Scalar<$normal>> for Scalar<$squared> {
            type Output = Scalar<$normal>;
            fn div(self, rhs: Scalar<$normal>) -> Self::Output {
                Self::Output::from(self.value / rhs.value)
            }
        }
    );
}

divide_convert!(Meters, Seconds, MetersPerSecond);
divide_convert!(MetersPerSecond, Seconds, MetersPerSecondSquared);
divide_convert_scalars!(Newtons, Kilograms, MetersPerSecondSquared);
divide_convert_scalars!(Meters, SecondsSquared, MetersPerSecondSquared);
divide_convert_scalars!(Joules, Meters, Newtons);
divide_convert_scalars!(Joules, Kilograms, JoulesPerKilogram);
divide_convert_scalars!(Joules, Seconds, JoulesPerSecond);
divide_convert_scalars!(JoulesPerSecond, JoulesPerKilogram, KilogramsPerSecond);
divide_convert_scalars!(Kilograms, Seconds, KilogramsPerSecond);
squares_scalar!(Meters, MetersSquared);
squares_scalar!(Seconds, SecondsSquared);
divide_convert_scalars!(MetersCubed, Meters, MetersSquared);
divide_convert_scalars!(Kilograms, MetersCubed, KilogramsPerMeterCubed);
divide_convert!(Meters, Pixels, MetersPerPixel);
divide_convert!(Radians, Seconds, RadiansPerSecond);

impl Div<Scalar<MetersPerPixel>> for Vector<Meters> {
    type Output = Vector<Pixels>;
    fn div(self, rhs: Scalar<MetersPerPixel>) -> Self::Output {
        Self::Output::from((self.x.value / rhs.value, self.y.value / rhs.value))
    }
}

impl Mul<Scalar<MetersPerPixel>> for Vector<Pixels> {
    type Output = Vector<Meters>;
    fn mul(self, rhs: Scalar<MetersPerPixel>) -> Self::Output {
        Self::Output::from((self.x.value * rhs.value, self.y.value * rhs.value))
    }
}


#[cfg(test)]
mod tests {
    use crate::types::*;

    #[test]
    fn time_and_velocity_to_position() {
        let v = Velocity::in_meters_per_second(2.0, 3.0);
        let t = Time::in_seconds(5.0);

        let expected = Position::in_meters(10.0, 15.0);

        assert_eq!(expected, v * t);
        assert_eq!(expected, t * v);
    }

    #[test]
    fn time_and_position_to_velocity() {
        let p = Position::in_meters(2.0, 3.0);
        let t = Time::in_seconds(5.0);

        let expected = Velocity::in_meters_per_second(0.4, 0.6);

        assert_eq!(expected, p / t);
    }

    #[test]
    fn time_and_acceleration_to_velocity() {
        let a = Acceleration::in_meters_per_second_squared(2.0, 3.0);
        let t = Time::in_seconds(5.0);

        let expected = Velocity::in_meters_per_second(10.0, 15.0);

        assert_eq!(expected, a.clone() * t);
        assert_eq!(expected, t * a);
    }

    #[test]
    fn time_and_velocity_to_acceleration() {
        let v = Velocity::in_meters_per_second(2.0, 3.0);
        let t = Time::in_seconds(5.0);

        let expected = Acceleration::in_meters_per_second_squared(0.4, 0.6);

        assert_eq!(expected, v / t);
    }

    #[test]
    fn length_and_length_to_area() {
        let l1 = Length::from(2.0);
        let l2 = Length::from(3.0);

        let expected = Area::from(6.0);

        assert_eq!(expected, l1 * l2);
    }

    #[test]
    fn area_and_length_to_length() {
        let area = Area::from(6.0);
        let length = Length::from(3.0);

        let expected = Length::from(2.0);

        assert_eq!(expected, area / length);
    }

    #[test]
    fn area_and_length_to_volume() {
        let area = Area::from(2.0);
        let length = Length::from(3.0);

        let expected = Volume::from(6.0);

        assert_eq!(expected, area * length);
    }

    #[test]
    fn volume_and_length_to_area() {
        let volume = Volume::from(6.0);
        let length = Length::from(3.0);

        let expected = Area::from(2.0);

        assert_eq!(expected, volume / length);
    }

    #[test]
    fn pixels() {
        let position = Position::in_meters(2.0, 3.0);
        let scale = Scale::from(0.5);
        let resolution = position / scale;

        assert_eq!(Resolution::from((4.0, 6.0)), resolution);
    }
}