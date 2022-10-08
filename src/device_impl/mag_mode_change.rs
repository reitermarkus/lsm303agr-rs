use crate::{
    c::Lsm303c,
    interface::{ReadData, WriteData},
    mode, Error, Lsm303agr, ModeChangeError, PhantomData,
};

macro_rules! impl_mag_mode_change {
    ($Lsm:ident, $mode_reg_field:ident, $($other_field:ident,)*) => {
        impl<DI, CommE, PinE> $Lsm<DI, mode::MagOneShot>
        where
            DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
        {
            /// Change the magnetometer to continuous measurement mode.
            pub fn into_mag_continuous(
                mut self,
            ) -> Result<$Lsm<DI, mode::MagContinuous>, ModeChangeError<CommE, PinE, Self>> {
                let cfg = self.$mode_reg_field.continuous_mode();
                match self.iface.write_mag_register(cfg) {
                    Err(error) => Err(ModeChangeError { error, dev: self }),
                    Ok(_) => Ok($Lsm {
                        $mode_reg_field: cfg,
                        $(
                            $other_field: self.$other_field,
                        )*
                        _mag_mode: PhantomData,
                    }),
                }
            }
        }

        impl<DI, CommE, PinE> $Lsm<DI, mode::MagContinuous>
        where
            DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
        {
            /// Change the magnetometer to one-shot mode.
            ///
            /// After this the magnetometer is in idle mode until a one-shot measurement
            /// is started.
            pub fn into_mag_one_shot(
                mut self,
            ) -> Result<$Lsm<DI, mode::MagOneShot>, ModeChangeError<CommE, PinE, Self>> {
                let cfg = self.$mode_reg_field.idle_mode();
                match self.iface.write_mag_register(cfg) {
                    Err(error) => Err(ModeChangeError { error, dev: self }),
                    Ok(_) => Ok($Lsm {
                        $mode_reg_field: cfg,
                        $(
                            $other_field: self.$other_field,
                        )*
                        _mag_mode: PhantomData,
                    }),
                }
            }
        }
    }
}

impl_mag_mode_change!(
    Lsm303agr,
    cfg_reg_a_m,
    iface,
    ctrl_reg1_a,
    ctrl_reg3_a,
    ctrl_reg4_a,
    ctrl_reg5_a,
    cfg_reg_b_m,
    cfg_reg_c_m,
    temp_cfg_reg_a,
    fifo_ctrl_reg_a,
    accel_odr,
);

impl_mag_mode_change!(
    Lsm303c,
    ctrl_reg3_m,
    iface,
    ctrl_reg1_a,
    ctrl_reg2_a,
    ctrl_reg3_a,
    ctrl_reg4_a,
    ctrl_reg1_m,
    ctrl_reg2_m,
    ctrl_reg5_m,
    fifo_ctrl,
    accel_odr,
);
