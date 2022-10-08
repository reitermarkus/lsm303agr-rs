use super::*;

macro_rules! impl_fifo {
    ($Lsm:ident, $FifoMode:ty, $en_reg_field:ident: $en_reg:ty, $mode_reg_field:ident) => {
        impl<DI, CommE, PinE, MODE> $Lsm<DI, MODE>
        where
            DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
        {
            /// Set the accelerometer FIFO mode and full threshold.
            ///
            /// The threshold is clamped to \[0, 31\].
            pub fn acc_set_fifo_mode(
                &mut self,
                mode: $FifoMode,
                fth: u8,
            ) -> Result<(), Error<CommE, PinE>> {
                let mut en_reg = self.$en_reg_field;
                en_reg.set(<$en_reg>::FIFO_EN, mode != <$FifoMode>::Bypass);
                self.iface.write_accel_register(en_reg)?;
                self.$en_reg_field = en_reg;

                let mode_reg = self
                    .$mode_reg_field
                    .with_mode(mode)
                    .with_full_threshold(fth);
                self.iface.write_accel_register(mode_reg)?;
                self.$mode_reg_field = mode_reg;

                Ok(())
            }
        }
    };
}

impl_fifo!(Lsm303agr, FifoMode, ctrl_reg5_a: CtrlReg5A, fifo_ctrl_reg_a);

impl_fifo!(
    Lsm303c,
    c::FifoMode,
    ctrl_reg3_a: c::register::CtrlReg3A,
    fifo_ctrl
);
