use core::marker::PhantomData;

pub(crate) mod register;
use register::{
    CfgRegAM, CfgRegBM, CfgRegCM, CtrlReg1A, CtrlReg3A, CtrlReg4A, CtrlReg5A, FifoCtrlRegA,
    TempCfgRegA,
};

mod types;
pub use types::{
    AccelMode, AccelOutputDataRate, AccelScale, FifoMode, Interrupt, MagMode, MagOutputDataRate,
};

pub(crate) const ACC_ADDR: u8 = 0b001_1001;
pub(crate) const MAG_ADDR: u8 = 0b001_1110;

/// An LSM303AGR accelerometer and magnetometer.
#[derive(Debug)]
pub struct Lsm303agr<DI, MODE> {
    /// Digital interface: I2C or SPI
    pub(crate) iface: DI,
    pub(crate) ctrl_reg1_a: CtrlReg1A,
    pub(crate) ctrl_reg3_a: CtrlReg3A,
    pub(crate) ctrl_reg4_a: CtrlReg4A,
    pub(crate) ctrl_reg5_a: CtrlReg5A,
    pub(crate) cfg_reg_a_m: CfgRegAM,
    pub(crate) cfg_reg_b_m: CfgRegBM,
    pub(crate) cfg_reg_c_m: CfgRegCM,
    pub(crate) temp_cfg_reg_a: TempCfgRegA,
    pub(crate) fifo_ctrl_reg_a: FifoCtrlRegA,
    pub(crate) accel_odr: Option<AccelOutputDataRate>,
    pub(crate) _mag_mode: PhantomData<MODE>,
}
