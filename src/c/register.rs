#![allow(unused)]

use crate::reg::{register, RegRead};
use crate::types::{AccelerometerId, MagnetometerId, StatusFlags, Temperature};

use super::types::{
    AccelOutputDataRate, AccelScale, FifoMode, Interrupt, MagMode, MagOutputDataRate,
};

register! {
  /// WHO_AM_I_A
  pub type WhoAmIA: 0x0F = AccelerometerId;
}

impl WhoAmIA {
    pub(crate) const ID: u8 = 0b01000001;
}

register! {
  /// CTRL_REG1_A
  pub struct CtrlReg1A: 0x20 {
    const HR = 0b10000000;
    const ODR2 = 0b01000000;
    const ODR1 = 0b00100000;
    const ODR0 = 0b00010000;
    const BDU = 0b00001000;
    const ZEN  = 0b00000100;
    const YEN  = 0b00000010;
    const XEN  = 0b00000001;

    const ODR = Self::ODR2.bits | Self::ODR1.bits | Self::ODR0.bits;
  }
}

impl Default for CtrlReg1A {
    fn default() -> Self {
        Self::ZEN | Self::YEN | Self::XEN
    }
}

impl CtrlReg1A {
    pub const fn with_odr(self, odr: AccelOutputDataRate) -> Self {
        let reg = self.difference(Self::ODR);

        match odr {
            AccelOutputDataRate::Hz10 => reg.union(Self::ODR0),
            AccelOutputDataRate::Hz50 => reg.union(Self::ODR1),
            AccelOutputDataRate::Hz100 => reg.union(Self::ODR1).union(Self::ODR0),
            AccelOutputDataRate::Hz200 => reg.union(Self::ODR2),
            AccelOutputDataRate::Hz400 => reg.union(Self::ODR2).union(Self::ODR0),
            AccelOutputDataRate::Hz800 => reg.union(Self::ODR2).union(Self::ODR1).union(Self::ODR0),
        }
    }
}

register! {
  /// CTRL_REG2_A
  #[derive(Default)]
  pub struct CtrlReg2A: 0x21 {
    const DFC1  = 0b01000000;
    const DFC0  = 0b00100000;
    const HPM1  = 0b00010000;
    const HPM0  = 0b00001000;
    const FDS   = 0b00000100;
    const HPIS2 = 0b00000010;
    const HPIS1 = 0b00000001;

    const DFC = Self::DFC1.bits | Self::DFC0.bits;
  }
}

register! {
  /// CTRL_REG3_A
  #[derive(Default)]
  pub struct CtrlReg3A: 0x22 {
    const FIFO_EN      = 0b10000000;
    const STOP_FTH     = 0b01000000;
    const INT_XL_INACT = 0b00100000;
    const INT_XL_IG2   = 0b00010000;
    const INT_XL_IG1   = 0b00001000;
    const INT_XL_OVR   = 0b00000100;
    const INT_XL_FTH   = 0b00000010;
    const INT_XL_DRDY  = 0b00000001;
  }
}

impl CtrlReg3A {
    pub const fn with_interrupt(self, interrupt: Interrupt) -> Self {
        match interrupt {
            Interrupt::Inactivity => self.union(Self::INT_XL_INACT),
            Interrupt::Ig2 => self.union(Self::INT_XL_IG2),
            Interrupt::Ig1 => self.union(Self::INT_XL_IG1),
            Interrupt::FifoOverrun => self.union(Self::INT_XL_OVR),
            Interrupt::FifoThreshold => self.union(Self::INT_XL_FTH),
            Interrupt::DataReady => self.union(Self::INT_XL_DRDY),
        }
    }

    pub const fn without_interrupt(self, interrupt: Interrupt) -> Self {
        match interrupt {
            Interrupt::Inactivity => self.difference(Self::INT_XL_INACT),
            Interrupt::Ig2 => self.difference(Self::INT_XL_IG2),
            Interrupt::Ig1 => self.difference(Self::INT_XL_IG1),
            Interrupt::FifoOverrun => self.difference(Self::INT_XL_OVR),
            Interrupt::FifoThreshold => self.difference(Self::INT_XL_FTH),
            Interrupt::DataReady => self.difference(Self::INT_XL_DRDY),
        }
    }
}

register! {
  /// CTRL_REG4_A
  #[derive(Default)]
  pub struct CtrlReg4A: 0x23 {
    const BW2          = 0b10000000;
    const BW1          = 0b01000000;
    const FS1          = 0b00100000;
    const FS0          = 0b00010000;
    const BW_SCALE_ODR = 0b00001000;
    const IF_ADD_INC   = 0b00000100;
    const I2C_DISABLE  = 0b00000010;
    const SIM          = 0b00000001;

    const FS = Self::FS1.bits | Self::FS0.bits;
  }
}

impl CtrlReg4A {
    pub const fn scale(&self) -> AccelScale {
        match self.intersection(Self::FS).bits() >> 4 {
            0b00 => AccelScale::G2,
            0b10 => AccelScale::G4,
            0b11 => AccelScale::G8,
            _ => unreachable!(),
        }
    }

    pub const fn with_scale(self, scale: AccelScale) -> Self {
        match scale {
            AccelScale::G2 => self.difference(Self::FS),
            AccelScale::G4 => self.union(Self::FS1).difference(Self::FS0),
            AccelScale::G8 => self.union(Self::FS),
        }
    }
}

register! {
  /// CTRL_REG5_A
  #[derive(Default)]
  pub struct CtrlReg5A: 0x24 {
    const DEBUG      = 0b10000000;
    const SOFT_RESET = 0b01000000;
    const DEC1       = 0b00100000;
    const DEC0       = 0b00010000;
    const ST2        = 0b00001000;
    const ST1        = 0b00000100;
    const H_LACTIVE  = 0b00000010;
    const PP_OD      = 0b00000001;
  }
}

register! {
  /// CTRL_REG6_A
  #[derive(Default)]
  pub struct CtrlReg6A: 0x25 {
    const BOOT      = 0b10000000;
  }
}

register! {
  /// CTRL_REG7_A
  #[derive(Default)]
  pub struct CtrlReg7A: 0x26 {
    const DCRM2  = 0b00100000;
    const DCRM1  = 0b00010000;
    const LIR2   = 0b00001000;
    const LIR1   = 0b00000100;
    const D4_IG2 = 0b00000010;
    const D4_IG1 = 0b00000001;
  }
}

register! {
  /// STATUS_REG_A
  pub type StatusRegA: 0x27 = StatusFlags;
}

register! {
  /// FIFO_CTRL
  #[derive(Default)]
  pub struct FifoCtrl: 0x2E {
    const FMODE2 = 0b10000000;
    const FMODE1 = 0b01000000;
    const FMODE0 = 0b00100000;
    const FTH4   = 0b00010000;
    const FTH3   = 0b00001000;
    const FTH2   = 0b00000100;
    const FTH1   = 0b00000010;
    const FTH0   = 0b00000001;

    const FMODE = Self::FMODE2.bits | Self::FMODE1.bits | Self::FMODE0.bits;
    const FTH = Self::FTH4.bits | Self::FTH3.bits | Self::FTH2.bits | Self::FTH1.bits | Self::FTH0.bits;
  }
}

impl FifoCtrl {
    pub const fn with_mode(self, mode: FifoMode) -> Self {
        match mode {
            FifoMode::Bypass => self.difference(Self::FMODE),
            FifoMode::Fifo => self
                .difference(Self::FMODE2)
                .difference(Self::FMODE1)
                .union(Self::FMODE0),
            FifoMode::Stream => self
                .difference(Self::FMODE2)
                .union(Self::FMODE1)
                .difference(Self::FMODE0),
            FifoMode::StreamToFifo => self
                .difference(Self::FMODE2)
                .union(Self::FMODE1)
                .union(Self::FMODE0),
            FifoMode::BypassToStream => self
                .union(Self::FMODE2)
                .difference(Self::FMODE1)
                .difference(Self::FMODE0),
            FifoMode::BypassToFifo => self.union(Self::FMODE),
        }
    }

    pub const fn with_full_threshold(self, n: u8) -> Self {
        let n = if n > Self::FTH.bits {
            Self::FTH.bits
        } else {
            n
        };
        self.difference(Self::FTH)
            .union(Self::from_bits_truncate(n))
    }
}

register! {
  /// FIFO_SRC
  pub struct FifoSrc: 0x2F {
    const FTH    = 0b10000000;
    const OVR    = 0b01000000;
    const EMPTY  = 0b00100000;
    const FSS4   = 0b00010000;
    const FSS3   = 0b00001000;
    const FSS2   = 0b00000100;
    const FSS1   = 0b00000010;
    const FSS0   = 0b00000001;
  }
}

register! {
  /// IG_CFG1_A
  #[derive(Default)]
  pub struct IgCfg1A: 0x30 {
    const AOI       = 0b10000000;
    const D6        = 0b01000000;
    const ZHIE      = 0b00100000;
    const ZLIE      = 0b00010000;
    const YHIE      = 0b00001000;
    const YLIE      = 0b00000100;
    const XHIE      = 0b00000010;
    const XLIE      = 0b00000001;
  }
}

register! {
  /// IG_SRC1_A
  #[derive(Default)]
  pub struct IgSrc1A: 0x31 {
    const IA = 0b01000000;
    const ZH = 0b00100000;
    const ZL = 0b00010000;
    const YH = 0b00001000;
    const YL = 0b00000100;
    const XH = 0b00000010;
    const XL = 0b00000001;
  }
}

register! {
  /// WHO_AM_I_M
  pub type WhoAmIM: 0x0F = MagnetometerId;
}

impl WhoAmIM {
    pub(crate) const ID: u8 = 0b00111101;
}

register! {
  /// CTRL_REG1_M
  pub struct CtrlReg1M: 0x20 {
    const TEMP_EN = 0b10000000;
    const OM1     = 0b01000000;
    const OM0     = 0b00100000;
    const DO2     = 0b00010000;
    const DO1     = 0b00001000;
    const DO0     = 0b00000100;
    const ST      = 0b00000001;

    const OM = Self::OM1.bits() | Self::OM0.bits();
    const DO = Self::DO2.bits() | Self::DO1.bits() | Self::DO0.bits();
  }
}

impl Default for CtrlReg1M {
    fn default() -> Self {
        Self::DO2
    }
}

impl CtrlReg1M {
    pub const fn xy_mode(&self) -> MagMode {
        match (self.intersects(Self::OM1), self.intersects(Self::OM0)) {
            (false, false) => MagMode::LowPower,
            (false, true) => MagMode::MediumPerformance,
            (true, false) => MagMode::HighPerformance,
            (true, true) => MagMode::UltraHighPerformance,
        }
    }

    pub const fn with_xy_mode(self, mode: MagMode) -> Self {
        let this = self.difference(Self::OM);

        match mode {
            MagMode::LowPower => this,
            MagMode::MediumPerformance => self.union(Self::OM0),
            MagMode::HighPerformance => self.union(Self::OM1),
            MagMode::UltraHighPerformance => self.union(Self::OM1).union(Self::OM0),
        }
    }

    pub const fn odr(&self) -> MagOutputDataRate {
        match (
            self.intersects(Self::DO2),
            self.intersects(Self::DO1),
            self.intersects(Self::DO0),
        ) {
            (false, false, false) => MagOutputDataRate::Hz0_625,
            (false, false, true) => MagOutputDataRate::Hz1_25,
            (false, true, false) => MagOutputDataRate::Hz2_5,
            (false, true, true) => MagOutputDataRate::Hz5,
            (true, false, false) => MagOutputDataRate::Hz10,
            (true, false, true) => MagOutputDataRate::Hz20,
            (true, true, false) => MagOutputDataRate::Hz40,
            (true, true, true) => MagOutputDataRate::Hz80,
        }
    }

    pub const fn with_odr(self, odr: MagOutputDataRate) -> Self {
        let this = self.difference(Self::DO);

        match odr {
            MagOutputDataRate::Hz0_625 => this,
            MagOutputDataRate::Hz1_25 => this.union(Self::DO0),
            MagOutputDataRate::Hz2_5 => this.union(Self::DO1),
            MagOutputDataRate::Hz5 => this.union(Self::DO1).union(Self::DO0),
            MagOutputDataRate::Hz10 => this.union(Self::DO2),
            MagOutputDataRate::Hz20 => this.union(Self::DO2).union(Self::DO0),
            MagOutputDataRate::Hz40 => this.union(Self::DO2).union(Self::DO1),
            MagOutputDataRate::Hz80 => this.union(Self::DO2).union(Self::DO1).union(Self::DO0),
        }
    }
}

register! {
  /// CTRL_REG2_M
  #[derive(Default)]
  pub struct CtrlReg2M: 0x21 {
    const FS1      = 0b01000000;
    const FS0      = 0b00100000;
    const REBOOT   = 0b00001000;
    const SOFT_RST = 0b00000100;

    const FS = Self::FS1.bits | Self::FS0.bits;
  }
}

register! {
  /// CTRL_REG3_M
  pub struct CtrlReg3M: 0x22 {
    const I2C_DISABLE = 0b10000000;
    const LP          = 0b00100000;
    const SIM         = 0b00000100;
    const MD1         = 0b00000010;
    const MD0         = 0b00000001;

    const MD = Self::MD1.bits | Self::MD0.bits;
  }
}

impl Default for CtrlReg3M {
    fn default() -> Self {
        Self::MD
    }
}

impl CtrlReg3M {
    pub const fn continuous_mode(self) -> Self {
        self.difference(Self::MD1).difference(Self::MD0) // 0b00
    }

    pub const fn is_single_mode(&self) -> bool {
        !self.contains(Self::MD1) && self.contains(Self::MD0)
    }

    pub const fn single_mode(self) -> Self {
        self.difference(Self::MD1).union(Self::MD0) // 0b01
    }

    #[cfg(test)]
    pub const fn is_idle_mode(&self) -> bool {
        self.contains(Self::MD1) // 0b10 or 0b11
    }

    pub const fn idle_mode(self) -> Self {
        self.union(Self::MD1).union(Self::MD0) // 0b11
    }
}

register! {
  /// CTRL_REG4_M
  #[derive(Default)]
  pub struct CtrlReg4M: 0x24 {
    const OMZ1 = 0b00001000;
    const OMZ0 = 0b00000100;
    const BLE  = 0b00000010;

    const OMZ = Self::OMZ1.bits | Self::OMZ0.bits;
  }
}

impl CtrlReg4M {
    pub const fn z_mode(&self) -> MagMode {
        match (self.intersects(Self::OMZ1), self.intersects(Self::OMZ0)) {
            (false, false) => MagMode::LowPower,
            (false, true) => MagMode::MediumPerformance,
            (true, false) => MagMode::HighPerformance,
            (true, true) => MagMode::UltraHighPerformance,
        }
    }

    pub const fn with_z_mode(self, mode: MagMode) -> Self {
        let this = self.difference(Self::OMZ);

        match mode {
            MagMode::LowPower => this,
            MagMode::MediumPerformance => self.union(Self::OMZ0),
            MagMode::HighPerformance => self.union(Self::OMZ1),
            MagMode::UltraHighPerformance => self.union(Self::OMZ1).union(Self::OMZ0),
        }
    }
}

register! {
  /// CTRL_REG5_M
  #[derive(Default)]
  pub struct CtrlReg5M: 0x24 {
    const BDU = 0b01000000;
  }
}

register! {
  /// STATUS_REG_M
  pub type StatusRegM: 0x27 = StatusFlags;
}

/// TEMP_L_M (register `0x2F`)
pub struct TempLM;

impl RegRead<u16> for TempLM {
    type Output = Temperature;

    /// TEMP_L_M
    const ADDR: u8 = 0x2F;

    #[inline]
    fn from_data(data: u16) -> Self::Output {
        Temperature { raw: data }
    }
}

register! {
  /// INT_CFG_M
  pub struct IntCfgM: 0x30 {
    const XIEN = 0b10000000;
    const YIEN = 0b01000000;
    const ZIEN = 0b00100000;

    const _1   = 0b00001000;

    const IEA  = 0b00000100;
    const IEL  = 0b00000010;
    const IEN  = 0b00000001;
  }
}

impl Default for IntCfgM {
    fn default() -> Self {
        Self::_1
    }
}
