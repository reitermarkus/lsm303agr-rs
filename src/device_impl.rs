use crate::{
    agr::{self, Lsm303agr},
    c::{self, Lsm303c},
    interface::{I2cInterface, ReadData, SpiInterface, WriteData},
    mode, Acceleration, AccelerometerId, Error, MagnetometerId, PhantomData, Status, Temperature,
    TemperatureStatus,
};

mod new;

mod init;

mod reset;

mod accel_mode_and_odr;

mod mag_mode_change;

mod magnetometer;

mod fifo;

mod interrupts;

macro_rules! impl_device {
    (
        $Lsm:ident,
        $StatusRegA:ty, $WhoAmIA:ty,
        $StatusRegM:ty, $WhoAmIM:ty,
        $AccelerationReg:ty,
        $temp_method:ident: $TempReg:ty,
    ) => {
        impl<DI, CommE, PinE, MODE> $Lsm<DI, MODE>
        where
            DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
        {
            /// Accelerometer status
            pub fn accel_status(&mut self) -> Result<Status, Error<CommE, PinE>> {
                self.iface
                    .read_accel_register::<$StatusRegA>()
                    .map(Status::new)
            }

            /// Get measured acceleration.
            pub fn acceleration(&mut self) -> Result<Acceleration, Error<CommE, PinE>> {
                let (x, y, z) = self
                    .iface
                    .read_accel_3_double_registers::<$AccelerationReg>()?;

                let mode = self.get_accel_mode();
                let scale = self.get_accel_scale();

                Ok(Acceleration {
                    x,
                    y,
                    z,
                    factor: mode.factor(scale),
                })
            }

            /// Get the magnetometer status.
            pub fn mag_status(&mut self) -> Result<Status, Error<CommE, PinE>> {
                self.iface
                    .read_mag_register::<$StatusRegM>()
                    .map(Status::new)
            }

            /// Get the accelerometer device ID.
            pub fn accelerometer_id(&mut self) -> Result<AccelerometerId, Error<CommE, PinE>> {
                let mut id = self.iface.read_accel_register::<$WhoAmIA>()?;
                id.id = <$WhoAmIA>::ID;
                Ok(id)
            }

            /// Get the magnetometer device ID.
            pub fn magnetometer_id(&mut self) -> Result<MagnetometerId, Error<CommE, PinE>> {
                let mut id = self.iface.read_mag_register::<$WhoAmIM>()?;
                id.id = <$WhoAmIM>::ID;
                Ok(id)
            }

            /// Get measured temperature.
            pub fn temperature(&mut self) -> Result<Temperature, Error<CommE, PinE>> {
                self.iface.$temp_method::<$TempReg>()
            }
        }
    };
}

impl_device!(
    Lsm303agr,
    agr::register::StatusRegA,
    agr::register::WhoAmIA,
    agr::register::StatusRegM,
    agr::register::WhoAmIM,
    agr::register::OutXLA,
    read_accel_double_register: agr::register::OutTempLA,
);

impl_device!(
    Lsm303c,
    c::register::StatusRegA,
    c::register::WhoAmIA,
    c::register::StatusRegM,
    c::register::WhoAmIM,
    c::register::OutXLA,
    read_mag_double_register: c::register::TempLM,
);

impl<DI, CommE, PinE, MODE> Lsm303agr<DI, MODE>
where
    DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
{
    /// Get the temperature sensor status.
    pub fn temperature_status(&mut self) -> Result<TemperatureStatus, Error<CommE, PinE>> {
        self.iface
            .read_accel_register::<agr::register::StatusRegAuxA>()
            .map(TemperatureStatus::new)
    }

    /// Enable magnetometer low-pass filter.
    pub fn mag_enable_low_pass_filter(&mut self) -> Result<(), Error<CommE, PinE>> {
        let regb = self.cfg_reg_b_m.union(agr::register::CfgRegBM::LPF);
        self.iface.write_mag_register(regb)?;
        self.cfg_reg_b_m = regb;

        Ok(())
    }

    /// Disable magnetometer low-pass filter.
    pub fn mag_disable_low_pass_filter(&mut self) -> Result<(), Error<CommE, PinE>> {
        let regb = self.cfg_reg_b_m.difference(agr::register::CfgRegBM::LPF);
        self.iface.write_mag_register(regb)?;
        self.cfg_reg_b_m = regb;

        Ok(())
    }
}
