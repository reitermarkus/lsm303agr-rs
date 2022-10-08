pub(crate) use crate::types::{AccelerometerId, MagnetometerId, StatusFlags};

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
    pub(crate) const fn resolution_factor(&self) -> i16 {
        match self {
            Self::PowerDown => 1,
            Self::HighResolution => 1 << 4,
            Self::Normal => 1 << 6,
        }
    }

    pub(crate) const fn scaling_factor(&self, scale: AccelScale) -> u8 {
        match self {
            Self::PowerDown => 0,
            Self::HighResolution => scale as u8 / 2,
            Self::Normal => scale as u8 * 2,
        }
    }
}

/// Accelerometer scaling factor
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccelScale {
    /// ±2*g*
    G2 = 2,
    /// ±4*g*
    G4 = 4,
    /// ±8*g*
    G8 = 8,
}

/// Magnetometer mode
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
