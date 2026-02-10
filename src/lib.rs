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
//! ```rust,ignore
//! use drv260x::{Drv260x, OperatingMode};
//! use embedded_hal::i2c::I2c;
//!
//! let mut haptic = Drv260x::new(i2c);
//!
//! // Initialize the driver
//! haptic.init().unwrap();
//!
//! // Set operating mode to internal trigger
//! haptic.set_mode(OperatingMode::Internal).unwrap();
//!
//! // Play an effect by raw ID (works on all variants)
//! haptic.set_single_effect(1).unwrap();
//! haptic.go().unwrap();
//!
//! // On DRV2605/DRV2605L, use the predefined Effect enum:
//! // use drv260x::Effect;
//! // haptic.set_single_effect_enum(Effect::StrongClick100).unwrap();
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

#[cfg(not(any(
    feature = "drv2604",
    feature = "drv2604l",
    feature = "drv2605",
    feature = "drv2605l"
)))]
compile_error!(
    "Exactly one device feature must be enabled: drv2604, drv2604l, drv2605, or drv2605l"
);

// Module declarations
#[cfg(feature = "async")]
mod async_impl;
pub mod effects;
pub mod ll;
mod sync_impl;

// Re-export the low-level types from ll module
pub use ll::{
    AutoCalibTime, AutoOpenLoopCnt, FbBrakeFactor, LoopGain, NoiseGateThreshold, OperatingMode,
    SampleTime, ZeroCrossTime,
};

// Re-export ROM-only types (audio-to-vibe, library selection)
#[cfg(any(feature = "drv2605", feature = "drv2605l"))]
pub use ll::{AthFilter, AthPeakTime, LibrarySelection};

// Re-export the effects and waveform types from effects module
pub use effects::WaveformEntry;

#[cfg(any(feature = "drv2605", feature = "drv2605l"))]
pub use effects::Effect;

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
    /// Feedback status (DRV2604/DRV2605 only, reserved on L-variants)
    pub feedback_status: bool,
    /// Diagnostic result flag (meaning depends on last operation)
    pub diagnostic_result: bool,
    /// Illegal address detection flag (DRV2604/DRV2604L only, reserved on DRV2605/DRV2605L)
    pub illegal_address: bool,
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
    device: ll::Registers<ll::DeviceInterface<I2C>>,
    // Device state tracking
    current_mode: Option<OperatingMode>,
}

impl<I2C> Drv260x<I2C> {
    /// Create a new DRV260X driver instance
    pub fn new(i2c: I2C) -> Self {
        Self {
            device: ll::Registers::new(ll::DeviceInterface { i2c }),
            current_mode: None,
        }
    }

    /// Get a reference to the underlying device for advanced operations
    pub fn device(&mut self) -> &mut ll::Registers<ll::DeviceInterface<I2C>> {
        &mut self.device
    }
}

// The sync and async implementations are now in separate modules and are
// automatically included via the module system. This makes lib.rs much cleaner
// and more maintainable.
