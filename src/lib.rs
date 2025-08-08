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
//! - **Predefined effects library** with 123 built-in haptic effects
//! - **Waveform timing control** for fine-tuning library effects
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use drv260x::{Drv260x, OperatingMode, Effect};
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
//! // Play a predefined effect
//! haptic.set_single_effect_enum(Effect::StrongClick100).unwrap();
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

// Module declarations
#[cfg(feature = "async")]
mod async_impl;
pub mod effects;
pub mod ll;
mod sync_impl;

// Re-export the low-level types from ll module
pub use ll::{
    AthFilter, AthPeakTime, AutoCalibTime, AutoOpenLoopCnt, FbBrakeFactor, LibrarySelection,
    LoopGain, NoiseGateThreshold, OperatingMode, SampleTime, ZeroCrossTime,
};

// Re-export the effects and waveform types from effects module
pub use effects::{Effect, WaveformEntry};

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

// The sync and async implementations are now in separate modules and are
// automatically included via the module system. This makes lib.rs much cleaner
// and more maintainable.
