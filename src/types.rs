use bitflags::bitflags;

use super::register::StatusRegAuxA;

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<CommE, PinE> {
    /// I²C / SPI communication error
    Comm(CommE),
    /// Chip-select pin error (SPI)
    Pin(PinE),
    /// Invalid input data provided
    InvalidInputData,
}

/// Error when changing operating mode.
#[derive(Debug)]
pub struct ModeChangeError<CommE, PinE, DEV> {
    /// Inner error.
    pub error: Error<CommE, PinE>,
    /// Original device without mode changed.
    pub dev: DEV,
}

/// Device operation modes
pub mod mode {
    /// Marker type for magnetometer in one-shot (single) mode.
    #[derive(Debug)]
    pub enum MagOneShot {}
    /// Marker type for magnetometer in continuous mode.
    #[derive(Debug)]
    pub enum MagContinuous {}
}

/// An Accelerometer ID.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AccelerometerId {
    pub(crate) raw: u8,
    pub(crate) id: u8,
}

impl AccelerometerId {
    pub(crate) fn from_bits_truncate(raw: u8) -> Self {
        Self { raw, id: 0 }
    }

    /// Raw accelerometer ID.
    pub const fn raw(&self) -> u8 {
        self.raw
    }

    /// Check if the ID corresponds to the expected value.
    pub const fn is_correct(&self) -> bool {
        self.raw == self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResolutionFactor {
    R1,
    R16,
    R64,
    R256,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ScalingFactor {
    S0,
    S1,
    S2,
    S4,
    S8,
    S16,
    S32,
    S64,
    S128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AccelerationFactor {
    resolution_factor: ResolutionFactor,
    scaling_factor: ScalingFactor,
}

impl AccelerationFactor {
    pub const fn new(resolution_factor: ResolutionFactor, scaling_factor: ScalingFactor) -> Self {
        Self {
            resolution_factor,
            scaling_factor,
        }
    }

    pub const fn resolution(self) -> i16 {
        use ResolutionFactor::*;

        match self.resolution_factor {
            R1 => 1,
            R16 => 16,
            R64 => 64,
            R256 => 256,
        }
    }

    pub const fn scaling(self) -> i32 {
        use ScalingFactor::*;

        match self.scaling_factor {
            S0 => 0,
            S1 => 1,
            S2 => 2,
            S4 => 4,
            S8 => 8,
            S16 => 16,
            S32 => 32,
            S64 => 64,
            S128 => 128,
        }
    }
}

/// An acceleration measurement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Acceleration {
    pub(crate) x: u16,
    pub(crate) y: u16,
    pub(crate) z: u16,
    pub(crate) factor: AccelerationFactor,
}

impl Acceleration {
    /// Raw acceleration in X-direction.
    #[inline]
    pub const fn x_raw(&self) -> u16 {
        self.x
    }

    /// Raw acceleration in Y-direction.
    #[inline]
    pub const fn y_raw(&self) -> u16 {
        self.y
    }

    /// Raw acceleration in Z-direction.
    #[inline]
    pub const fn z_raw(&self) -> u16 {
        self.z
    }

    /// Raw acceleration in X-, Y- and Z-directions.
    #[inline]
    pub const fn xyz_raw(&self) -> (u16, u16, u16) {
        (self.x, self.y, self.z)
    }

    /// Unscaled acceleration in X-direction.
    #[inline]
    pub const fn x_unscaled(&self) -> i16 {
        self.x as i16 / self.factor.resolution()
    }

    /// Unscaled acceleration in Y-direction.
    #[inline]
    pub const fn y_unscaled(&self) -> i16 {
        self.y as i16 / self.factor.resolution()
    }

    /// Unscaled acceleration in Z-direction.
    #[inline]
    pub const fn z_unscaled(&self) -> i16 {
        self.z as i16 / self.factor.resolution()
    }

    /// Unscaled acceleration in X-, Y- and Z-directions.
    #[inline]
    pub const fn xyz_unscaled(&self) -> (i16, i16, i16) {
        (
            self.x as i16 / self.factor.resolution(),
            self.y as i16 / self.factor.resolution(),
            self.z as i16 / self.factor.resolution(),
        )
    }

    /// Acceleration in X-direction in m*g* (milli-*g*).
    #[inline]
    pub const fn x_mg(&self) -> i32 {
        self.x_unscaled() as i32 * self.factor.scaling()
    }

    /// Acceleration in Y-direction in m*g* (milli-*g*).
    #[inline]
    pub const fn y_mg(&self) -> i32 {
        self.y_unscaled() as i32 * self.factor.scaling()
    }

    /// Acceleration in Z-direction in m*g* (milli-*g*).
    #[inline]
    pub const fn z_mg(&self) -> i32 {
        self.z_unscaled() as i32 * self.factor.scaling()
    }

    /// Acceleration in X-, Y- and Z-directions in m*g* (milli-*g*).
    #[inline]
    pub const fn xyz_mg(&self) -> (i32, i32, i32) {
        let (x_unscaled, y_unscaled, z_unscaled) = self.xyz_unscaled();

        (
            x_unscaled as i32 * self.factor.scaling(),
            y_unscaled as i32 * self.factor.scaling(),
            z_unscaled as i32 * self.factor.scaling(),
        )
    }
}

/// A Magnetometer ID.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MagnetometerId {
    pub(crate) raw: u8,
    pub(crate) id: u8,
}

impl MagnetometerId {
    pub(crate) fn from_bits_truncate(raw: u8) -> Self {
        Self { raw, id: 0 }
    }

    /// Raw magnetometer ID.
    pub const fn raw(&self) -> u8 {
        self.raw
    }

    /// Check if the ID corresponds to the expected value.
    pub const fn is_correct(&self) -> bool {
        self.raw == self.id
    }
}

/// A magnetic field measurement.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct MagneticField {
    pub(crate) x: u16,
    pub(crate) y: u16,
    pub(crate) z: u16,
}

impl MagneticField {
    const SCALING_FACTOR: i32 = 150;

    /// Raw magnetic field in X-direction.
    #[inline]
    pub const fn x_raw(&self) -> u16 {
        self.x
    }

    /// Raw magnetic field in Y-direction.
    #[inline]
    pub const fn y_raw(&self) -> u16 {
        self.y
    }

    /// Raw magnetic field in Z-direction.
    #[inline]
    pub const fn z_raw(&self) -> u16 {
        self.z
    }

    /// Raw magnetic field in X-, Y- and Z-directions.
    #[inline]
    pub const fn xyz_raw(&self) -> (u16, u16, u16) {
        (self.x, self.y, self.z)
    }

    /// Unscaled magnetic field in X-direction.
    #[inline]
    pub const fn x_unscaled(&self) -> i16 {
        self.x as i16
    }

    /// Unscaled magnetic field in Y-direction.
    #[inline]
    pub const fn y_unscaled(&self) -> i16 {
        self.y as i16
    }

    /// Unscaled magnetic field in Z-direction.
    #[inline]
    pub const fn z_unscaled(&self) -> i16 {
        self.z as i16
    }

    /// Unscaled magnetic field in X-, Y- and Z-directions.
    #[inline]
    pub const fn xyz_unscaled(&self) -> (i16, i16, i16) {
        (self.x as i16, self.y as i16, self.z as i16)
    }

    /// Magnetic field in X-direction in nT (nano-Tesla).
    #[inline]
    pub const fn x_nt(&self) -> i32 {
        (self.x_unscaled() as i32) * Self::SCALING_FACTOR
    }

    /// Magnetic field in Y-direction in nT (nano-Tesla).
    #[inline]
    pub const fn y_nt(&self) -> i32 {
        (self.y_unscaled() as i32) * Self::SCALING_FACTOR
    }

    /// Magnetic field in Z-direction in nT (nano-Tesla).
    #[inline]
    pub const fn z_nt(&self) -> i32 {
        (self.z_unscaled() as i32) * Self::SCALING_FACTOR
    }

    /// Magnetic field in X-, Y- and Z-directions in nT (nano-Tesla).
    #[inline]
    pub const fn xyz_nt(&self) -> (i32, i32, i32) {
        (self.x_nt(), self.y_nt(), self.z_nt())
    }
}

bitflags! {
    #[derive(Default)]
    pub struct StatusFlags: u8 {
        const ZYXOR = 0b10000000;
        const ZOR   = 0b01000000;
        const YOR   = 0b00100000;
        const XOR   = 0b00010000;
        const ZYXDA = 0b00001000;
        const ZDA   = 0b00000100;
        const YDA   = 0b00000010;
        const XDA   = 0b00000001;
    }
}

/// Data status
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Status {
    flags: StatusFlags,
}

impl Status {
    pub(crate) const fn new(flags: StatusFlags) -> Self {
        Self { flags }
    }

    /// X-axis new data available.
    #[inline]
    pub const fn x_new_data(&self) -> bool {
        self.flags.contains(StatusFlags::XDA)
    }

    /// Y-axis new data available.
    #[inline]
    pub const fn y_new_data(&self) -> bool {
        self.flags.contains(StatusFlags::YDA)
    }

    /// Z-axis new data available.
    #[inline]
    pub const fn z_new_data(&self) -> bool {
        self.flags.contains(StatusFlags::ZDA)
    }

    /// X-, Y- and Z-axis new data available.
    #[inline]
    pub const fn xyz_new_data(&self) -> bool {
        self.flags.contains(StatusFlags::ZYXDA)
    }

    /// X-axis data overrun.
    #[inline]
    pub const fn x_overrun(&self) -> bool {
        self.flags.contains(StatusFlags::XOR)
    }

    /// Y-axis data overrun.
    #[inline]
    pub const fn y_overrun(&self) -> bool {
        self.flags.contains(StatusFlags::YOR)
    }

    /// Z-axis data overrun.
    #[inline]
    pub const fn z_overrun(&self) -> bool {
        self.flags.contains(StatusFlags::ZOR)
    }

    /// X-, Y- and Z-axis data overrun.
    #[inline]
    pub const fn xyz_overrun(&self) -> bool {
        self.flags.contains(StatusFlags::ZYXOR)
    }
}

/// Temperature sensor status
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TemperatureStatus {
    flags: StatusRegAuxA,
}

impl TemperatureStatus {
    pub(crate) const fn new(flags: StatusRegAuxA) -> Self {
        Self { flags }
    }

    /// Temperature data overrun.
    #[inline]
    pub const fn overrun(&self) -> bool {
        self.flags.contains(StatusRegAuxA::TOR)
    }

    /// Temperature new data available.
    #[inline]
    pub const fn new_data(&self) -> bool {
        self.flags.contains(StatusRegAuxA::TDA)
    }
}

/// A temperature measurement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Temperature {
    pub(crate) raw: u16,
}

impl Temperature {
    const DEFAULT: f32 = 25.0;

    /// Raw temperature.
    #[inline]
    pub const fn raw(&self) -> u16 {
        self.raw
    }

    /// Unscaled temperature.
    #[inline]
    pub const fn unscaled(&self) -> i16 {
        self.raw as i16
    }

    /// Temperature in °C.
    #[inline]
    pub fn degrees_celsius(&self) -> f32 {
        (self.unscaled() as f32) / 256.0 + Self::DEFAULT
    }
}
