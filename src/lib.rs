//! This is a platform agnostic Rust driver for the LSM303AGR ultra-compact
//! high-performance eCompass module: ultra-low-power 3D accelerometer and
//! 3D magnetometer using the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Connect through I2C or SPI. See: [`new_with_i2c()`](Lsm303agr::new_with_i2c) and [`new_with_spi()`](Lsm303agr::new_with_spi) .
//! - Initialize the device. See: [`init()`](Lsm303agr::init).
//! - Accelerometer:
//!     - Read measured acceleration. See: [`acceleration()`](Lsm303agr::acceleration).
//!     - Get accelerometer status. See: [`accel_status()`](Lsm303agr::accel_status).
//!     - Set accelerometer output data rate. See: [`set_accel_odr()`](Lsm303agr::set_accel_odr).
//!     - Set accelerometer mode. See: [`set_accel_mode()`](Lsm303agr::set_accel_mode).
//!     - Set accelerometer scale. See: [`set_accel_scale()`](Lsm303agr::set_accel_scale).
//!     - Get accelerometer ID. See: [`accelerometer_id()`](Lsm303agr::accelerometer_id).
//!     - Get temperature sensor status. See: [`temperature_status()`](Lsm303agr::temperature_status).
//!     - Read measured temperature. See: [`temperature()`](Lsm303agr::temperature).
//!     - Configure FIFO. See: [`acc_set_fifo_mode()`](Lsm303agr::acc_set_fifo_mode).
//!     - Enable/disable interrupts. See: [`acc_enable_interrupt()`](Lsm303agr::acc_enable_interrupt).
//! - Magnetometer:
//!     - Get the magnetometer status. See: [`mag_status()`](Lsm303agr::mag_status).
//!     - Change into continuous/one-shot mode. See: [`into_mag_continuous()`](Lsm303agr::into_mag_continuous).
//!     - Read measured magnetic field. See: [`magnetic_field()`](Lsm303agr::magnetic_field).
//!     - Set magnetometer output data rate. See: [`set_mag_odr()`](Lsm303agr::set_mag_odr).
//!     - Get magnetometer ID. See: [`magnetometer_id()`](Lsm303agr::magnetometer_id).
//!     - Enable/disable magnetometer built in offset cancellation. See: [`enable_mag_offset_cancellation()`](Lsm303agr::enable_mag_offset_cancellation).
//!     - Enable/disable magnetometer low-pass filter. See: [`mag_enable_low_pass_filter()`](Lsm303agr::mag_enable_low_pass_filter).
//!     - Set magnetometer mode. See: [`set_mag_mode()`](Lsm303agr::set_mag_mode).
//!
//! <!-- TODO
//! [Introductory blog post](TODO)
//! -->
//!
//! ## The devices
//!
//! The LSM303AGR is an inertial measurement unit (IMU) consisting of a
//! state-of-the-art 3-axis, low-g accelerometer and a low power 3-axis
//! gyroscope. It has been designed for low power, high precision 6-axis and
//! 9-axis applications in mobile phones, tablets, wearable devices, remote
//! controls, game controllers, head-mounted devices and toys.
//!
//! The LSM303AGR is available in a compact 14-pin 2.5 × 3.0 × 0.83 mm3 LGA
//! package. When accelerometer and gyroscope are in full operation mode, power
//! consumption is typically 925 μA, enabling always-on applications in
//! battery driven devices.
//!
//! Further Bosch Sensortec sensors, e.g. geomagnetic (BMM150) can be connected
//! as slave via a secondary I2C interface. In this configuration, the LSM303AGR
//! controls the data acquisition of the external sensor and the synchronized
//! data of all sensors is stored the register data and can be additionally
//! stored in the built-in FIFO.
//!
//! Besides the flexible primary interface (I2C or SPI) that is used to connect
//! to the host, LSM303AGR provides an additional secondary interface. This
//! secondary interface can be used in SPI mode for OIS (optical image
//! stabilization) applications in conjunction with camera modules, or in
//! advanced gaming use cases.
//!
//! Documents: [Datasheet](https://www.st.com/resource/en/datasheet/lsm303agr.pdf) - [Application note](https://www.st.com/resource/en/application_note/dm00265383-ultracompact-highperformance-ecompass-module-based-on-the-lsm303agr-stmicroelectronics.pdf)
//!
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the appropriate device.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Connect through I2C, initialize and take some measurements
//!
//! ```no_run
//! # #[cfg(target_os = "linux")] {
//! use linux_embedded_hal::{Delay, I2cdev};
//! use lsm303agr::{AccelOutputDataRate, Lsm303agr};
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Lsm303agr::new_with_i2c(dev);
//!
//! sensor.init().unwrap();
//! sensor.set_accel_odr(&mut Delay, AccelOutputDataRate::Hz10).unwrap();
//!
//! loop {
//!     if sensor.accel_status().unwrap().xyz_new_data() {
//!         let data = sensor.acceleration().unwrap();
//!         println!("Acceleration: x {} y {} z {}", data.x_mg(), data.y_mg(), data.z_mg());
//!     }
//! }
//! # }
//! ```
//!
//! ### Connect through SPI, initialize and take some measurements
//!
//! ```no_run
//! # #[cfg(target_os = "linux")] {
//! use linux_embedded_hal::{Delay, Spidev, Pin};
//! use lsm303agr::{AccelOutputDataRate, Lsm303agr};
//!
//! let dev = Spidev::open("/dev/spidev0.0").unwrap();
//! let accel_cs = Pin::new(17);
//! let mag_cs = Pin::new(27);
//! let mut sensor = Lsm303agr::new_with_spi(dev, accel_cs, mag_cs);
//!
//! sensor.init().unwrap();
//! sensor.set_accel_odr(&mut Delay, AccelOutputDataRate::Hz10).unwrap();
//!
//! loop {
//!     if sensor.accel_status().unwrap().xyz_new_data() {
//!         let data = sensor.acceleration().unwrap();
//!         println!("Acceleration: x {} y {} z {}", data.x_mg(), data.y_mg(), data.z_mg());
//!     }
//! }
//! # }
//! ```

#![deny(unsafe_code, missing_docs)]
#![no_std]
#![doc(html_root_url = "https://docs.rs/lsm303agr/0.2.2")]

use core::marker::PhantomData;
mod accel_mode_and_odr;
mod device_impl;
pub mod interface;
mod mag_mode_change;
mod magnetometer;
mod types;
pub use crate::types::{
    mode, AccelMode, AccelOutputDataRate, AccelScale, Acceleration, AccelerometerId, Error,
    FifoMode, Interrupt, MagMode, MagOutputDataRate, MagneticField, MagnetometerId,
    ModeChangeError, Status, Temperature, TemperatureStatus,
};
mod register_address;
use crate::register_address::{
    CfgRegAM, CfgRegBM, CfgRegCM, CtrlReg1A, CtrlReg3A, CtrlReg4A, CtrlReg5A, FifoCtrlRegA,
    TempCfgRegA,
};

/// LSM303AGR device driver
#[derive(Debug)]
pub struct Lsm303agr<DI, MODE> {
    /// Digital interface: I2C or SPI
    iface: DI,
    ctrl_reg1_a: CtrlReg1A,
    ctrl_reg3_a: CtrlReg3A,
    ctrl_reg4_a: CtrlReg4A,
    ctrl_reg5_a: CtrlReg5A,
    cfg_reg_a_m: CfgRegAM,
    cfg_reg_b_m: CfgRegBM,
    cfg_reg_c_m: CfgRegCM,
    temp_cfg_reg_a: TempCfgRegA,
    fifo_ctrl_reg_a: FifoCtrlRegA,
    accel_odr: Option<AccelOutputDataRate>,
    _mag_mode: PhantomData<MODE>,
}

mod private {
    use crate::interface;
    pub trait Sealed {}

    impl<SPI, CSXL, CSMAG> Sealed for interface::SpiInterface<SPI, CSXL, CSMAG> {}
    impl<I2C> Sealed for interface::I2cInterface<I2C> {}
}
