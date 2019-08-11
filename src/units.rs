pub trait Unit : 'static + Send + Sync + Copy + PartialEq {
    fn symbol() -> Option<&'static str>;
}


#[derive(Debug, Default, PartialEq, Copy, Clone, PartialOrd)]
pub struct Unitless;

impl Unit for Unitless {
    fn symbol() -> Option<&'static str> { None }
}

#[derive(Debug, Default, PartialEq, Copy, Clone, PartialOrd)]
pub struct Seconds;

impl Unit for Seconds {
    fn symbol() -> Option<&'static str> { Some("s") }
}

#[derive(Debug, Default, PartialEq, Copy, Clone, PartialOrd)]
pub struct SecondsSquared;

impl Unit for SecondsSquared {
    fn symbol() -> Option<&'static str> { Some("s²") }
}

#[derive(Debug, Default, PartialEq, Copy, Clone, PartialOrd)]
pub struct Kilograms;

impl Unit for Kilograms {
    fn symbol() -> Option<&'static str> { Some("kg") }
}

#[derive(Debug, Default, PartialEq, Copy, Clone, PartialOrd)]
pub struct Meters;

impl Unit for Meters {
    fn symbol() -> Option<&'static str> { Some("m") }
}

#[derive(Debug, Default, PartialEq, Copy, Clone, PartialOrd)]
pub struct MetersPerSecond;

impl Unit for MetersPerSecond {
    fn symbol() -> Option<&'static str> { Some("m/s") }
}

#[derive(Debug, Default, PartialEq, Copy, Clone, PartialOrd)]
pub struct MetersPerSecondSquared;

impl Unit for MetersPerSecondSquared {
    fn symbol() -> Option<&'static str> { Some("m/s²") }
}

#[derive(Debug, Default, PartialEq, Copy, Clone, PartialOrd)]
pub struct Kelvin;

impl Unit for Kelvin {
    fn symbol() -> Option<&'static str> { Some("K") }
}

#[derive(Debug, Default, PartialEq, Copy, Clone, PartialOrd)]
pub struct Newtons;

impl Unit for Newtons {
    fn symbol() -> Option<&'static str> { Some("N") }
}

#[derive(Debug, Default, PartialEq, Copy, Clone, PartialOrd)]
pub struct Joules;

impl Unit for Joules {
    fn symbol() -> Option<&'static str> { Some("J") }
}

#[derive(Debug, Default, PartialEq, Copy, Clone, PartialOrd)]
pub struct JoulesPerKilogram;

impl Unit for JoulesPerKilogram {
    fn symbol() -> Option<&'static str> { Some("J/kg") }
}

#[derive(Debug, Default, PartialEq, Copy, Clone, PartialOrd)]
pub struct JoulesPerSecond;

impl Unit for JoulesPerSecond {
    fn symbol() -> Option<&'static str> { Some("J/s") }
}

#[derive(Debug, Default, PartialEq, Copy, Clone, PartialOrd)]
pub struct KilogramsPerSecond;

impl Unit for KilogramsPerSecond {
    fn symbol() -> Option<&'static str> { Some("kg/s") }
}

#[derive(Debug, Default, PartialEq, Copy, Clone, PartialOrd)]
pub struct KilogramsPerMeterCubed;

impl Unit for KilogramsPerMeterCubed {
    fn symbol() -> Option<&'static str> { Some("kg/m³") }
}

#[derive(Debug, Default, PartialEq, Copy, Clone, PartialOrd)]
pub struct MetersSquared;

impl Unit for MetersSquared {
    fn symbol() -> Option<&'static str> { Some("m²") }
}

#[derive(Debug, Default, PartialEq, Copy, Clone, PartialOrd)]
pub struct MetersCubed;

impl Unit for MetersCubed {
    fn symbol() -> Option<&'static str> { Some("m³") }
}