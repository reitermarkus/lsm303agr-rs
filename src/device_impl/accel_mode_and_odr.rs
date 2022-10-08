use embedded_hal::blocking::delay::DelayUs;

use crate::{
    agr::{self, Lsm303agr},
    c::{self, Lsm303c},
    interface::{ReadData, WriteData},
    register::{CtrlReg1A, CtrlReg4A},
    Error,
};

impl<DI, CommE, PinE, MODE> Lsm303agr<DI, MODE>
where
    DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
{
    /// Set accelerometer power/resolution mode and output data rate.
    ///
    /// Returns `Error::InvalidInputData` if the mode is incompatible with
    /// the given output data rate.
    ///
    #[doc = include_str!("delay.md")]
    pub fn set_accel_mode_and_odr<D: DelayUs<u32>>(
        &mut self,
        delay: &mut D,
        mode: agr::AccelMode,
        odr: impl Into<Option<agr::AccelOutputDataRate>>,
    ) -> Result<(), Error<CommE, PinE>> {
        let odr = odr.into();

        check_accel_odr_is_compatible_with_mode(odr, mode)?;

        let old_mode = self.get_accel_mode();

        let mut reg1 = self.ctrl_reg1_a.difference(CtrlReg1A::ODR);

        if let Some(odr) = odr {
            reg1 = reg1.with_odr(odr);
        }

        let reg1 = if mode == agr::AccelMode::LowPower {
            reg1.union(CtrlReg1A::LPEN)
        } else {
            reg1.difference(CtrlReg1A::LPEN)
        };

        let reg4 = self.ctrl_reg4_a.difference(CtrlReg4A::HR);

        if mode != agr::AccelMode::HighResolution {
            self.iface.write_accel_register(reg4)?;
            self.ctrl_reg4_a = reg4;
        }

        self.iface.write_accel_register(reg1)?;
        self.ctrl_reg1_a = reg1;
        self.accel_odr = odr;

        if mode == agr::AccelMode::HighResolution {
            let reg4 = reg4.union(CtrlReg4A::HR);
            self.iface.write_accel_register(reg4)?;
            self.ctrl_reg4_a = reg4;
        }

        if let Some(odr) = self.accel_odr {
            let change_time = old_mode.change_time_us(mode, odr);
            delay.delay_us(change_time);
        }

        Ok(())
    }

    /// Get the accelerometer mode.
    pub fn get_accel_mode(&mut self) -> agr::AccelMode {
        let power_down = self.ctrl_reg1_a.intersection(CtrlReg1A::ODR).is_empty();
        let lp_enabled = self.ctrl_reg1_a.contains(CtrlReg1A::LPEN);
        let hr_enabled = self.ctrl_reg4_a.contains(CtrlReg4A::HR);

        if power_down {
            agr::AccelMode::PowerDown
        } else if hr_enabled {
            agr::AccelMode::HighResolution
        } else if lp_enabled {
            agr::AccelMode::LowPower
        } else {
            agr::AccelMode::Normal
        }
    }
}

impl<DI, CommE, PinE, MODE> Lsm303c<DI, MODE>
where
    DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
{
    /// Set accelerometer power/resolution mode and output data rate.
    ///
    /// Returns `Error::InvalidInputData` if the mode is incompatible with
    /// the given output data rate.
    ///
    #[doc = include_str!("delay.md")]
    pub fn set_accel_mode_and_odr<D: DelayUs<u32>>(
        &mut self,
        delay: &mut D,
        mode: c::AccelMode,
        odr: impl Into<Option<c::AccelOutputDataRate>>,
    ) -> Result<(), Error<CommE, PinE>> {
        let odr = odr.into();

        if odr.is_none() && mode != c::AccelMode::PowerDown {
            return Err(Error::InvalidInputData);
        }

        let old_mode = self.get_accel_mode();

        let mut reg1 = self.ctrl_reg1_a.difference(c::register::CtrlReg1A::ODR);

        if let Some(odr) = odr {
            reg1 = reg1.with_odr(odr);
        }

        let reg1 = if mode == c::AccelMode::HighResolution {
            reg1.union(c::register::CtrlReg1A::HR)
        } else {
            reg1.difference(c::register::CtrlReg1A::HR)
        };

        self.iface.write_accel_register(reg1)?;
        self.ctrl_reg1_a = reg1;
        self.accel_odr = odr;

        if old_mode == c::AccelMode::PowerDown && mode != c::AccelMode::PowerDown {
            delay.delay_us(100_000);
        }

        Ok(())
    }

    /// Get the accelerometer mode.
    pub fn get_accel_mode(&mut self) -> c::AccelMode {
        let power_down = self
            .ctrl_reg1_a
            .intersection(c::register::CtrlReg1A::ODR)
            .is_empty();
        let hr_enabled = self.ctrl_reg1_a.contains(c::register::CtrlReg1A::HR);

        if power_down {
            c::AccelMode::PowerDown
        } else if hr_enabled {
            c::AccelMode::HighResolution
        } else {
            c::AccelMode::Normal
        }
    }
}

macro_rules! impl_acc_mode_change {
    (
        $Lsm:ident,
        $SCALE:ty,
        $scale_reg_field:ident
    ) => {
        impl<DI, CommE, PinE, MODE> $Lsm<DI, MODE>
        where
            DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
        {
            /// Set accelerometer scaling factor.
            ///
            /// This changes the scale at which the acceleration is read.
            /// `AccelScale::G2` for example can return values between -2g and +2g
            /// where g is the gravity of the earth (~9.82 m/s²).
            pub fn set_accel_scale(&mut self, scale: $SCALE) -> Result<(), Error<CommE, PinE>> {
                let reg = self.$scale_reg_field.with_scale(scale);
                self.iface.write_accel_register(reg)?;
                self.$scale_reg_field = reg;
                Ok(())
            }

            /// Get accelerometer scaling factor.
            pub fn get_accel_scale(&self) -> $SCALE {
                self.$scale_reg_field.scale()
            }
        }
    };
}

impl_acc_mode_change!(Lsm303agr, agr::AccelScale, ctrl_reg4_a);
impl_acc_mode_change!(Lsm303c, c::AccelScale, ctrl_reg4_a);

fn check_accel_odr_is_compatible_with_mode<CommE, PinE>(
    odr: Option<agr::AccelOutputDataRate>,
    mode: agr::AccelMode,
) -> Result<(), Error<CommE, PinE>> {
    match (odr, mode) {
        (None, agr::AccelMode::PowerDown) => Ok(()),
        (None, _) => Err(Error::InvalidInputData),
        (Some(odr), mode) => match (odr, mode) {
            (agr::AccelOutputDataRate::Khz1_344, agr::AccelMode::LowPower)
            | (
                agr::AccelOutputDataRate::Khz1_620LowPower
                | agr::AccelOutputDataRate::Khz5_376LowPower,
                agr::AccelMode::Normal | agr::AccelMode::HighResolution,
            ) => Err(Error::InvalidInputData),
            _ => Ok(()),
        },
    }
}

#[cfg(test)]
mod accel_odr_mode_tests {
    use super::agr::AccelMode;
    use super::agr::AccelOutputDataRate as ODR;
    use super::check_accel_odr_is_compatible_with_mode;

    macro_rules! compatible {
        ($odr:ident, $power:ident) => {
            check_accel_odr_is_compatible_with_mode::<(), ()>(Some(ODR::$odr), AccelMode::$power)
                .unwrap();
        };
    }

    macro_rules! not_compatible {
        ($odr:ident, $power:ident) => {
            check_accel_odr_is_compatible_with_mode::<(), ()>(Some(ODR::$odr), AccelMode::$power)
                .expect_err("Should have returned error");
        };
    }

    macro_rules! none_odr_compatible {
        ($power:ident) => {
            check_accel_odr_is_compatible_with_mode::<(), ()>(None, AccelMode::$power).unwrap();
        };
    }

    macro_rules! not_none_odr_compatible {
        ($power:ident) => {
            check_accel_odr_is_compatible_with_mode::<(), ()>(None, AccelMode::$power)
                .expect_err("Shout not be compatible");
        };
    }

    #[test]
    fn all_modes_are_compatible_with_powerdown() {
        compatible!(Hz1, PowerDown);
        compatible!(Hz10, PowerDown);
        compatible!(Hz25, PowerDown);
        compatible!(Hz50, PowerDown);
        compatible!(Hz100, PowerDown);
        compatible!(Hz200, PowerDown);
        compatible!(Hz400, PowerDown);
        compatible!(Khz1_620LowPower, PowerDown);
        compatible!(Khz5_376LowPower, PowerDown);
        compatible!(Khz1_344, PowerDown);
    }

    #[test]
    fn normal_mode_compatibility() {
        compatible!(Hz1, Normal);
        compatible!(Hz10, Normal);
        compatible!(Hz25, Normal);
        compatible!(Hz50, Normal);
        compatible!(Hz100, Normal);
        compatible!(Hz200, Normal);
        compatible!(Hz400, Normal);
        not_compatible!(Khz1_620LowPower, Normal);
        not_compatible!(Khz5_376LowPower, Normal);
        compatible!(Khz1_344, Normal);
    }

    #[test]
    fn high_resolution_mode_compatibility() {
        compatible!(Hz1, HighResolution);
        compatible!(Hz10, HighResolution);
        compatible!(Hz25, HighResolution);
        compatible!(Hz50, HighResolution);
        compatible!(Hz100, HighResolution);
        compatible!(Hz200, HighResolution);
        compatible!(Hz400, HighResolution);
        not_compatible!(Khz1_620LowPower, HighResolution);
        not_compatible!(Khz5_376LowPower, HighResolution);
        compatible!(Khz1_344, HighResolution);
    }

    #[test]
    fn low_power_mode_compatibility() {
        compatible!(Hz1, LowPower);
        compatible!(Hz10, LowPower);
        compatible!(Hz25, LowPower);
        compatible!(Hz50, LowPower);
        compatible!(Hz100, LowPower);
        compatible!(Hz200, LowPower);
        compatible!(Hz400, LowPower);
        compatible!(Khz1_620LowPower, LowPower);
        compatible!(Khz5_376LowPower, LowPower);
        not_compatible!(Khz1_344, LowPower);
    }

    #[test]
    fn none_odr_compatibility() {
        not_none_odr_compatible!(LowPower);
        not_none_odr_compatible!(Normal);
        not_none_odr_compatible!(HighResolution);
        none_odr_compatible!(PowerDown);
    }
}
