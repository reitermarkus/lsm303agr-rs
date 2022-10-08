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

mod accel_mode_and_odr;

mod mag_mode_change;

mod magnetometer;

mod fifo;

impl<DI, CommE, PinE, MODE> Lsm303agr<DI, MODE>
where
    DI: ReadData<Error = Error<CommE, PinE>> + WriteData<Error = Error<CommE, PinE>>,
{
    /// Enable accelerometer interrupt.
    pub fn acc_enable_interrupt(&mut self, interrupt: Interrupt) -> Result<(), Error<CommE, PinE>> {
        let reg3 = self.ctrl_reg3_a.with_interrupt(interrupt);
        self.iface.write_accel_register(reg3)?;
        self.ctrl_reg3_a = reg3;

        Ok(())
    }

    /// Disable accelerometer interrupt.
    pub fn acc_disable_interrupt(
        &mut self,
        interrupt: Interrupt,
    ) -> Result<(), Error<CommE, PinE>> {
        let reg3 = self.ctrl_reg3_a.without_interrupt(interrupt);
        self.iface.write_accel_register(reg3)?;
        self.ctrl_reg3_a = reg3;

        Ok(())
    }

    /// Reboot accelerometer memory content.
    pub fn acc_reboot_mem(&mut self) -> Result<(), Error<CommE, PinE>> {
        let reg5 = self.ctrl_reg5_a | CtrlReg5A::BOOT;
        self.iface.write_accel_register(reg5)?;

        // Ensure the `BOOT` flag is cleared again.
        let reg5 = CtrlReg5A::default();
        self.iface.write_accel_register(reg5)?;
        self.ctrl_reg5_a = reg5;

        // Registers are now reset.
        self.temp_cfg_reg_a = Default::default();

        Ok(())
    }

    /// Reboot magnetometer memory content.
    pub fn mag_reboot_mem(&mut self) -> Result<(), Error<CommE, PinE>> {
        let rega = self.cfg_reg_a_m | CfgRegAM::REBOOT;
        self.iface.write_mag_register(rega)?;

        // Ensure the `REBOOT` flag is cleared again.
        let rega = CfgRegAM::default();
        self.iface.write_mag_register(rega)?;

        // Registers are now reset.
        self.cfg_reg_a_m = rega;
        self.cfg_reg_b_m = Default::default();
        self.cfg_reg_c_m = Default::default();

        Ok(())
    }

    /// Soft reset magnetometer and clear registers.
    pub fn mag_soft_reset(&mut self) -> Result<(), Error<CommE, PinE>> {
        let rega = self.cfg_reg_a_m | CfgRegAM::SOFT_RST;
        self.iface.write_mag_register(rega)?;

        // Ensure `SOFT_RST` flag is cleared again.
        let rega = CfgRegAM::default();
        self.iface.write_mag_register(rega)?;

        // Registers are now reset.
        self.cfg_reg_a_m = rega;
        self.cfg_reg_b_m = Default::default();
        self.cfg_reg_c_m = Default::default();

        Ok(())
    }

    /// Configure the DRDY pin as a digital output.
    pub fn mag_enable_interrupt(&mut self) -> Result<(), Error<CommE, PinE>> {
        let regc = self.cfg_reg_c_m.union(CfgRegCM::INT_MAG);
        self.iface.write_mag_register(regc)?;
        self.cfg_reg_c_m = regc;

        Ok(())
    }

    /// Unconfigure the DRDY pin as a digital output.
    pub fn mag_disable_interrupt(&mut self) -> Result<(), Error<CommE, PinE>> {
        let regc = self.cfg_reg_c_m.difference(CfgRegCM::INT_MAG);
        self.iface.write_mag_register(regc)?;
        self.cfg_reg_c_m = regc;

        Ok(())
    }

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
