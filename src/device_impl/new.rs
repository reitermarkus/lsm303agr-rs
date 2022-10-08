use super::*;

macro_rules! impl_new {
    ($Lsm:ident, $($field:ident,)*) => {
        impl<I2C> $Lsm<I2cInterface<I2C>, mode::MagOneShot> {
            /// Create new instance of the LSM303AGR device communicating through I2C.
            pub fn new_with_i2c(i2c: I2C) -> Self {
                Self {
                    iface: I2cInterface { i2c },
                    $(
                        $field: Default::default(),
                    )*
                    accel_odr: None,
                    _mag_mode: PhantomData,
                }
            }
        }

        impl<I2C, MODE> $Lsm<I2cInterface<I2C>, MODE> {
            /// Destroy driver instance, return I2C bus.
            pub fn destroy(self) -> I2C {
                self.iface.i2c
            }
        }

        impl<SPI, CSXL, CSMAG> $Lsm<SpiInterface<SPI, CSXL, CSMAG>, mode::MagOneShot> {
            /// Create new instance of the LSM303AGR device communicating through SPI.
            pub fn new_with_spi(spi: SPI, chip_select_accel: CSXL, chip_select_mag: CSMAG) -> Self {
                Self {
                    iface: SpiInterface {
                        spi,
                        cs_xl: chip_select_accel,
                        cs_mag: chip_select_mag,
                    },
                    $(
                        $field: Default::default(),
                    )*
                    accel_odr: None,
                    _mag_mode: PhantomData,
                }
            }
        }

        impl<SPI, CSXL, CSMAG, MODE> $Lsm<SpiInterface<SPI, CSXL, CSMAG>, MODE> {
            /// Destroy driver instance, return SPI bus instance and chip select pin.
            pub fn destroy(self) -> (SPI, CSXL, CSMAG) {
                (self.iface.spi, self.iface.cs_xl, self.iface.cs_mag)
            }
        }
    }
}

impl_new!(
    Lsm303agr,
    ctrl_reg1_a,
    ctrl_reg3_a,
    ctrl_reg4_a,
    ctrl_reg5_a,
    cfg_reg_a_m,
    cfg_reg_b_m,
    cfg_reg_c_m,
    temp_cfg_reg_a,
    fifo_ctrl_reg_a,
);

impl_new!(
    Lsm303c,
    ctrl_reg1_a,
    ctrl_reg2_a,
    ctrl_reg3_a,
    ctrl_reg4_a,
    ctrl_reg1_m,
    ctrl_reg2_m,
    ctrl_reg3_m,
    ctrl_reg5_m,
    fifo_ctrl,
);
