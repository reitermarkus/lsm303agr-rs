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
                self.acc_enable_bdu()?;
                self.mag_enable_bdu()?;
                self.enable_temp()?;
                self.enable_acc()?;
                self.enable_mag()
            }

            /// Enable block data update for accelerometer.
            #[inline]
            fn acc_enable_bdu(&mut self) -> Result<(), Error<CommE, PinE>> {
                let reg = self.$acc_bdu_reg_field | <$acc_bdu_reg>::BDU;
                self.iface.write_accel_register(reg)?;
                self.$acc_bdu_reg_field = reg;

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

            /// Enable the temperature sensor.
            #[inline]
            fn enable_temp(&mut self) -> Result<(), Error<CommE, PinE>> {
                let reg = self.$temp_reg_field | <$temp_reg>::TEMP_EN;
                self.iface.write_accel_register(reg)?;
                self.$temp_reg_field = reg;

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

impl<DI, CommE, PinE, MODE> Lsm303agr<DI, MODE>
where
    DI: WriteData<Error = Error<CommE, PinE>>,
{
    #[inline(always)]
    fn enable_acc(&mut self) -> Result<(), Error<CommE, PinE>> {
        Ok(())
    }

    #[inline(always)]
    fn enable_mag(&mut self) -> Result<(), Error<CommE, PinE>> {
        Ok(())
    }
}

impl<DI, CommE, PinE, MODE> Lsm303c<DI, MODE>
where
    DI: WriteData<Error = Error<CommE, PinE>>,
{
    #[inline(always)]
    fn enable_acc(&mut self) -> Result<(), Error<CommE, PinE>> {
        // Enable address auto-increment for multi-byte reads.
        let reg = self.ctrl_reg4_a.union(c::register::CtrlReg4A::IF_ADD_INC);
        self.iface.write_accel_register(reg)?;
        self.ctrl_reg4_a = reg;
        Ok(())
    }

    #[inline(always)]
    fn enable_mag(&mut self) -> Result<(), Error<CommE, PinE>> {
        // Initialize scale to ±16 gauss (since this is the only possible option).
        let reg = self
            .ctrl_reg2_m
            .union(c::register::CtrlReg2M::FS1)
            .union(c::register::CtrlReg2M::FS0);
        self.iface.write_mag_register(reg)?;
        self.ctrl_reg2_m = reg;

        // Enable both read and write operations over SPI.
        let reg = self.ctrl_reg3_m.union(c::register::CtrlReg3M::SIM);
        self.iface.write_mag_register(reg)?;
        self.ctrl_reg3_m = reg;

        Ok(())
    }
}
