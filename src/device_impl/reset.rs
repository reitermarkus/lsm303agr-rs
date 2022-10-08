use super::*;

macro_rules! impl_reset {
    (
        $Lsm:ident,
        $acc_reg_field:ident: $acc_reg:ty, [$($acc_reset_field:ident),* $(,)?],
        $mag_reg_field:ident: $mag_reg:ty, [$($mag_reset_field:ident),* $(,)?],
    ) => {
        impl<DI, CommE, PinE, MODE> $Lsm<DI, MODE>
        where
            DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
        {
            /// Reboot accelerometer memory content.
            pub fn acc_reboot_mem(&mut self) -> Result<(), Error<CommE, PinE>> {
                let reg = self.$acc_reg_field | <$acc_reg>::BOOT;
                self.iface.write_accel_register(reg)?;

                // Ensure the `BOOT` flag is cleared again.
                let reg = <$acc_reg>::default();
                self.iface.write_accel_register(reg)?;
                self.$acc_reg_field = reg;

                // Registers are now reset.
                $(
                    self.$acc_reset_field = Default::default();
                )*

                Ok(())
            }

            /// Reboot magnetometer memory content.
            pub fn mag_reboot_mem(&mut self) -> Result<(), Error<CommE, PinE>> {
                let reg = self.$mag_reg_field | <$mag_reg>::REBOOT;
                self.iface.write_mag_register(reg)?;

                // Ensure the `REBOOT` flag is cleared again.
                let reg = <$mag_reg>::default();
                self.iface.write_mag_register(reg)?;
                self.$mag_reg_field = reg;

                // Registers are now reset.
                $(
                    self.$mag_reset_field = Default::default();
                )*

                Ok(())
            }

            /// Soft reset magnetometer and clear registers.
            pub fn mag_soft_reset(&mut self) -> Result<(), Error<CommE, PinE>> {
                let reg = self.$mag_reg_field | <$mag_reg>::SOFT_RST;
                self.iface.write_mag_register(reg)?;

                // Ensure `SOFT_RST` flag is cleared again.
                let reg = <$mag_reg>::default();
                self.iface.write_mag_register(reg)?;
                self.$mag_reg_field = reg;

                // Registers are now reset.
                $(
                    self.$mag_reset_field = Default::default();
                )*

                Ok(())
            }
        }
    };
}

impl_reset!(
    Lsm303agr,
    ctrl_reg5_a: agr::register::CtrlReg5A,
    [temp_cfg_reg_a],
    cfg_reg_a_m: agr::register::CfgRegAM,
    [cfg_reg_b_m, cfg_reg_c_m],
);

impl_reset!(
    Lsm303c,
    ctrl_reg6_a: c::register::CtrlReg6A,
    [ctrl_reg1_m],
    ctrl_reg2_m: c::register::CtrlReg2M,
    [
        ctrl_reg3_m,
        // ctrl_reg4_m,
    ],
);
