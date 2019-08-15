pub trait Unit : 'static + Send + Sync + Copy + PartialEq + Default {
    fn symbol() -> Option<&'static str>;
}

macro_rules! define_unit {
    ($unit:ident) => (
        #[derive(Debug, Default, PartialEq, Copy, Clone, PartialOrd)]
        pub struct $unit;

        impl Unit for $unit {
            fn symbol() -> Option<&'static str> { None }
        }
    );
    ($unit:ident, $symbol:literal) => (
        #[derive(Debug, Default, PartialEq, Copy, Clone, PartialOrd)]
        pub struct $unit;

        impl Unit for $unit {
            fn symbol() -> Option<&'static str> { Some($symbol) }
        }
    )
}

define_unit!(Unitless);
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