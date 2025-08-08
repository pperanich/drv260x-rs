# DRV260X Driver Roadmap

This document outlines the planned features and enhancements for the DRV260X haptic driver family crate.

## Current Status ✅

- [x] **Basic Register Access**: Complete device.yaml definition with all registers
- [x] **High-Level API Structure**: Core driver struct with initialization and basic controls
- [x] **Async Support**: Full async/await support with feature gating
- [x] **Multi-Chip Support**: Feature flags for DRV2604, DRV2604L, DRV2605, DRV2605L variants
- [x] **Waveform Sequencing**: Support for 8-entry waveform sequences with wait states
- [x] **Operating Modes**: All 8 operating modes (internal trigger, external trigger, PWM, audio-to-vibe, RTP, diagnostics, auto-calibration)
- [x] **Device Management**: Initialization, reset, standby, status monitoring

## High-Priority Features 🚀

### 1. Predefined Haptic Effects Library

**Status**: Not Started\
**Complexity**: Medium\
**Description**: Port the comprehensive effect library from the legacy DRV2605 driver (120+ effects) with proper categorization.

**Implementation Details**:

- Create `effects.rs` module with `Effect` enum containing all predefined effects (1-123)
- Group effects by category: clicks, buzzes, transitions, ramps, alerts, etc.
- Add effect duration and description metadata
- Support for effect intensity scaling
- Helper methods for common effect patterns

**Dependencies**: Core API (✅ Complete)

```rust
pub enum Effect {
    StrongClick100 = 1,
    StrongClick60 = 2,
    SharpClick100 = 4,
    // ... 120+ more effects
}

impl Effect {
    pub fn duration_ms(&self) -> u16 { /* ... */ }
    pub fn category(&self) -> EffectCategory { /* ... */ }
    pub fn description(&self) -> &'static str { /* ... */ }
}
```

### 2. Advanced Auto-Calibration API

**Status**: Not Started\
**Complexity**: High\
**Description**: Comprehensive auto-calibration support for both ERM and LRA actuators with result validation and retry logic.

**Implementation Details**:

- Pre-calibration actuator parameter validation
- Automatic rated voltage calculation helpers
- Calibration result interpretation and validation
- Calibration failure diagnosis and retry strategies
- Support for custom calibration parameters
- Integration with feedback control optimization

**Dependencies**: Core API (✅ Complete)

```rust
pub struct CalibrationConfig {
    pub actuator_type: ActuatorType,
    pub rated_voltage_mv: u16,
    pub overdrive_voltage_mv: u16,
    pub cal_time: AutoCalibTime,
}

impl Drv260x<I2C> {
    pub async fn calibrate_actuator(&mut self, config: CalibrationConfig) -> Result<CalibrationResult, Error<E>>;
    pub fn validate_calibration_result(&self, result: &CalibrationResult) -> CalibrationStatus;
}
```

### 3. Real-Time Playback (RTP) Utilities

**Status**: Not Started\
**Complexity**: Medium\
**Description**: High-level utilities for real-time haptic control with waveform generation and streaming support.

**Implementation Details**:

- Waveform generation utilities (sine, triangle, sawtooth, noise)
- Amplitude envelope support (ADSR, fade-in/out)
- Sample rate control and timing utilities
- Streaming interface for continuous RTP data
- Integration with embedded timer abstractions

**Dependencies**: Core API (✅ Complete)

```rust
pub struct RtpController<I2C> {
    driver: Drv260x<I2C>,
    sample_rate_hz: u32,
}

impl<I2C> RtpController<I2C> {
    pub async fn stream_waveform(&mut self, waveform: &[u8]) -> Result<(), Error<E>>;
    pub async fn generate_sine_wave(&mut self, freq_hz: f32, duration_ms: u16) -> Result<(), Error<E>>;
    pub async fn apply_envelope(&mut self, envelope: Envelope) -> Result<(), Error<E>>;
}
```

### 4. Audio-to-Vibe Configuration

**Status**: Not Started\
**Complexity**: Medium\
**Description**: Advanced audio-to-haptic conversion with configurable filters and response curves.

**Implementation Details**:

- Audio input level calibration utilities
- Configurable filter settings (HPF, LPF, bandpass)
- Dynamic range compression controls
- Peak detection and response tuning
- Audio source compatibility testing utilities

**Dependencies**: Core API (✅ Complete)

```rust
pub struct AudioToVibeConfig {
    pub filter: AthFilter,
    pub peak_time: AthPeakTime,
    pub min_input_level: u8,
    pub max_input_level: u8,
    pub min_output_drive: u8,
    pub max_output_drive: u8,
}

impl Drv260x<I2C> {
    pub async fn configure_audio_to_vibe(&mut self, config: AudioToVibeConfig) -> Result<(), Error<E>>;
    pub async fn calibrate_audio_levels(&mut self) -> Result<AudioCalibrationResult, Error<E>>;
}
```

## Medium-Priority Features 📋

### 5. Advanced Diagnostics and Health Monitoring

**Status**: Not Started\
**Complexity**: Medium\
**Description**: Comprehensive actuator health monitoring with predictive maintenance capabilities.

**Implementation Details**:

- Extended diagnostic routines beyond basic pass/fail
- Actuator impedance measurement and trending
- Temperature monitoring and thermal management
- Supply voltage monitoring and brown-out detection
- Diagnostic result interpretation with recommended actions

**Dependencies**: Core API (✅ Complete)

### 6. Complex Waveform Composition

**Status**: Not Started\
**Complexity**: Medium\
**Description**: Tools for creating complex haptic patterns by combining and layering effects.

**Implementation Details**:

- Waveform sequencer with conditional logic
- Effect layering and blending capabilities
- Pattern templates and macro support
- Timing synchronization utilities
- Pattern validation and optimization

**Dependencies**: Effect Library, Core API

### 7. Power Management Optimization

**Status**: Not Started\
**Complexity**: Medium\
**Description**: Advanced power management features for battery-powered applications.

**Implementation Details**:

- Intelligent standby mode management
- Power consumption profiling tools
- Battery voltage monitoring integration
- Low-power effect alternatives
- Wake-on-trigger functionality

**Dependencies**: Core API (✅ Complete)

## Low-Priority Features 🔮

### 8. Haptic Pattern Editor/Builder

**Status**: Not Started\
**Complexity**: High\
**Description**: Build-time tools for creating and validating complex haptic patterns.

### 9. Platform Integration Helpers

**Status**: Not Started\
**Complexity**: Low\
**Description**: Platform-specific integration helpers for common embedded systems.

### 10. Haptic Accessibility Features

**Status**: Not Started\
**Complexity**: Medium\
**Description**: Accessibility-focused haptic patterns for UI/UX applications.

## Chip Variant Differences 🔧

Once the base implementation is complete, chip-specific features will be added:

- **DRV2605 vs DRV2604**: ROM library vs RAM-based waveforms
- **Low-voltage variants (L)**: Voltage scaling and protection features
- **Register differences**: Optional registers with cfg attributes
- **Performance optimizations**: Chip-specific calibration and timing parameters

## Testing Strategy 🧪

- **Unit tests**: Core API functionality with mock I2C
- **Integration tests**: Real hardware validation on development boards
- **Example applications**: Demonstration programs for each major feature
- **Benchmarking**: Performance and power consumption characterization
- **Documentation**: Comprehensive API documentation with usage examples

## Contributing 🤝

This roadmap is a living document. Features can be prioritized based on:

- Community feedback and use cases
- Hardware availability for testing
- Contributor interest and availability
- Integration requirements with other crates

Each feature should include:

- Comprehensive documentation
- Usage examples
- Unit tests where applicable
- Integration with existing async/sync APIs
- Backward compatibility considerations
