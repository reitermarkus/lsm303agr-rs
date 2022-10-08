use super::*;

macro_rules! impl_interrupts {
    ($Lsm:ident, $Interrupt:ty, $int_mag_reg_field:ident: $int_mag:expr) => {
        impl<DI, CommE, PinE, MODE> $Lsm<DI, MODE>
        where
            DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
        {
            /// Enable accelerometer interrupt.
            pub fn acc_enable_interrupt(
                &mut self,
                interrupt: $Interrupt,
            ) -> Result<(), Error<CommE, PinE>> {
                let reg3 = self.ctrl_reg3_a.with_interrupt(interrupt);
                self.iface.write_accel_register(reg3)?;
                self.ctrl_reg3_a = reg3;

                Ok(())
            }

            /// Disable accelerometer interrupt.
            pub fn acc_disable_interrupt(
                &mut self,
                interrupt: $Interrupt,
            ) -> Result<(), Error<CommE, PinE>> {
                let reg3 = self.ctrl_reg3_a.without_interrupt(interrupt);
                self.iface.write_accel_register(reg3)?;
                self.ctrl_reg3_a = reg3;

                Ok(())
            }

            /// Configure the DRDY pin as a digital output.
            pub fn mag_enable_interrupt(&mut self) -> Result<(), Error<CommE, PinE>> {
                let reg = self.$int_mag_reg_field.union($int_mag);
                self.iface.write_mag_register(reg)?;
                self.$int_mag_reg_field = reg;

                Ok(())
            }

            /// Unconfigure the DRDY pin as a digital output.
            pub fn mag_disable_interrupt(&mut self) -> Result<(), Error<CommE, PinE>> {
                let reg = self.$int_mag_reg_field.difference($int_mag);
                self.iface.write_mag_register(reg)?;
                self.$int_mag_reg_field = reg;

                Ok(())
            }
        }
    };
}

impl_interrupts!(
    Lsm303agr,
    agr::Interrupt,
    cfg_reg_c_m: agr::register::CfgRegCM::INT_MAG
);
impl_interrupts!(Lsm303c, c::Interrupt, int_reg_m: c::register::IntCfgM::IEA);
