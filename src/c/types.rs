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
