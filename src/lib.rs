//! # DRV260X Haptic Driver Family
//!
//! This is a platform-agnostic Rust driver for the Texas Instruments DRV260X haptic driver family,
//! built using the [`embedded-hal`] traits for I2C communication.
//!
//! The DRV260X family includes:
//! - DRV2605: Haptic driver with licensed ROM library
//! - DRV2605L: Low-voltage version of DRV2605
//! - DRV2604: Haptic driver with RAM (no ROM library)
//! - DRV2604L: Low-voltage version of DRV2604
//!
//! ## Features
//!
//! - **High-level API** for haptic effect playback and control
//! - **Async/await support** with feature gating (optional)
//! - **Multiple operating modes** including internal trigger, external trigger, PWM, audio-to-vibe, and real-time playback
//! - **Auto-calibration support** for ERM and LRA actuators
//! - **Diagnostics mode** for actuator health monitoring
//! - **Waveform sequencing** for complex haptic patterns
//! - **Real-time playback** for custom haptic effects
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use drv260x::{Drv260x, OperatingMode};
//! use embedded_hal::i2c::I2c;
//!
//! # fn main() {
//! # let i2c = embedded_hal_mock::eh1::i2c::Mock::new(&[]);
//! let mut haptic = Drv260x::new(i2c);
//!
//! // Initialize the driver
//! haptic.init().unwrap();
//!
//! // Set operating mode to internal trigger
//! haptic.set_mode(OperatingMode::Internal).unwrap();
//!
//! // Trigger a haptic effect
//! haptic.go().unwrap();
//! # }
//! ```
//!
//! ## Async Usage
//!
//! Enable the `async` feature to use async/await patterns:
//!
//! ```toml
//! [dependencies]
//! drv260x = { version = "0.1", features = ["async"] }
//! ```
//!
//! [`embedded-hal`]: https://crates.io/crates/embedded-hal

#![no_std]
#![deny(missing_docs)]

use embedded_hal::i2c::I2c;

#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as AsyncI2c;

pub mod ll;
pub use ll::{
    AthFilter, AthPeakTime, AutoCalibTime, AutoOpenLoopCnt, FbBrakeFactor, LibrarySelection,
    LoopGain, NoiseGateThreshold, OperatingMode, SampleTime, ZeroCrossTime,
};

/// I2C address of the DRV260X family
pub const I2C_ADDRESS: u8 = ll::I2C_ADDRESS;

/// Device status information
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct StatusInfo {
    /// Overcurrent detection flag
    pub overcurrent_detected: bool,
    /// Overtemperature detection flag
    pub overtemperature_detected: bool,
    /// Diagnostic result flag (meaning depends on last operation)
    pub diagnostic_result: bool,
    /// Device identifier (3=DRV2605, 4=DRV2604, 6=DRV2604L, 7=DRV2605L)
    pub device_id: u8,
}

/// Waveform sequencer entry
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct WaveformEntry {
    /// Waveform sequence value (0-127) or wait time if wait flag is set
    pub value: u8,
    /// Wait flag - if true, value represents wait time in 10ms units
    pub is_wait: bool,
}

impl WaveformEntry {
    /// Create a new waveform entry for an effect
    pub fn effect(effect_id: u8) -> Self {
        Self {
            value: effect_id & 0x7F,
            is_wait: false,
        }
    }

    /// Create a new wait entry (wait time in 10ms units)
    pub fn wait(wait_time_10ms: u8) -> Self {
        Self {
            value: wait_time_10ms & 0x7F,
            is_wait: true,
        }
    }

    /// Create a stop entry (terminates sequence)
    pub fn stop() -> Self {
        Self {
            value: 0,
            is_wait: false,
        }
    }
}

/// All possible errors in this crate
#[derive(Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum Error<E> {
    /// I2C communication error
    I2c(E),
    /// Invalid device ID detected
    InvalidDeviceId {
        /// Expected device ID
        expected: u8,
        /// Found device ID
        found: u8,
    },
    /// Device not ready for operation
    NotReady,
    /// Invalid configuration parameter
    InvalidConfig(&'static str),
    /// Operation timeout
    Timeout,
    /// Invalid waveform sequence
    InvalidWaveform,
}

// Implement From conversion for ll::DeviceInterfaceError
impl<E> From<ll::DeviceInterfaceError<E>> for Error<E> {
    fn from(error: ll::DeviceInterfaceError<E>) -> Self {
        match error {
            ll::DeviceInterfaceError::I2c(e) => Error::I2c(e),
        }
    }
}

/// High-level DRV260X driver
pub struct Drv260x<I2C> {
    device: ll::Device<ll::DeviceInterface<I2C>>,
    // Device state tracking
    current_mode: Option<OperatingMode>,
}

impl<I2C> Drv260x<I2C> {
    /// Create a new DRV260X driver instance
    pub fn new(i2c: I2C) -> Self {
        Self {
            device: ll::Device::new(ll::DeviceInterface { i2c }),
            current_mode: None,
        }
    }

    /// Get a reference to the underlying device for advanced operations
    pub fn device(&mut self) -> &mut ll::Device<ll::DeviceInterface<I2C>> {
        &mut self.device
    }
}

impl<I2C, E> Drv260x<I2C>
where
    I2C: I2c<Error = E>,
{
    /// Initialize the driver with basic configuration
    pub fn init(&mut self) -> Result<(), Error<E>> {
        // Read and verify device ID
        let status = self.device.status().read()?;
        let device_id = status.device_id();

        // Valid device IDs: 3=DRV2605, 4=DRV2604, 6=DRV2604L, 7=DRV2605L
        match device_id {
            3 | 4 | 6 | 7 => {}
            _ => {
                return Err(Error::InvalidDeviceId {
                    expected: 3, // Use DRV2605 as example
                    found: device_id,
                });
            }
        }

        // Clear standby mode
        self.device.mode().modify(|reg| reg.set_standby(false))?;

        // Set default mode to internal trigger
        self.set_mode(OperatingMode::Internal)?;

        Ok(())
    }

    /// Get comprehensive device status information
    pub fn get_status(&mut self) -> Result<StatusInfo, Error<E>> {
        let status = self.device.status().read()?;
        Ok(StatusInfo {
            overcurrent_detected: status.oc_detect(),
            overtemperature_detected: status.over_temp(),
            diagnostic_result: status.diag_result(),
            device_id: status.device_id(),
        })
    }

    /// Set the operating mode
    pub fn set_mode(&mut self, mode: OperatingMode) -> Result<(), Error<E>> {
        self.device.mode().modify(|reg| reg.set_mode(mode))?;
        self.current_mode = Some(mode);
        Ok(())
    }

    /// Get the current operating mode
    pub fn get_mode(&mut self) -> Result<OperatingMode, Error<E>> {
        let mode_reg = self.device.mode().read()?;
        Ok(mode_reg.mode())
    }

    /// Set standby mode
    pub fn set_standby(&mut self, standby: bool) -> Result<(), Error<E>> {
        self.device.mode().modify(|reg| reg.set_standby(standby))?;
        Ok(())
    }

    /// Perform device reset
    pub fn reset(&mut self) -> Result<(), Error<E>> {
        self.device.mode().modify(|reg| reg.set_dev_reset(true))?;

        // Clear cached state after reset
        self.current_mode = None;
        Ok(())
    }

    /// Set library selection
    pub fn set_library(&mut self, library: LibrarySelection) -> Result<(), Error<E>> {
        self.device
            .library_selection()
            .modify(|reg| reg.set_library_sel(library))?;
        Ok(())
    }

    /// Set high-impedance state
    pub fn set_high_impedance(&mut self, hi_z: bool) -> Result<(), Error<E>> {
        self.device
            .library_selection()
            .modify(|reg| reg.set_hi_z(hi_z))?;
        Ok(())
    }

    /// Set a single waveform entry in the sequencer
    pub fn set_waveform_entry(&mut self, index: u8, entry: WaveformEntry) -> Result<(), Error<E>> {
        if index > 7 {
            return Err(Error::InvalidWaveform);
        }

        match index {
            0 => self.device.waveform_sequencer_0().write(|reg| {
                reg.set_wav_frm_seq(entry.value);
                reg.set_wait(entry.is_wait);
            })?,
            1 => self.device.waveform_sequencer_1().write(|reg| {
                reg.set_wav_frm_seq(entry.value);
                reg.set_wait(entry.is_wait);
            })?,
            2 => self.device.waveform_sequencer_2().write(|reg| {
                reg.set_wav_frm_seq(entry.value);
                reg.set_wait(entry.is_wait);
            })?,
            3 => self.device.waveform_sequencer_3().write(|reg| {
                reg.set_wav_frm_seq(entry.value);
                reg.set_wait(entry.is_wait);
            })?,
            4 => self.device.waveform_sequencer_4().write(|reg| {
                reg.set_wav_frm_seq(entry.value);
                reg.set_wait(entry.is_wait);
            })?,
            5 => self.device.waveform_sequencer_5().write(|reg| {
                reg.set_wav_frm_seq(entry.value);
                reg.set_wait(entry.is_wait);
            })?,
            6 => self.device.waveform_sequencer_6().write(|reg| {
                reg.set_wav_frm_seq(entry.value);
                reg.set_wait(entry.is_wait);
            })?,
            7 => self.device.waveform_sequencer_7().write(|reg| {
                reg.set_wav_frm_seq(entry.value);
                reg.set_wait(entry.is_wait);
            })?,
            _ => unreachable!(),
        }

        Ok(())
    }

    /// Set multiple waveform entries (up to 8 entries)
    pub fn set_waveform_sequence(&mut self, entries: &[WaveformEntry]) -> Result<(), Error<E>> {
        if entries.len() > 8 {
            return Err(Error::InvalidWaveform);
        }

        // Set provided entries
        for (i, &entry) in entries.iter().enumerate() {
            self.set_waveform_entry(i as u8, entry)?;
        }

        // Clear remaining entries if fewer than 8 provided
        for i in entries.len()..8 {
            self.set_waveform_entry(i as u8, WaveformEntry::stop())?;
        }

        Ok(())
    }

    /// Set a single effect in the first sequencer slot
    pub fn set_single_effect(&mut self, effect_id: u8) -> Result<(), Error<E>> {
        let sequence = [WaveformEntry::effect(effect_id), WaveformEntry::stop()];
        self.set_waveform_sequence(&sequence)
    }

    /// Trigger playback (set GO bit)
    pub fn go(&mut self) -> Result<(), Error<E>> {
        self.device.go().write(|reg| reg.set_go(true))?;
        Ok(())
    }

    /// Stop playback (clear GO bit)
    pub fn stop(&mut self) -> Result<(), Error<E>> {
        self.device.go().write(|reg| reg.set_go(false))?;
        Ok(())
    }

    /// Check if playback is active (GO bit status)
    pub fn is_active(&mut self) -> Result<bool, Error<E>> {
        let go_reg = self.device.go().read()?;
        Ok(go_reg.go())
    }

    /// Set real-time playback input value
    pub fn set_rtp_input(&mut self, value: u8) -> Result<(), Error<E>> {
        self.device
            .real_time_playback_input()
            .write(|reg| reg.set_rtp_input(value))?;
        Ok(())
    }

    /// Set rated voltage for calibration
    pub fn set_rated_voltage(&mut self, voltage: u8) -> Result<(), Error<E>> {
        self.device
            .rated_voltage()
            .write(|reg| reg.set_rated_voltage(voltage))?;
        Ok(())
    }

    /// Set overdrive clamp voltage
    pub fn set_overdrive_clamp_voltage(&mut self, voltage: u8) -> Result<(), Error<E>> {
        self.device
            .overdrive_clamp_voltage()
            .write(|reg| reg.set_od_clamp(voltage))?;
        Ok(())
    }

    /// Configure feedback control for ERM/LRA selection
    pub fn set_actuator_type(&mut self, is_lra: bool) -> Result<(), Error<E>> {
        self.device
            .feedback_control()
            .modify(|reg| reg.set_n_erm_lra(is_lra))?;
        Ok(())
    }

    /// Set feedback control parameters
    pub fn set_feedback_control(
        &mut self,
        loop_gain: LoopGain,
        brake_factor: FbBrakeFactor,
        bemf_gain: u8,
    ) -> Result<(), Error<E>> {
        self.device.feedback_control().modify(|reg| {
            reg.set_loop_gain(loop_gain);
            reg.set_fb_brake_factor(brake_factor);
            reg.set_bemf_gain(bemf_gain & 0x3); // 2-bit field
        })?;
        Ok(())
    }

    /// Start auto-calibration process
    pub fn start_auto_calibration(&mut self) -> Result<(), Error<E>> {
        // Set mode to auto-calibration
        self.set_mode(OperatingMode::AutoCalibration)?;
        // Trigger calibration
        self.go()
    }

    /// Start diagnostics process
    pub fn start_diagnostics(&mut self) -> Result<(), Error<E>> {
        // Set mode to diagnostics
        self.set_mode(OperatingMode::Diagnostics)?;
        // Trigger diagnostics
        self.go()
    }
}

#[cfg(feature = "async")]
impl<I2C, E> Drv260x<I2C>
where
    I2C: AsyncI2c<Error = E>,
{
    /// Initialize the driver with basic configuration (async version)
    pub async fn init_async(&mut self) -> Result<(), Error<E>> {
        // Read and verify device ID
        let status = self.device.status().read_async().await?;
        let device_id = status.device_id();

        // Valid device IDs: 3=DRV2605, 4=DRV2604, 6=DRV2604L, 7=DRV2605L
        match device_id {
            3 | 4 | 6 | 7 => {}
            _ => {
                return Err(Error::InvalidDeviceId {
                    expected: 3, // Use DRV2605 as example
                    found: device_id,
                });
            }
        }

        // Clear standby mode
        self.device
            .mode()
            .modify_async(|reg| reg.set_standby(false))
            .await?;

        // Set default mode to internal trigger
        self.set_mode_async(OperatingMode::Internal).await?;

        Ok(())
    }

    /// Get comprehensive device status information (async version)
    pub async fn get_status_async(&mut self) -> Result<StatusInfo, Error<E>> {
        let status = self.device.status().read_async().await?;
        Ok(StatusInfo {
            overcurrent_detected: status.oc_detect(),
            overtemperature_detected: status.over_temp(),
            diagnostic_result: status.diag_result(),
            device_id: status.device_id(),
        })
    }

    /// Set the operating mode (async version)
    pub async fn set_mode_async(&mut self, mode: OperatingMode) -> Result<(), Error<E>> {
        self.device
            .mode()
            .modify_async(|reg| reg.set_mode(mode))
            .await?;
        self.current_mode = Some(mode);
        Ok(())
    }

    /// Get the current operating mode (async version)
    pub async fn get_mode_async(&mut self) -> Result<OperatingMode, Error<E>> {
        let mode_reg = self.device.mode().read_async().await?;
        Ok(mode_reg.mode())
    }

    /// Set standby mode (async version)
    pub async fn set_standby_async(&mut self, standby: bool) -> Result<(), Error<E>> {
        self.device
            .mode()
            .modify_async(|reg| reg.set_standby(standby))
            .await?;
        Ok(())
    }

    /// Perform device reset (async version)
    pub async fn reset_async(&mut self) -> Result<(), Error<E>> {
        self.device
            .mode()
            .modify_async(|reg| reg.set_dev_reset(true))
            .await?;

        // Clear cached state after reset
        self.current_mode = None;
        Ok(())
    }

    /// Set library selection (async version)
    pub async fn set_library_async(&mut self, library: LibrarySelection) -> Result<(), Error<E>> {
        self.device
            .library_selection()
            .modify_async(|reg| reg.set_library_sel(library))
            .await?;
        Ok(())
    }

    /// Set high-impedance state (async version)
    pub async fn set_high_impedance_async(&mut self, hi_z: bool) -> Result<(), Error<E>> {
        self.device
            .library_selection()
            .modify_async(|reg| reg.set_hi_z(hi_z))
            .await?;
        Ok(())
    }

    /// Set a single waveform entry in the sequencer (async version)
    pub async fn set_waveform_entry_async(
        &mut self,
        index: u8,
        entry: WaveformEntry,
    ) -> Result<(), Error<E>> {
        if index > 7 {
            return Err(Error::InvalidWaveform);
        }

        match index {
            0 => {
                self.device
                    .waveform_sequencer_0()
                    .write_async(|reg| {
                        reg.set_wav_frm_seq(entry.value);
                        reg.set_wait(entry.is_wait);
                    })
                    .await?
            }
            1 => {
                self.device
                    .waveform_sequencer_1()
                    .write_async(|reg| {
                        reg.set_wav_frm_seq(entry.value);
                        reg.set_wait(entry.is_wait);
                    })
                    .await?
            }
            2 => {
                self.device
                    .waveform_sequencer_2()
                    .write_async(|reg| {
                        reg.set_wav_frm_seq(entry.value);
                        reg.set_wait(entry.is_wait);
                    })
                    .await?
            }
            3 => {
                self.device
                    .waveform_sequencer_3()
                    .write_async(|reg| {
                        reg.set_wav_frm_seq(entry.value);
                        reg.set_wait(entry.is_wait);
                    })
                    .await?
            }
            4 => {
                self.device
                    .waveform_sequencer_4()
                    .write_async(|reg| {
                        reg.set_wav_frm_seq(entry.value);
                        reg.set_wait(entry.is_wait);
                    })
                    .await?
            }
            5 => {
                self.device
                    .waveform_sequencer_5()
                    .write_async(|reg| {
                        reg.set_wav_frm_seq(entry.value);
                        reg.set_wait(entry.is_wait);
                    })
                    .await?
            }
            6 => {
                self.device
                    .waveform_sequencer_6()
                    .write_async(|reg| {
                        reg.set_wav_frm_seq(entry.value);
                        reg.set_wait(entry.is_wait);
                    })
                    .await?
            }
            7 => {
                self.device
                    .waveform_sequencer_7()
                    .write_async(|reg| {
                        reg.set_wav_frm_seq(entry.value);
                        reg.set_wait(entry.is_wait);
                    })
                    .await?
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    /// Set multiple waveform entries (up to 8 entries) (async version)
    pub async fn set_waveform_sequence_async(
        &mut self,
        entries: &[WaveformEntry],
    ) -> Result<(), Error<E>> {
        if entries.len() > 8 {
            return Err(Error::InvalidWaveform);
        }

        // Set provided entries
        for (i, &entry) in entries.iter().enumerate() {
            self.set_waveform_entry_async(i as u8, entry).await?;
        }

        // Clear remaining entries if fewer than 8 provided
        for i in entries.len()..8 {
            self.set_waveform_entry_async(i as u8, WaveformEntry::stop())
                .await?;
        }

        Ok(())
    }

    /// Set a single effect in the first sequencer slot (async version)
    pub async fn set_single_effect_async(&mut self, effect_id: u8) -> Result<(), Error<E>> {
        let sequence = [WaveformEntry::effect(effect_id), WaveformEntry::stop()];
        self.set_waveform_sequence_async(&sequence).await
    }

    /// Trigger playback (set GO bit) (async version)
    pub async fn go_async(&mut self) -> Result<(), Error<E>> {
        self.device.go().write_async(|reg| reg.set_go(true)).await?;
        Ok(())
    }

    /// Stop playback (clear GO bit) (async version)
    pub async fn stop_async(&mut self) -> Result<(), Error<E>> {
        self.device
            .go()
            .write_async(|reg| reg.set_go(false))
            .await?;
        Ok(())
    }

    /// Check if playback is active (GO bit status) (async version)
    pub async fn is_active_async(&mut self) -> Result<bool, Error<E>> {
        let go_reg = self.device.go().read_async().await?;
        Ok(go_reg.go())
    }

    /// Set real-time playback input value (async version)
    pub async fn set_rtp_input_async(&mut self, value: u8) -> Result<(), Error<E>> {
        self.device
            .real_time_playback_input()
            .write_async(|reg| reg.set_rtp_input(value))
            .await?;
        Ok(())
    }

    /// Set rated voltage for calibration (async version)
    pub async fn set_rated_voltage_async(&mut self, voltage: u8) -> Result<(), Error<E>> {
        self.device
            .rated_voltage()
            .write_async(|reg| reg.set_rated_voltage(voltage))
            .await?;
        Ok(())
    }

    /// Set overdrive clamp voltage (async version)
    pub async fn set_overdrive_clamp_voltage_async(&mut self, voltage: u8) -> Result<(), Error<E>> {
        self.device
            .overdrive_clamp_voltage()
            .write_async(|reg| reg.set_od_clamp(voltage))
            .await?;
        Ok(())
    }

    /// Configure feedback control for ERM/LRA selection (async version)
    pub async fn set_actuator_type_async(&mut self, is_lra: bool) -> Result<(), Error<E>> {
        self.device
            .feedback_control()
            .modify_async(|reg| reg.set_n_erm_lra(is_lra))
            .await?;
        Ok(())
    }

    /// Set feedback control parameters (async version)
    pub async fn set_feedback_control_async(
        &mut self,
        loop_gain: LoopGain,
        brake_factor: FbBrakeFactor,
        bemf_gain: u8,
    ) -> Result<(), Error<E>> {
        self.device
            .feedback_control()
            .modify_async(|reg| {
                reg.set_loop_gain(loop_gain);
                reg.set_fb_brake_factor(brake_factor);
                reg.set_bemf_gain(bemf_gain & 0x3); // 2-bit field
            })
            .await?;
        Ok(())
    }

    /// Start auto-calibration process (async version)
    pub async fn start_auto_calibration_async(&mut self) -> Result<(), Error<E>> {
        // Set mode to auto-calibration
        self.set_mode_async(OperatingMode::AutoCalibration).await?;
        // Trigger calibration
        self.go_async().await
    }

    /// Start diagnostics process (async version)
    pub async fn start_diagnostics_async(&mut self) -> Result<(), Error<E>> {
        // Set mode to diagnostics
        self.set_mode_async(OperatingMode::Diagnostics).await?;
        // Trigger diagnostics
        self.go_async().await
    }
}
