use embedded_hal::blocking::delay::DelayUs;

use crate::{
    agr::{self, Lsm303agr},
    c::{self, Lsm303c},
    interface::{ReadData, WriteData},
    mode,
    register::{CfgRegAM, CfgRegBM},
    Error, MagneticField,
};

macro_rules! impl_mag {
    ($Lsm:ident, $MODE:ty, $ODR:ty, $power_mode_reg_field:ident, $conversion_mode_reg_field:ident $(, $offset_cancellation_field:ident: $offset_cancellation_reg:ident)?) => {
        impl<DI, CommE, PinE, MODE> $Lsm<DI, MODE>
        where
            DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
        {
            /// Set magnetometer power/resolution mode and output data rate.
            ///
            #[doc = include_str!("delay.md")]
            pub fn set_mag_mode_and_odr<D: DelayUs<u32>>(
                &mut self,
                delay: &mut D,
                mode: $MODE,
                odr: $ODR,
            ) -> Result<(), Error<CommE, PinE>> {
                let reg = self.$power_mode_reg_field;

                #[allow(unused)]
                let old_mode = reg.mode();
                #[allow(unused)]
                let old_odr = reg.odr();

                let reg = reg.with_mode(mode).with_odr(odr);
                self.iface.write_mag_register(reg)?;
                self.$power_mode_reg_field = reg;

                $(
                    let offset_cancellation = self.$offset_cancellation_field.offset_cancellation();
                    if old_mode != mode {
                        delay.delay_us(reg.turn_on_time_us(offset_cancellation));
                    } else if old_odr != odr && offset_cancellation {
                        // Mode did not change, so only wait for 1/ODR ms.
                        delay.delay_us(odr.turn_on_time_us_frac_1());
                    }
                )*

                drop(delay);

                Ok(())
            }

            /// Get magnetometer power/resolution mode.
            pub fn get_mag_mode(&self) -> $MODE {
                self.$power_mode_reg_field.mode()
            }
        }

        impl<DI, CommE, PinE> $Lsm<DI, mode::MagContinuous>
        where
            DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
        {
            /// Get the measured magnetic field.
            pub fn magnetic_field(&mut self) -> Result<MagneticField, Error<CommE, PinE>> {
                self.iface.read_mag_3_double_registers::<MagneticField>()
            }

            $(
                /// Enable the magnetometer's built in offset cancellation.
                ///
                /// Offset cancellation is **automatically** managed by the device in **continuous** mode.
                ///
                /// To later disable offset cancellation, use the
                #[doc = concat!("[`disable_mag_offset_cancellation`](", stringify!($Lsm), "::disable_mag_offset_cancellation)")]
                /// function.
                pub fn enable_mag_offset_cancellation(&mut self) -> Result<(), Error<CommE, PinE>> {
                    let reg = self.$offset_cancellation_field | $offset_cancellation_reg::OFF_CANC;

                    self.iface.write_mag_register(reg)?;
                    self.$offset_cancellation_field = reg;

                    Ok(())
                }

                /// Disable the magnetometer's built in offset cancellation.
                pub fn disable_mag_offset_cancellation(&mut self) -> Result<(), Error<CommE, PinE>> {
                    let reg = self.$offset_cancellation_field & !$offset_cancellation_reg::OFF_CANC;

                    self.iface.write_mag_register(reg)?;
                    self.$offset_cancellation_field = reg;

                    Ok(())
                }
            )*
        }

        impl<DI, CommE, PinE> $Lsm<DI, mode::MagOneShot>
        where
            DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
        {
            /// Get the measured magnetic field.
            pub fn magnetic_field(&mut self) -> nb::Result<MagneticField, Error<CommE, PinE>> {
                let status = self.mag_status()?;
                if status.xyz_new_data() {
                    Ok(self.iface.read_mag_3_double_registers::<MagneticField>()?)
                } else {
                    let cfg = self.iface.read_mag_register::<CfgRegAM>()?;
                    if !cfg.is_single_mode() {
                        // Switch to single mode.
                        let cfg = self.$conversion_mode_reg_field.single_mode();
                        self.iface.write_mag_register(cfg)?;
                        self.$conversion_mode_reg_field = cfg;
                    }
                    Err(nb::Error::WouldBlock)
                }
            }

            $(
                /// Enable the magnetometer's built in offset cancellation.
                ///
                /// Offset cancellation has to be **managed by the user** in **single measurement** (`OneShot`) mode averaging
                /// two consecutive measurements H<sub>n</sub> and H<sub>n-1</sub>.
                ///
                /// To later disable offset cancellation, use the
                #[doc = concat!("[`disable_mag_offset_cancellation`](", stringify!($Lsm), "::disable_mag_offset_cancellation)")]
                /// function.
                pub fn enable_mag_offset_cancellation(&mut self) -> Result<(), Error<CommE, PinE>> {
                    let reg = self.$offset_cancellation_field
                        .union($offset_cancellation_reg::OFF_CANC)
                        .union($offset_cancellation_reg::OFF_CANC_ONE_SHOT);

                    self.iface.write_mag_register(reg)?;
                    self.$offset_cancellation_field = reg;

                    Ok(())
                }

                /// Disable the magnetometer's built in offset cancellation.
                pub fn disable_mag_offset_cancellation(&mut self) -> Result<(), Error<CommE, PinE>> {
                    let reg = self.$offset_cancellation_field
                        .difference($offset_cancellation_reg::OFF_CANC)
                        .difference($offset_cancellation_reg::OFF_CANC_ONE_SHOT);

                    self.iface.write_mag_register(reg)?;
                    self.$offset_cancellation_field = reg;

                    Ok(())
                }
            )*
        }
    }
}

impl_mag!(
    Lsm303agr,
    agr::MagMode,
    agr::MagOutputDataRate,
    cfg_reg_a_m,
    cfg_reg_a_m,
    cfg_reg_b_m: CfgRegBM
);

impl_mag!(
    Lsm303c,
    c::MagMode,
    c::MagOutputDataRate,
    ctrl_reg1_m,
    ctrl_reg3_m
);
