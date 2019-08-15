use crate::*;
use crate::units::*;

pub type UnitVector = Vector<Unitless>;

pub type Time = Scalar<Seconds>;

pub type Mass = Scalar<Kilograms>;

pub type Length = Scalar<Meters>;
pub type Position = Vector<Meters>;

pub type Speed = Scalar<MetersPerSecond>;
pub type Velocity = Vector<MetersPerSecond>;

pub type AccelScalar = Scalar<MetersPerSecondSquared>;
pub type Acceleration = Vector<MetersPerSecondSquared>;

pub type Temperature = Scalar<Kelvin>;

pub type Force = Scalar<Newtons>;

pub type Energy = Scalar<Joules>;

pub type EnergyDensity = Scalar<JoulesPerKilogram>;
pub type EnergyRate = Scalar<JoulesPerSecond>;
pub type MassRate = Scalar<KilogramsPerSecond>;

pub type Area = Scalar<MetersSquared>;
pub type Volume = Scalar<MetersCubed>;
pub type Density = Scalar<KilogramsPerMeterCubed>;

pub type Resolution = Vector<Pixels>;
pub type Scale = Vector<MetersPerPixel>;