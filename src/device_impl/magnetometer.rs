use embedded_hal::blocking::delay::DelayUs;

use crate::{
    agr::{self, Lsm303agr},
    c::{self, Lsm303c},
    interface::{ReadData, WriteData},
    mode, Error, MagneticField,
};

impl<DI, CommE, PinE, MODE> Lsm303agr<DI, MODE>
where
    DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
{
    /// Set magnetometer power/resolution mode and output data rate.
    ///
    #[doc = include_str!("delay.md")]
    pub fn set_mag_mode_and_odr<D: DelayUs<u32>>(
        &mut self,
        delay: &mut D,
        mode: agr::MagMode,
        odr: agr::MagOutputDataRate,
    ) -> Result<(), Error<CommE, PinE>> {
        let reg = self.cfg_reg_a_m;

        let old_mode = reg.mode();
        let old_odr = reg.odr();

        let reg = reg.with_mode(mode).with_odr(odr);
        self.iface.write_mag_register(reg)?;
        self.cfg_reg_a_m = reg;

        let offset_cancellation = self.cfg_reg_b_m.offset_cancellation();
        if old_mode != mode {
            delay.delay_us(reg.turn_on_time_us(offset_cancellation));
        } else if old_odr != odr && offset_cancellation {
            // Mode did not change, so only wait for 1/ODR ms.
            delay.delay_us(odr.turn_on_time_us_frac_1());
        }

        Ok(())
    }

    /// Get magnetometer power/resolution mode.
    pub fn get_mag_mode(&self) -> agr::MagMode {
        self.cfg_reg_a_m.mode()
    }
}

impl<DI, CommE, PinE, MODE> Lsm303c<DI, MODE>
where
    DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
{
    /// Set magnetometer power/resolution mode and output data rate.
    ///
    #[doc = include_str!("delay.md")]
    pub fn set_mag_mode_and_odr<D: DelayUs<u32>>(
        &mut self,
        delay: &mut D,
        mode: c::MagMode,
        odr: c::MagOutputDataRate,
    ) -> Result<(), Error<CommE, PinE>> {
        let xy_reg = self.ctrl_reg1_m;
        let z_reg = self.ctrl_reg4_m;

        let old_xy_mode = xy_reg.xy_mode();
        let old_z_mode = z_reg.z_mode();
        let old_odr = xy_reg.odr();

        let xy_reg = xy_reg.with_xy_mode(mode).with_odr(odr);
        self.iface.write_mag_register(xy_reg)?;
        self.ctrl_reg1_m = xy_reg;

        let z_reg = z_reg.with_z_mode(mode);
        self.iface.write_mag_register(z_reg)?;
        self.ctrl_reg4_m = z_reg;

        if old_xy_mode != mode || old_z_mode != mode || old_odr != odr {
            delay.delay_us(odr.turn_on_time_us_frac_1());
        }

        Ok(())
    }

    /// Get magnetometer power/resolution mode.
    pub fn get_mag_mode(&self) -> c::MagMode {
        self.ctrl_reg1_m.xy_mode()
    }
}

macro_rules! impl_mag {
    (
        $Lsm:ident,
        $MODE:ty, $ODR:ty,
        $MagneticFieldReg:ty,
        $power_mode_reg_field:ident,
        $conversion_mode_reg_field:ident: $conversion_mode_reg:ty,
        $($offset_cancellation_field:ident: $offset_cancellation_reg:ty)?
    ) => {
        impl<DI, CommE, PinE> $Lsm<DI, mode::MagContinuous>
        where
            DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
        {
            /// Get the measured magnetic field.
            pub fn magnetic_field(&mut self) -> Result<MagneticField, Error<CommE, PinE>> {
                self.iface.read_mag_3_double_registers::<$MagneticFieldReg>()
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
                    let reg = self.$offset_cancellation_field | <$offset_cancellation_reg>::OFF_CANC;

                    self.iface.write_mag_register(reg)?;
                    self.$offset_cancellation_field = reg;

                    Ok(())
                }

                /// Disable the magnetometer's built in offset cancellation.
                pub fn disable_mag_offset_cancellation(&mut self) -> Result<(), Error<CommE, PinE>> {
                    let reg = self.$offset_cancellation_field & !<$offset_cancellation_reg>::OFF_CANC;

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
                    Ok(self.iface.read_mag_3_double_registers::<$MagneticFieldReg>()?)
                } else {
                    let cfg = self.iface.read_mag_register::<$conversion_mode_reg>()?;
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
                        .union(<$offset_cancellation_reg>::OFF_CANC)
                        .union(<$offset_cancellation_reg>::OFF_CANC_ONE_SHOT);

                    self.iface.write_mag_register(reg)?;
                    self.$offset_cancellation_field = reg;

                    Ok(())
                }

                /// Disable the magnetometer's built in offset cancellation.
                pub fn disable_mag_offset_cancellation(&mut self) -> Result<(), Error<CommE, PinE>> {
                    let reg = self.$offset_cancellation_field
                        .difference(<$offset_cancellation_reg>::OFF_CANC)
                        .difference(<$offset_cancellation_reg>::OFF_CANC_ONE_SHOT);

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
    agr::register::OutxLRegM,
    cfg_reg_a_m,
    cfg_reg_a_m: agr::register::CfgRegAM,
    cfg_reg_b_m: agr::register::CfgRegBM
);

impl_mag!(
    Lsm303c,
    c::MagMode,
    c::MagOutputDataRate,
    c::register::OutXLM,
    ctrl_reg1_m,
    ctrl_reg3_m: c::register::CtrlReg3M,
);
