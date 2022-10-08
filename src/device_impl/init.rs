use super::*;

macro_rules! impl_init {
    (
        $Lsm:ident,
        $acc_bdu_reg_field:ident: $acc_bdu_reg:ty,
        $mag_bdu_reg_field:ident: $mag_bdu_reg:ty,
        $temp_reg_field:ident: $temp_reg:ty,
    ) => {
        impl<DI, CommE, PinE, MODE> $Lsm<DI, MODE>
        where
            DI: WriteData<Error = Error<CommE, PinE>>,
        {
            /// Initialize registers
            pub fn init(&mut self) -> Result<(), Error<CommE, PinE>> {
                self.acc_enable_temp()?; // Also enables BDU.
                self.mag_enable_bdu()
            }

            /// Enable block data update for accelerometer.
            #[inline]
            fn acc_enable_bdu(&mut self) -> Result<(), Error<CommE, PinE>> {
                let reg = self.$acc_bdu_reg_field | <$acc_bdu_reg>::BDU;
                self.iface.write_accel_register(reg)?;
                self.$acc_bdu_reg_field = reg;

                Ok(())
            }

            /// Enable the temperature sensor.
            #[inline]
            fn acc_enable_temp(&mut self) -> Result<(), Error<CommE, PinE>> {
                self.acc_enable_bdu()?;

                let reg = self.$temp_reg_field | <$temp_reg>::TEMP_EN;
                self.iface.write_accel_register(reg)?;
                self.$temp_reg_field = reg;

                Ok(())
            }

            /// Enable block data update for magnetometer.
            #[inline]
            fn mag_enable_bdu(&mut self) -> Result<(), Error<CommE, PinE>> {
                let reg = self.$mag_bdu_reg_field | <$mag_bdu_reg>::BDU;
                self.iface.write_mag_register(reg)?;
                self.$mag_bdu_reg_field = reg;

                Ok(())
            }
        }
    };
}

impl_init!(
    Lsm303agr,
    ctrl_reg4_a: agr::register::CtrlReg4A,
    cfg_reg_c_m: agr::register::CfgRegCM,
    temp_cfg_reg_a: agr::register::TempCfgRegA,
);

impl_init!(
    Lsm303c,
    ctrl_reg1_a: c::register::CtrlReg1A,
    ctrl_reg5_m: c::register::CtrlReg5M,
    ctrl_reg1_m: c::register::CtrlReg1M,
);
