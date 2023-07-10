use crate::types::{AccelerationFactor, ResolutionFactor, ScalingFactor};

/// Accelerometer mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccelMode {
    /// Power down
    PowerDown,
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
            Self::Normal => 1600,
            Self::HighResolution => odr.turn_on_time_us_frac_7(),
        }
    }

    #[inline]
    pub(crate) const fn change_time_us(&self, other: AccelMode, odr: AccelOutputDataRate) -> u32 {
        match (self, other) {
            (Self::HighResolution, Self::Normal) => odr.turn_on_time_us_frac_1(),
            (Self::Normal, Self::HighResolution) => odr.turn_on_time_us_frac_7(),
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
            },
            Self::Normal => match scale {
                AccelScale::G2 => S4,
                AccelScale::G4 => S8,
                AccelScale::G8 => S16,
            },
        }
    }
}

/// Accelerometer output data rate
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccelOutputDataRate {
    /// 10 Hz
    Hz10,
    /// 50 Hz
    Hz50,
    /// 100 Hz
    Hz100,
    /// 200 Hz
    Hz200,
    /// 400 Hz
    Hz400,
    /// 800 Hz
    Hz800,
}

impl AccelOutputDataRate {
    /// Create an `AccelOutputDataRate` with the given frequency in Hertz.
    pub const fn from_hertz(hz: u16) -> Option<Self> {
        Some(match hz {
            10 => Self::Hz10,
            50 => Self::Hz50,
            100 => Self::Hz100,
            200 => Self::Hz200,
            400 => Self::Hz400,
            800 => Self::Hz800,
            _ => return None,
        })
    }

    /// Get the frequency in Hertz.
    pub const fn to_hertz(&self) -> u16 {
        match self {
            Self::Hz10 => 10,
            Self::Hz50 => 50,
            Self::Hz100 => 100,
            Self::Hz200 => 200,
            Self::Hz400 => 400,
            Self::Hz800 => 800,
        }
    }

    /// 1/ODR ms
    #[inline]
    pub(crate) const fn turn_on_time_us_frac_1(&self) -> u32 {
        match self {
            Self::Hz10 => 100,
            Self::Hz50 => 20,
            Self::Hz100 => 10,
            Self::Hz200 => 5,
            Self::Hz400 => 3, //  2.5
            Self::Hz800 => 2, // 1.25
        }
    }

    /// 7/ODR ms
    #[inline]
    pub(crate) const fn turn_on_time_us_frac_7(&self) -> u32 {
        match self {
            Self::Hz10 => 700,
            Self::Hz50 => 140,
            Self::Hz100 => 70,
            Self::Hz200 => 35,
            Self::Hz400 => 18, // 17.5
            Self::Hz800 => 9,  // 8.75
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
}

/// Magnetometer mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MagMode {
    /// Low-power mode
    LowPower,
    /// Medium-performance mode
    MediumPerformance,
    /// High-performance mode
    HighPerformance,
    /// Ultra-high performance mode
    UltraHighPerformance,
}

/// Magnetometer output data rate
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MagOutputDataRate {
    /// 0.625 Hz
    Hz0_625,
    /// 1.25 Hz
    Hz1_25,
    /// 2.5 Hz
    Hz2_5,
    /// 5 Hz
    Hz5,
    /// 10 Hz
    Hz10,
    /// 20 Hz
    Hz20,
    /// 40 Hz
    Hz40,
    /// 80 Hz
    Hz80,
}

impl MagOutputDataRate {
    /// Create an `MagOutputDataRate` with the given frequency in Hertz.
    pub fn from_hertz(hz: f32) -> Option<Self> {
        Some(if hz == 0.625 {
            Self::Hz0_625
        } else if hz == 1.25 {
            Self::Hz1_25
        } else if hz == 2.5 {
            Self::Hz2_5
        } else if hz == 5.0 {
            Self::Hz5
        } else if hz == 10.0 {
            Self::Hz10
        } else if hz == 20.0 {
            Self::Hz20
        } else if hz == 40.0 {
            Self::Hz40
        } else if hz == 80.0 {
            Self::Hz80
        } else {
            return None;
        })
    }

    /// Get the frequency in Hertz.
    pub const fn to_hertz(&self) -> f32 {
        match self {
            Self::Hz0_625 => 0.625,
            Self::Hz1_25 => 1.25,
            Self::Hz2_5 => 2.5,
            Self::Hz5 => 5.0,
            Self::Hz10 => 10.0,
            Self::Hz20 => 20.0,
            Self::Hz40 => 40.0,
            Self::Hz80 => 80.0,
        }
    }

    /// 1/ODR ms
    #[inline]
    pub(crate) const fn turn_on_time_us_frac_1(&self) -> u32 {
        match self {
            Self::Hz0_625 => 1600,
            Self::Hz1_25 => 800,
            Self::Hz2_5 => 400,
            Self::Hz5 => 200,
            Self::Hz10 => 100,
            Self::Hz20 => 50,
            Self::Hz40 => 25,
            Self::Hz80 => 13, // 12.5
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
    /// Bypass-to-Stream mode
    BypassToStream,
    /// Bypass-to-FIFO mode
    BypassToFifo,
}

/// An interrupt.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Interrupt {
    /// Inactivity interrupt on `INT_XL` pin.
    Inactivity,
    /// Interrupt generator 2 on `INT_XL` pin.
    Ig2,
    /// Interrupt generator 1 on `INT_XL` pin.
    Ig1,
    /// FIFO overrun interrupt on `INT_XL` pin.
    FifoOverrun,
    /// FIFO threshold signal on `INT_XL` pin.
    FifoThreshold,
    /// Data ready signal on `INT_XL` pin.
    DataReady,
}
