use super::*;

macro_rules! impl_fifo {
    ($Lsm:ident, $FifoMode:ty, $en_reg_field:ident: [$($en_bit:expr),*], $mode_reg_field:ident) => {
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
                $(
                    en_reg.set($en_bit, mode != <$FifoMode>::Bypass);
                )*
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

impl_fifo!(
    Lsm303agr,
    agr::FifoMode,
    ctrl_reg5_a: [agr::register::CtrlReg5A::FIFO_EN],
    fifo_ctrl_reg_a
);

impl_fifo!(
    Lsm303c,
    c::FifoMode,
    ctrl_reg3_a: [c::register::CtrlReg3A::FIFO_EN, c::register::CtrlReg3A::STOP_FTH],
    fifo_ctrl
);
