use core::marker::PhantomData;

// mod mag_mode_change;
pub(crate) mod register;
mod types;
pub use types::*;

use register::{
    CtrlReg1A, CtrlReg1M, CtrlReg2A, CtrlReg2M, CtrlReg3A, CtrlReg3M, CtrlReg4A, CtrlReg4M,
    CtrlReg5M, CtrlReg6A, FifoCtrl, IgCfg1A, IntCfgM,
};

pub(crate) const ACC_ADDR: u8 = 0b001_1101;
pub(crate) const MAG_ADDR: u8 = 0b001_1110;

/// An LSM303C accelerometer and magnetometer.
#[derive(Debug)]
pub struct Lsm303c<DI, MODE> {
    /// Digital interface: I2C or SPI
    pub(crate) iface: DI,
    pub(crate) ctrl_reg1_a: CtrlReg1A,
    pub(crate) ctrl_reg2_a: CtrlReg2A,
    pub(crate) ctrl_reg3_a: CtrlReg3A,
    pub(crate) ctrl_reg4_a: CtrlReg4A,
    pub(crate) ctrl_reg6_a: CtrlReg6A,
    pub(crate) ig_cfg1_a: IgCfg1A,
    pub(crate) ctrl_reg1_m: CtrlReg1M,
    pub(crate) ctrl_reg2_m: CtrlReg2M,
    pub(crate) ctrl_reg3_m: CtrlReg3M,
    pub(crate) ctrl_reg4_m: CtrlReg4M,
    pub(crate) ctrl_reg5_m: CtrlReg5M,
    pub(crate) fifo_ctrl: FifoCtrl,
    pub(crate) int_reg_m: IntCfgM,
    pub(crate) accel_odr: Option<AccelOutputDataRate>,
    pub(crate) _mag_mode: PhantomData<MODE>,
}
