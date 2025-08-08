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

/// Predefined haptic effects from the DRV260X ROM library
///
/// These effects are pre-programmed waveforms stored in the device's ROM.
/// Each effect has a specific intensity and characteristic designed for
/// different haptic feedback scenarios.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
#[repr(u8)]
pub enum Effect {
    /// Strong Click - 100%
    StrongClick100 = 1,
    /// Strong Click - 60%
    StrongClick60 = 2,
    /// Strong Click - 30%
    StrongClick30 = 3,
    /// Sharp Click - 100%
    SharpClick100 = 4,
    /// Sharp Click - 60%
    SharpClick60 = 5,
    /// Sharp Click - 30%
    SharpClick30 = 6,
    /// Soft Bump - 100%
    SoftBump100 = 7,
    /// Soft Bump - 60%
    SoftBump60 = 8,
    /// Soft Bump - 30%
    SoftBump30 = 9,
    /// Double Click - 100%
    DoubleClick100 = 10,
    /// Double Click - 60%
    DoubleClick60 = 11,
    /// Triple Click - 100%
    TripleClick100 = 12,
    /// Soft Fuzz - 60%
    SoftFuzz60 = 13,
    /// Strong Buzz - 100%
    StrongBuzz100 = 14,
    /// 750 ms Alert 100%
    Alert750ms = 15,
    /// 1000 ms Alert 100%
    Alert1000ms = 16,
    /// Strong Click 1 - 100%
    StrongClick1_100 = 17,
    /// Strong Click 2 - 80%
    StrongClick2_80 = 18,
    /// Strong Click 3 - 60%
    StrongClick3_60 = 19,
    /// Strong Click 4 - 30%
    StrongClick4_30 = 20,
    /// Medium Click 1 - 100%
    MediumClick1_100 = 21,
    /// Medium Click 2 - 80%
    MediumClick2_80 = 22,
    /// Medium Click 3 - 60%
    MediumClick3_60 = 23,
    /// Sharp Tick 1 - 100%
    SharpTick1_100 = 24,
    /// Sharp Tick 2 - 80%
    SharpTick2_80 = 25,
    /// Sharp Tick 3 - 60%
    SharpTick3_60 = 26,
    /// Short Double Click Strong 1 - 100%
    ShortDoubleClickStrong1_100 = 27,
    /// Short Double Click Strong 2 - 80%
    ShortDoubleClickStrong2_80 = 28,
    /// Short Double Click Strong 3 - 60%
    ShortDoubleClickStrong3_60 = 29,
    /// Short Double Click Strong 4 - 30%
    ShortDoubleClickStrong4_30 = 30,
    /// Short Double Click Medium 1 - 100%
    ShortDoubleClickMedium1_100 = 31,
    /// Short Double Click Medium 2 - 80%
    ShortDoubleClickMedium2_80 = 32,
    /// Short Double Click Medium 3 - 60%
    ShortDoubleClickMedium3_60 = 33,
    /// Short Double Sharp Tick 1 - 100%
    ShortDoubleSharpTick1_100 = 34,
    /// Short Double Sharp Tick 2 - 80%
    ShortDoubleSharpTick2_80 = 35,
    /// Short Double Sharp Tick 3 - 60%
    ShortDoubleSharpTick3_60 = 36,
    /// Long Double Sharp Click Strong 1 - 100%
    LongDoubleSharpClickStrong1_100 = 37,
    /// Long Double Sharp Click Strong 2 - 80%
    LongDoubleSharpClickStrong2_80 = 38,
    /// Long Double Sharp Click Strong 3 - 60%
    LongDoubleSharpClickStrong3_60 = 39,
    /// Long Double Sharp Click Strong 4 - 30%
    LongDoubleSharpClickStrong4_30 = 40,
    /// Long Double Sharp Click Medium 1 - 100%
    LongDoubleSharpClickMedium1_100 = 41,
    /// Long Double Sharp Click Medium 2 - 80%
    LongDoubleSharpClickMedium2_80 = 42,
    /// Long Double Sharp Click Medium 3 - 60%
    LongDoubleSharpClickMedium3_60 = 43,
    /// Long Double Sharp Tick 1 - 100%
    LongDoubleSharpTick1_100 = 44,
    /// Long Double Sharp Tick 2 - 80%
    LongDoubleSharpTick2_80 = 45,
    /// Long Double Sharp Tick 3 - 60%
    LongDoubleSharpTick3_60 = 46,
    /// Buzz 1 - 100%
    Buzz1_100 = 47,
    /// Buzz 2 - 80%
    Buzz2_80 = 48,
    /// Buzz 3 - 60%
    Buzz3_60 = 49,
    /// Buzz 4 - 40%
    Buzz4_40 = 50,
    /// Buzz 5 - 20%
    Buzz5_20 = 51,
    /// Pulsing Strong 1 - 100%
    PulsingStrong1_100 = 52,
    /// Pulsing Strong 2 - 60%
    PulsingStrong2_60 = 53,
    /// Pulsing Medium 1 - 100%
    PulsingMedium1_100 = 54,
    /// Pulsing Medium 2 - 60%
    PulsingMedium2_60 = 55,
    /// Pulsing Sharp 1 - 100%
    PulsingSharp1_100 = 56,
    /// Pulsing Sharp 2 - 60%
    PulsingSharp2_60 = 57,
    /// Transition Click 1 - 100%
    TransitionClick1_100 = 58,
    /// Transition Click 2 - 80%
    TransitionClick2_80 = 59,
    /// Transition Click 3 - 60%
    TransitionClick3_60 = 60,
    /// Transition Click 4 - 40%
    TransitionClick4_40 = 61,
    /// Transition Click 5 - 20%
    TransitionClick5_20 = 62,
    /// Transition Click 6 - 10%
    TransitionClick6_10 = 63,
    /// Transition Hum 1 - 100%
    TransitionHum1_100 = 64,
    /// Transition Hum 2 - 80%
    TransitionHum2_80 = 65,
    /// Transition Hum 3 - 60%
    TransitionHum3_60 = 66,
    /// Transition Hum 4 - 40%
    TransitionHum4_40 = 67,
    /// Transition Hum 5 - 20%
    TransitionHum5_20 = 68,
    /// Transition Hum 6 - 10%
    TransitionHum6_10 = 69,
    /// Transition Ramp Down Long Smooth 1 - 100 to 0%
    TransitionRampDownLongSmooth1_100to0 = 70,
    /// Transition Ramp Down Long Smooth 2 - 100 to 0%
    TransitionRampDownLongSmooth2_100to0 = 71,
    /// Transition Ramp Down Medium Smooth 1 - 100 to 0%
    TransitionRampDownMediumSmooth1_100to0 = 72,
    /// Transition Ramp Down Medium Smooth 2 - 100 to 0%
    TransitionRampDownMediumSmooth2_100to0 = 73,
    /// Transition Ramp Down Short Smooth 1 - 100 to 0%
    TransitionRampDownShortSmooth1_100to0 = 74,
    /// Transition Ramp Down Short Smooth 2 - 100 to 0%
    TransitionRampDownShortSmooth2_100to0 = 75,
    /// Transition Ramp Down Long Sharp 1 - 100 to 0%
    TransitionRampDownLongSharp1_100to0 = 76,
    /// Transition Ramp Down Long Sharp 2 - 100 to 0%
    TransitionRampDownLongSharp2_100to0 = 77,
    /// Transition Ramp Down Medium Sharp 1 - 100 to 0%
    TransitionRampDownMediumSharp1_100to0 = 78,
    /// Transition Ramp Down Medium Sharp 2 - 100 to 0%
    TransitionRampDownMediumSharp2_100to0 = 79,
    /// Transition Ramp Down Short Sharp 1 - 100 to 0%
    TransitionRampDownShortSharp1_100to0 = 80,
    /// Transition Ramp Down Short Sharp 2 - 100 to 0%
    TransitionRampDownShortSharp2_100to0 = 81,
    /// Transition Ramp Up Long Smooth 1 - 0 to 100%
    TransitionRampUpLongSmooth1_0to100 = 82,
    /// Transition Ramp Up Long Smooth 2 - 0 to 100%
    TransitionRampUpLongSmooth2_0to100 = 83,
    /// Transition Ramp Up Medium Smooth 1 - 0 to 100%
    TransitionRampUpMediumSmooth1_0to100 = 84,
    /// Transition Ramp Up Medium Smooth 2 - 0 to 100%
    TransitionRampUpMediumSmooth2_0to100 = 85,
    /// Transition Ramp Up Short Smooth 1 - 0 to 100%
    TransitionRampUpShortSmooth1_0to100 = 86,
    /// Transition Ramp Up Short Smooth 2 - 0 to 100%
    TransitionRampUpShortSmooth2_0to100 = 87,
    /// Transition Ramp Up Long Sharp 1 - 0 to 100%
    TransitionRampUpLongSharp1_0to100 = 88,
    /// Transition Ramp Up Long Sharp 2 - 0 to 100%
    TransitionRampUpLongSharp2_0to100 = 89,
    /// Transition Ramp Up Medium Sharp 1 - 0 to 100%
    TransitionRampUpMediumSharp1_0to100 = 90,
    /// Transition Ramp Up Medium Sharp 2 - 0 to 100%
    TransitionRampUpMediumSharp2_0to100 = 91,
    /// Transition Ramp Up Short Sharp 1 - 0 to 100%
    TransitionRampUpShortSharp1_0to100 = 92,
    /// Transition Ramp Up Short Sharp 2 - 0 to 100%
    TransitionRampUpShortSharp2_0to100 = 93,
    /// Transition Ramp Down Long Smooth 1 - 50 to 0%
    TransitionRampDownLongSmooth1_50to0 = 94,
    /// Transition Ramp Down Long Smooth 2 - 50 to 0%
    TransitionRampDownLongSmooth2_50to0 = 95,
    /// Transition Ramp Down Medium Smooth 1 - 50 to 0%
    TransitionRampDownMediumSmooth1_50to0 = 96,
    /// Transition Ramp Down Medium Smooth 2 - 50 to 0%
    TransitionRampDownMediumSmooth2_50to0 = 97,
    /// Transition Ramp Down Short Smooth 1 - 50 to 0%
    TransitionRampDownShortSmooth1_50to0 = 98,
    /// Transition Ramp Down Short Smooth 2 - 50 to 0%
    TransitionRampDownShortSmooth2_50to0 = 99,
    /// Transition Ramp Down Long Sharp 1 - 50 to 0%
    TransitionRampDownLongSharp1_50to0 = 100,
    /// Transition Ramp Down Long Sharp 2 - 50 to 0%
    TransitionRampDownLongSharp2_50to0 = 101,
    /// Transition Ramp Down Medium Sharp 1 - 50 to 0%
    TransitionRampDownMediumSharp1_50to0 = 102,
    /// Transition Ramp Down Medium Sharp 2 - 50 to 0%
    TransitionRampDownMediumSharp2_50to0 = 103,
    /// Transition Ramp Down Short Sharp 1 - 50 to 0%
    TransitionRampDownShortSharp1_50to0 = 104,
    /// Transition Ramp Down Short Sharp 2 - 50 to 0%
    TransitionRampDownShortSharp2_50to0 = 105,
    /// Transition Ramp Up Long Smooth 1 - 0 to 50%
    TransitionRampUpLongSmooth1_0to50 = 106,
    /// Transition Ramp Up Long Smooth 2 - 0 to 50%
    TransitionRampUpLongSmooth2_0to50 = 107,
    /// Transition Ramp Up Medium Smooth 1 - 0 to 50%
    TransitionRampUpMediumSmooth1_0to50 = 108,
    /// Transition Ramp Up Medium Smooth 2 - 0 to 50%
    TransitionRampUpMediumSmooth2_0to50 = 109,
    /// Transition Ramp Up Short Smooth 1 - 0 to 50%
    TransitionRampUpShortSmooth1_0to50 = 110,
    /// Transition Ramp Up Short Smooth 2 - 0 to 50%
    TransitionRampUpShortSmooth2_0to50 = 111,
    /// Transition Ramp Up Long Sharp 1 - 0 to 50%
    TransitionRampUpLongSharp1_0to50 = 112,
    /// Transition Ramp Up Long Sharp 2 - 0 to 50%
    TransitionRampUpLongSharp2_0to50 = 113,
    /// Transition Ramp Up Medium Sharp 1 - 0 to 50%
    TransitionRampUpMediumSharp1_0to50 = 114,
    /// Transition Ramp Up Medium Sharp 2 - 0 to 50%
    TransitionRampUpMediumSharp2_0to50 = 115,
    /// Transition Ramp Up Short Sharp 1 - 0 to 50%
    TransitionRampUpShortSharp1_0to50 = 116,
    /// Transition Ramp Up Short Sharp 2 - 0 to 50%
    TransitionRampUpShortSharp2_0to50 = 117,
    /// Long Buzz For Programmatic Stopping - 100%
    LongBuzzForProgrammaticStopping100 = 118,
    /// Smooth Hum 1 (No kick or brake pulse) - 50%
    SmoothHum1_50 = 119,
    /// Smooth Hum 2 (No kick or brake pulse) - 40%
    SmoothHum2_40 = 120,
    /// Smooth Hum 3 (No kick or brake pulse) - 30%
    SmoothHum3_30 = 121,
    /// Smooth Hum 4 (No kick or brake pulse) - 20%
    SmoothHum4_20 = 122,
    /// Smooth Hum 5 (No kick or brake pulse) - 10%
    SmoothHum5_10 = 123,
}

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
    /// Create a new waveform entry for an effect using effect ID
    pub fn effect(effect_id: u8) -> Self {
        Self {
            value: effect_id & 0x7F,
            is_wait: false,
        }
    }

    /// Create a new waveform entry for a predefined effect
    pub fn effect_from_enum(effect: Effect) -> Self {
        Self {
            value: effect as u8,
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

impl From<Effect> for WaveformEntry {
    fn from(effect: Effect) -> Self {
        Self::effect_from_enum(effect)
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

    /// Initialize the driver for ERM actuator in open-loop mode
    ///
    /// This is a convenience method that configures the device for ERM (Eccentric Rotating Mass)
    /// actuators in open-loop operation mode. It performs device initialization and sets up
    /// appropriate default configuration.
    pub fn init_open_loop_erm(&mut self) -> Result<(), Error<E>> {
        // Initialize the device first
        self.init()?;

        // Set up for ERM actuator
        self.set_actuator_type(false)?; // false = ERM mode

        // Configure for open-loop operation using low-level access
        self.device.control_3().modify(|reg| {
            reg.set_erm_open_loop(true); // Enable ERM open-loop mode
        })?;

        // Set a default waveform (strong click)
        self.set_single_effect(1)?;

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

    /// Set a single predefined effect in the first sequencer slot
    pub fn set_single_effect_enum(&mut self, effect: Effect) -> Result<(), Error<E>> {
        let sequence = [WaveformEntry::from(effect), WaveformEntry::stop()];
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

    /// Set overdrive time offset for library waveforms
    ///
    /// This adds a time offset to the overdrive portion of library waveforms.
    /// The offset is interpreted as 2's complement, so it can be positive or negative.
    /// Overdrive Time Offset (ms) = value × PLAYBACK_INTERVAL
    /// This register is only useful in open-loop mode.
    pub fn set_overdrive_time_offset(&mut self, offset: i8) -> Result<(), Error<E>> {
        self.device
            .overdrive_time_offset()
            .write(|reg| reg.set_odt(offset as u8))?;
        Ok(())
    }

    /// Set positive sustain time offset for library waveforms
    ///
    /// This adds a time offset to the positive sustain portion of library waveforms.
    /// The offset is interpreted as 2's complement, so it can be positive or negative.
    /// Sustain-Time Positive Offset (ms) = value × PLAYBACK_INTERVAL
    pub fn set_sustain_time_offset_positive(&mut self, offset: i8) -> Result<(), Error<E>> {
        self.device
            .sustain_time_offset_pos()
            .write(|reg| reg.set_spt(offset as u8))?;
        Ok(())
    }

    /// Set negative sustain time offset for library waveforms
    ///
    /// This adds a time offset to the negative sustain portion of library waveforms.
    /// The offset is interpreted as 2's complement, so it can be positive or negative.
    /// Sustain-Time Negative Offset (ms) = value × PLAYBACK_INTERVAL
    pub fn set_sustain_time_offset_negative(&mut self, offset: i8) -> Result<(), Error<E>> {
        self.device
            .sustain_time_offset_neg()
            .write(|reg| reg.set_snt(offset as u8))?;
        Ok(())
    }

    /// Set brake time offset for library waveforms
    ///
    /// This adds a time offset to the braking portion of library waveforms.
    /// The offset is interpreted as 2's complement, so it can be positive or negative.
    /// Brake Time Offset (ms) = value × PLAYBACK_INTERVAL
    /// This register is only useful in open-loop mode.
    pub fn set_brake_time_offset(&mut self, offset: i8) -> Result<(), Error<E>> {
        self.device
            .brake_time_offset()
            .write(|reg| reg.set_brt(offset as u8))?;
        Ok(())
    }

    /// Configure audio-to-vibe control settings
    ///
    /// This method configures the audio-to-haptic conversion filter and peak time settings.
    pub fn set_audio_to_vibe_control(
        &mut self,
        filter: AthFilter,
        peak_time: AthPeakTime,
    ) -> Result<(), Error<E>> {
        self.device.audio_to_vibe_control().modify(|reg| {
            reg.set_ath_filter(filter);
            reg.set_ath_peak_time(peak_time);
        })?;
        Ok(())
    }

    /// Set audio-to-vibe minimum input level
    ///
    /// Sets the minimum input level for audio-to-haptic conversion.
    pub fn set_audio_to_vibe_min_input_level(&mut self, level: u8) -> Result<(), Error<E>> {
        self.device
            .audio_to_vibe_min_input_level()
            .write(|reg| reg.set_ath_min_input(level))?;
        Ok(())
    }

    /// Set audio-to-vibe maximum input level
    ///
    /// Sets the maximum input level for audio-to-haptic conversion.
    pub fn set_audio_to_vibe_max_input_level(&mut self, level: u8) -> Result<(), Error<E>> {
        self.device
            .audio_to_vibe_max_input_level()
            .write(|reg| reg.set_ath_max_input(level))?;
        Ok(())
    }

    /// Set audio-to-vibe minimum output drive
    ///
    /// Sets the minimum output drive level for audio-to-haptic conversion.
    pub fn set_audio_to_vibe_min_output_drive(&mut self, level: u8) -> Result<(), Error<E>> {
        self.device
            .audio_to_vibe_min_output_drive()
            .write(|reg| reg.set_ath_min_drive(level))?;
        Ok(())
    }

    /// Set audio-to-vibe maximum output drive
    ///
    /// Sets the maximum output drive level for audio-to-haptic conversion.
    pub fn set_audio_to_vibe_max_output_drive(&mut self, level: u8) -> Result<(), Error<E>> {
        self.device
            .audio_to_vibe_max_output_drive()
            .write(|reg| reg.set_ath_max_drive(level))?;
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

    /// Initialize the driver for ERM actuator in open-loop mode (async version)
    pub async fn init_open_loop_erm_async(&mut self) -> Result<(), Error<E>> {
        // Initialize the device first
        self.init_async().await?;

        // Set up for ERM actuator
        self.set_actuator_type_async(false).await?; // false = ERM mode

        // Configure for open-loop operation using low-level access
        self.device
            .control_3()
            .modify_async(|reg| {
                reg.set_erm_open_loop(true); // Enable ERM open-loop mode
            })
            .await?;

        // Set a default waveform (strong click)
        self.set_single_effect_async(1).await?;

        Ok(())
    }

    /// Set a single predefined effect in the first sequencer slot (async version)
    pub async fn set_single_effect_enum_async(&mut self, effect: Effect) -> Result<(), Error<E>> {
        let sequence = [WaveformEntry::from(effect), WaveformEntry::stop()];
        self.set_waveform_sequence_async(&sequence).await
    }

    /// Set overdrive time offset for library waveforms (async version)
    pub async fn set_overdrive_time_offset_async(&mut self, offset: i8) -> Result<(), Error<E>> {
        self.device
            .overdrive_time_offset()
            .write_async(|reg| reg.set_odt(offset as u8))
            .await?;
        Ok(())
    }

    /// Set positive sustain time offset for library waveforms (async version)
    pub async fn set_sustain_time_offset_positive_async(
        &mut self,
        offset: i8,
    ) -> Result<(), Error<E>> {
        self.device
            .sustain_time_offset_pos()
            .write_async(|reg| reg.set_spt(offset as u8))
            .await?;
        Ok(())
    }

    /// Set negative sustain time offset for library waveforms (async version)
    pub async fn set_sustain_time_offset_negative_async(
        &mut self,
        offset: i8,
    ) -> Result<(), Error<E>> {
        self.device
            .sustain_time_offset_neg()
            .write_async(|reg| reg.set_snt(offset as u8))
            .await?;
        Ok(())
    }

    /// Set brake time offset for library waveforms (async version)
    pub async fn set_brake_time_offset_async(&mut self, offset: i8) -> Result<(), Error<E>> {
        self.device
            .brake_time_offset()
            .write_async(|reg| reg.set_brt(offset as u8))
            .await?;
        Ok(())
    }

    /// Configure audio-to-vibe control settings (async version)
    pub async fn set_audio_to_vibe_control_async(
        &mut self,
        filter: AthFilter,
        peak_time: AthPeakTime,
    ) -> Result<(), Error<E>> {
        self.device
            .audio_to_vibe_control()
            .modify_async(|reg| {
                reg.set_ath_filter(filter);
                reg.set_ath_peak_time(peak_time);
            })
            .await?;
        Ok(())
    }

    /// Set audio-to-vibe minimum input level (async version)
    pub async fn set_audio_to_vibe_min_input_level_async(
        &mut self,
        level: u8,
    ) -> Result<(), Error<E>> {
        self.device
            .audio_to_vibe_min_input_level()
            .write_async(|reg| reg.set_ath_min_input(level))
            .await?;
        Ok(())
    }

    /// Set audio-to-vibe maximum input level (async version)
    pub async fn set_audio_to_vibe_max_input_level_async(
        &mut self,
        level: u8,
    ) -> Result<(), Error<E>> {
        self.device
            .audio_to_vibe_max_input_level()
            .write_async(|reg| reg.set_ath_max_input(level))
            .await?;
        Ok(())
    }

    /// Set audio-to-vibe minimum output drive (async version)
    pub async fn set_audio_to_vibe_min_output_drive_async(
        &mut self,
        level: u8,
    ) -> Result<(), Error<E>> {
        self.device
            .audio_to_vibe_min_output_drive()
            .write_async(|reg| reg.set_ath_min_drive(level))
            .await?;
        Ok(())
    }

    /// Set audio-to-vibe maximum output drive (async version)
    pub async fn set_audio_to_vibe_max_output_drive_async(
        &mut self,
        level: u8,
    ) -> Result<(), Error<E>> {
        self.device
            .audio_to_vibe_max_output_drive()
            .write_async(|reg| reg.set_ath_max_drive(level))
            .await?;
        Ok(())
    }
}
