use crate::Float;
use crate::scalars::Scalar;
use crate::vectors::Vector;

pub trait Unit : 'static + Send + Sync + Copy + PartialEq + Default {
    #[inline]
    fn symbol() -> Option<&'static str> {
        None
    }
}

impl Unit for Float {}

pub trait Units {
    type Output: Unit;

    #[inline]
    fn get_scalar(value: Float) -> Scalar<Self::Output> {
        Scalar::from(value)
    }

    #[inline]
    fn get_vector(x: Float, y: Float) -> Vector<Self::Output> {
        Vector::from((x, y))
    }
}

impl<T: Unit> Units for T {
    type Output = T;
}

macro_rules! define_unit {
    ($unit:ident) => (
        #[derive(Debug, Default, PartialEq, Copy, Clone, PartialOrd)]
        pub struct $unit;

        impl Unit for $unit {
            #[inline]
            fn symbol() -> Option<&'static str> { None }
        }
    );
    ($unit:ident, $symbol:literal) => (
        #[derive(Debug, Default, PartialEq, Copy, Clone, PartialOrd)]
        pub struct $unit;

        impl Unit for $unit {
            #[inline]
            fn symbol() -> Option<&'static str> { Some($symbol) }
        }
    )
}

define_unit!(Seconds, "s");
define_unit!(SecondsSquared, "s²");
define_unit!(Kilograms, "kg");
define_unit!(Meters, "m");
define_unit!(MetersSquared, "m²");
define_unit!(MetersCubed, "m³");
define_unit!(MetersPerSecond, "m/s");
define_unit!(MetersPerSecondSquared, "m/s²");
define_unit!(Kelvin, "K");
define_unit!(Newtons, "N");
define_unit!(Joules, "J");
define_unit!(JoulesPerKilogram, "J/kg");
define_unit!(JoulesPerSecond, "J/s");
define_unit!(KilogramsPerSecond, "kg/s");
define_unit!(KilogramsPerMeterCubed, "kg/m³");
define_unit!(Pixels, "px");
define_unit!(MetersPerPixel, "m/px");
define_unit!(Radians, "rad");
define_unit!(RadiansPerSecond, "rad/s");
