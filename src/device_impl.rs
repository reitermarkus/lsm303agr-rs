use crate::{
    c::{self, Lsm303c},
    interface::{I2cInterface, ReadData, SpiInterface, WriteData},
    mode,
    register_address::{
        CfgRegAM, CfgRegBM, CfgRegCM, CtrlReg4A, CtrlReg5A, StatusRegA, StatusRegAuxA, StatusRegM,
        TempCfgRegA, WhoAmIA, WhoAmIM,
    },
    Acceleration, AccelerometerId, Error, FifoMode, Interrupt, Lsm303agr, MagnetometerId,
    PhantomData, Status, Temperature, TemperatureStatus,
};

mod new;

mod init;

mod reset;

mod accel_mode_and_odr;

mod mag_mode_change;

mod magnetometer;

mod fifo;

mod interrupts;

impl<DI, CommE, PinE, MODE> Lsm303agr<DI, MODE>
where
    DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
{
    /// Enable magnetometer low-pass filter.
    pub fn mag_enable_low_pass_filter(&mut self) -> Result<(), Error<CommE, PinE>> {
        let regb = self.cfg_reg_b_m.union(CfgRegBM::LPF);
        self.iface.write_mag_register(regb)?;
        self.cfg_reg_b_m = regb;

        Ok(())
    }

    /// Disable magnetometer low-pass filter.
    pub fn mag_disable_low_pass_filter(&mut self) -> Result<(), Error<CommE, PinE>> {
        let regb = self.cfg_reg_b_m.difference(CfgRegBM::LPF);
        self.iface.write_mag_register(regb)?;
        self.cfg_reg_b_m = regb;

        Ok(())
    }
}

macro_rules! impl_device {
    ($Lsm:ident) => {
        impl<DI, CommE, PinE, MODE> $Lsm<DI, MODE>
        where
            DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
        {
            /// Accelerometer status
            pub fn accel_status(&mut self) -> Result<Status, Error<CommE, PinE>> {
                self.iface
                    .read_accel_register::<StatusRegA>()
                    .map(Status::new)
            }

            /// Get measured acceleration.
            pub fn acceleration(&mut self) -> Result<Acceleration, Error<CommE, PinE>> {
                let (x, y, z) = self.iface.read_accel_3_double_registers::<Acceleration>()?;

                let mode = self.get_accel_mode();
                let scale = self.get_accel_scale();

                Ok(Acceleration {
                    x,
                    y,
                    z,
                    resolution_factor: mode.resolution_factor(),
                    scaling_factor: mode.scaling_factor(scale),
                })
            }

            /// Get the magnetometer status.
            pub fn mag_status(&mut self) -> Result<Status, Error<CommE, PinE>> {
                self.iface
                    .read_mag_register::<StatusRegM>()
                    .map(Status::new)
            }

            /// Get the accelerometer device ID.
            pub fn accelerometer_id(&mut self) -> Result<AccelerometerId, Error<CommE, PinE>> {
                self.iface.read_accel_register::<WhoAmIA>()
            }

            /// Get the magnetometer device ID.
            pub fn magnetometer_id(&mut self) -> Result<MagnetometerId, Error<CommE, PinE>> {
                self.iface.read_mag_register::<WhoAmIM>()
            }

            /// Get measured temperature.
            pub fn temperature(&mut self) -> Result<Temperature, Error<CommE, PinE>> {
                self.iface.read_accel_double_register::<Temperature>()
            }

            /// Get the temperature sensor status.
            pub fn temperature_status(&mut self) -> Result<TemperatureStatus, Error<CommE, PinE>> {
                self.iface
                    .read_accel_register::<StatusRegAuxA>()
                    .map(TemperatureStatus::new)
            }
        }
    };
}

impl_device!(Lsm303agr);
impl_device!(Lsm303c);
