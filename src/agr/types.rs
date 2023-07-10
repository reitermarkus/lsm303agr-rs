use crate::types::{AccelerationFactor, ResolutionFactor, ScalingFactor};

/// Accelerometer output data rate
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccelOutputDataRate {
    /// 1 Hz (all modes)
    Hz1,
    /// 10 Hz all modes)
    Hz10,
    /// 25 Hz (all modes)
    Hz25,
    /// 50 Hz (all modes)
    Hz50,
    /// 100 Hz (all modes)
    Hz100,
    /// 200 Hz (all modes)
    Hz200,
    /// 400 Hz (all modes)
    Hz400,
    /// 1.344 kHz (only normal and high-resolution modes)
    Khz1_344,
    /// 1.620 kHz (only low-power mode)
    Khz1_620LowPower,
    /// 5.376 kHz (only low-power mode)
    Khz5_376LowPower,
}

impl AccelOutputDataRate {
    /// Create an `AccelOutputDataRate` with the given frequency in Hertz.
    pub const fn from_hertz(hz: u16) -> Option<Self> {
        Some(match hz {
            1 => Self::Hz1,
            10 => Self::Hz10,
            25 => Self::Hz25,
            50 => Self::Hz50,
            100 => Self::Hz100,
            200 => Self::Hz200,
            400 => Self::Hz400,
            1344 => Self::Khz1_344,
            1620 => Self::Khz1_620LowPower,
            5376 => Self::Khz5_376LowPower,
            _ => return None,
        })
    }

    /// Get the frequency in Hertz.
    pub const fn to_hertz(&self) -> u16 {
        match self {
            Self::Hz1 => 1,
            Self::Hz10 => 10,
            Self::Hz25 => 25,
            Self::Hz50 => 50,
            Self::Hz100 => 100,
            Self::Hz200 => 200,
            Self::Hz400 => 400,
            Self::Khz1_344 => 1344,
            Self::Khz1_620LowPower => 1620,
            Self::Khz5_376LowPower => 5376,
        }
    }

    /// 1/ODR ms
    #[inline]
    pub(crate) const fn turn_on_time_us_frac_1(&self) -> u32 {
        match self {
            Self::Hz1 => 1000,
            Self::Hz10 => 100,
            Self::Hz25 => 40,
            Self::Hz50 => 20,
            Self::Hz100 => 10,
            Self::Hz200 => 5,
            Self::Hz400 => 3,            //  2.5
            Self::Khz1_344 => 1,         // ~0.7
            Self::Khz1_620LowPower => 1, // ~0.6
            Self::Khz5_376LowPower => 1, // ~0.2
        }
    }

    /// 7/ODR ms
    #[inline]
    pub(crate) const fn turn_on_time_us_frac_7(&self) -> u32 {
        match self {
            Self::Hz1 => 7000,
            Self::Hz10 => 700,
            Self::Hz25 => 280,
            Self::Hz50 => 140,
            Self::Hz100 => 70,
            Self::Hz200 => 35,
            Self::Hz400 => 18,           // 17.5
            Self::Khz1_344 => 6,         // ~5.2
            Self::Khz1_620LowPower => 5, // ~4.3
            Self::Khz5_376LowPower => 2, // ~1.3
        }
    }
}

/// Accelerometer mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccelMode {
    /// Power down
    PowerDown,
    /// Low power (8-bit)
    LowPower,
    /// Normal mode (10-bit)
    Normal,
    /// High resolution (12-bit)
    HighResolution,
}

impl AccelMode {
    #[inline]
    pub(crate) const fn turn_on_time_us(&self, odr: AccelOutputDataRate) -> u32 {
        match self {
            Self::PowerDown => 0,
            Self::LowPower => 1000,
            Self::Normal => 1600,
            Self::HighResolution => odr.turn_on_time_us_frac_7(),
        }
    }

    #[inline]
    pub(crate) const fn change_time_us(&self, other: AccelMode, odr: AccelOutputDataRate) -> u32 {
        match (self, other) {
            (Self::HighResolution, Self::LowPower) => odr.turn_on_time_us_frac_1(),
            (Self::HighResolution, Self::Normal) => odr.turn_on_time_us_frac_1(),
            (Self::Normal, Self::LowPower) => odr.turn_on_time_us_frac_1(),
            (Self::Normal, Self::HighResolution) => odr.turn_on_time_us_frac_7(),
            (Self::LowPower, Self::Normal) => odr.turn_on_time_us_frac_1(),
            (Self::LowPower, Self::HighResolution) => odr.turn_on_time_us_frac_7(),
            (Self::PowerDown, new_mode) => new_mode.turn_on_time_us(odr),
            _ => 0,
        }
    }

    pub(crate) const fn factor(&self, scale: AccelScale) -> AccelerationFactor {
        AccelerationFactor::new(self.resolution_factor(), self.scaling_factor(scale))
    }

    const fn resolution_factor(&self) -> ResolutionFactor {
        use ResolutionFactor::*;

        match self {
            Self::PowerDown => R1,
            Self::HighResolution => R16,
            Self::Normal => R64,
            Self::LowPower => R256,
        }
    }

    const fn scaling_factor(&self, scale: AccelScale) -> ScalingFactor {
        use ScalingFactor::*;

        match self {
            Self::PowerDown => S0,
            Self::HighResolution => match scale {
                AccelScale::G2 => S1,
                AccelScale::G4 => S2,
                AccelScale::G8 => S4,
                AccelScale::G16 => S8,
            },
            Self::Normal => match scale {
                AccelScale::G2 => S4,
                AccelScale::G4 => S8,
                AccelScale::G8 => S16,
                AccelScale::G16 => S32,
            },
            Self::LowPower => match scale {
                AccelScale::G2 => S16,
                AccelScale::G4 => S32,
                AccelScale::G8 => S64,
                AccelScale::G16 => S128,
            },
        }
    }
}

/// Accelerometer scaling factor
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccelScale {
    /// ±2*g*
    G2,
    /// ±4*g*
    G4,
    /// ±8*g*
    G8,
    /// ±16*g*
    G16,
}

/// Magnetometer output data rate
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MagOutputDataRate {
    /// 10 Hz
    Hz10,
    /// 20 Hz
    Hz20,
    /// 50 Hz
    Hz50,
    /// 100 Hz
    Hz100,
}

impl MagOutputDataRate {
    /// Create an `MagOutputDataRate` with the given frequency in Hertz.
    pub const fn from_hertz(hz: u16) -> Option<Self> {
        Some(match hz {
            10 => Self::Hz10,
            20 => Self::Hz20,
            50 => Self::Hz50,
            100 => Self::Hz100,
            _ => return None,
        })
    }

    /// Get the frequency in Hertz.
    pub const fn to_hertz(&self) -> u16 {
        match self {
            Self::Hz10 => 10,
            Self::Hz20 => 20,
            Self::Hz50 => 50,
            Self::Hz100 => 100,
        }
    }

    /// 1/ODR ms
    pub(crate) const fn turn_on_time_us_frac_1(&self) -> u32 {
        match self {
            Self::Hz10 => 100,
            Self::Hz20 => 50,
            Self::Hz50 => 20,
            Self::Hz100 => 10,
        }
    }
}

/// Magnetometer mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MagMode {
    /// Low-power mode
    LowPower,
    /// High resolution mode
    HighResolution,
}

impl Default for MagMode {
    fn default() -> Self {
        Self::HighResolution
    }
}

impl MagMode {
    pub(crate) const fn turn_on_time_us(&self) -> u32 {
        match self {
            Self::LowPower => 9400,
            Self::HighResolution => 6400,
        }
    }
}

/// A FIFO mode.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FifoMode {
    /// Bypass mode
    Bypass,
    /// FIFO mode
    Fifo,
    /// Stream mode
    Stream,
    /// Stream-to-FIFO mode
    StreamToFifo,
}

/// An interrupt.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Interrupt {
    /// AOI1 interrupt on INT1 pin.
    Aoi1,
    /// AOI2 interrupt on INT1 pin.
    Aoi2,
    /// CLICK interrupt on INT1 pin.
    Click,
    /// DRDY1 interrupt on INT1 pin.
    DataReady1,
    /// DRDY2 interrupt on INT1 pin.
    DataReady2,
    /// FIFO overrun interrupt on INT1 pin.
    FifoOverrun,
    /// FIFO watermark interrupt on INT1 pin.
    FifoWatermark,
}
