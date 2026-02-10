# DRV260X Driver Roadmap

This document outlines the planned features and enhancements for the DRV260X haptic driver family crate.

## Current Status ✅

- [x] **Basic Register Access**: Complete device.yaml definition with all registers
- [x] **High-Level API Structure**: Core driver struct with initialization and basic controls
- [x] **Async Support**: Full async/await support with feature gating
- [x] **Multi-Chip Support**: Feature flags for DRV2604, DRV2604L, DRV2605, DRV2605L variants with compile-time API gating
- [x] **Waveform Sequencing**: Support for 8-entry waveform sequences with wait states
- [x] **Operating Modes**: All 8 operating modes (internal trigger, external trigger, PWM, audio-to-vibe, RTP, diagnostics, auto-calibration)
- [x] **Device Management**: Initialization, reset, standby, status monitoring
- [x] **Modular Architecture**: Clean separation of sync/async implementations and feature modules
- [x] **Predefined Effects Library**: Complete Effect enum with 123+ haptic effects ⭐ **NEW**
- [x] **Waveform Timing Control**: Fine-tuning methods for library waveform timing ⭐ **NEW**
- [x] **Audio-to-Vibe Configuration**: Complete audio-to-haptic conversion controls ⭐ **NEW**
- [x] **Convenience Initialization**: ERM open-loop setup with single method call ⭐ **NEW**

## Recently Completed Features 🎉

### ✅ 1. Predefined Haptic Effects Library

**Status**: ✅ **COMPLETED**\
**Implementation**: `src/effects.rs` module

**What was delivered**:

- Complete `Effect` enum with 123 predefined haptic effects (1-123)
- Effects grouped by category: clicks, buzzes, transitions, ramps, alerts, etc.
- `WaveformEntry::from(Effect)` conversion for seamless usage
- New `set_single_effect_enum(Effect)` convenience method
- Both sync and async versions of all effect methods

```rust
pub enum Effect {
    StrongClick100 = 1,
    SharpClick60 = 5,
    SoftBump100 = 7,
    DoubleClick100 = 10,
    Buzz1_100 = 47,
    TransitionRampUpLongSmooth1_0to100 = 82,
    SmoothHum1_50 = 119,
    // ... 123 total effects
}

// Usage
haptic.set_single_effect_enum(Effect::StrongClick100)?;
let sequence = [
    WaveformEntry::from(Effect::SharpClick100),
    WaveformEntry::wait(5),
    WaveformEntry::from(Effect::SoftBump60),
];
```

### ✅ 2. Waveform Timing Control

**Status**: ✅ **COMPLETED**\
**Implementation**: `src/sync_impl.rs` and `src/async_impl.rs`

**What was delivered**:

- Fine-tuning methods for library waveform timing characteristics
- Support for signed offset values (positive/negative adjustments)
- Complete sync and async implementations

```rust
// Fine-tune waveform characteristics
haptic.set_overdrive_time_offset(5)?;           // Extend overdrive
haptic.set_sustain_time_offset_positive(10)?;   // Extend positive sustain
haptic.set_sustain_time_offset_negative(-2)?;   // Reduce negative sustain
haptic.set_brake_time_offset(3)?;               // Extend braking
```

### ✅ 3. Audio-to-Vibe Configuration

**Status**: ✅ **COMPLETED**\
**Implementation**: `src/sync_impl.rs` and `src/async_impl.rs`

**What was delivered**:

- Complete audio-to-haptic conversion configuration
- Filter and peak time control
- Input/output level management
- Both sync and async API variants

```rust
// Configure audio-to-haptic conversion
haptic.set_audio_to_vibe_control(AthFilter::Hz150, AthPeakTime::Ms20)?;
haptic.set_audio_to_vibe_min_input_level(0x19)?;
haptic.set_audio_to_vibe_max_input_level(0xFF)?;
haptic.set_audio_to_vibe_min_output_drive(0x19)?;
haptic.set_audio_to_vibe_max_output_drive(0xFF)?;
```

### ✅ 4. Convenience Initialization

**Status**: ✅ **COMPLETED**\
**Implementation**: `src/sync_impl.rs` and `src/async_impl.rs`

**What was delivered**:

- One-call ERM actuator initialization for open-loop mode
- Automatic configuration of device settings
- Both sync and async versions

```rust
// Single call to initialize ERM in open-loop mode
haptic.init_open_loop_erm()?;
// Equivalent to manual: init() + set_actuator_type(false) + configure ERM open-loop + set default effect
```

## Architecture Improvements 🏗️

### ✅ Modular Codebase Structure

**Status**: ✅ **COMPLETED**

The codebase has been completely restructured for maintainability:

```
src/
├── lib.rs              # Main library entry point (150 lines, was 1250+)
├── effects.rs          # Effect enum and waveform utilities (300 lines)
├── sync_impl.rs        # Synchronous driver implementation (350 lines)
├── async_impl.rs       # Asynchronous driver implementation (400 lines)
└── ll.rs               # Low-level device interface (generated)
```

**Benefits**:

- 🔧 **Maintainability**: Clear separation of concerns, easier to modify specific functionality
- ⚡ **Development Speed**: Faster incremental compilation, better IDE support
- 📚 **Organization**: Logical grouping of related functionality
- 🚀 **Extensibility**: Easy to add new feature modules without cluttering main lib

## Critical Priority Features ⚠️

### 0. IC Compatibility Audit and Feature Gating

**Status**: In Progress 🔧\
**Complexity**: High\
**Description**: Comprehensive audit and update of device.yaml and high-level API for multi-chip compatibility.

#### Completed ✅

- [x] **Feature-gate ROM library methods**: `Effect` enum, `set_library`, `set_single_effect_enum`, audio-to-vibe methods gated behind `drv2605`/`drv2605l`
- [x] **Compile-time device selection**: `compile_error!` when no device feature selected
- [x] **Device ID validation**: `init()` / `init_async()` validate against feature-specific expected device ID
- [x] **Compilation tests**: All feature flag combinations (`drv2604`, `drv2604l`, `drv2605`, `drv2605l`, `async` combos) compile and pass
- [x] **Cross-compilation**: All feature combinations build correctly
- [x] **API availability**: ROM/audio-to-vibe methods only available with `drv2605`/`drv2605l`

#### Remaining Work

##### Device Register Compatibility

- [ ] **Audit device.yaml**: Review all register definitions against datasheets for DRV2604, DRV2604L, DRV2605, DRV2605L
- [ ] **Identify chip-specific registers**: Map which registers exist on which variants (e.g., Control5/LRA OL Period on L-variants, RAM on DRV2604/DRV2604L)
- [ ] **Add conditional register definitions**: Use `#[cfg(feature = "...")]` attributes in device.yaml if supported, or gate at API level
- [ ] **Validate register address differences**: Ensure no register conflicts between variants

##### Calibration and Timing

- [ ] **Audit calibration parameters**: Verify voltage ranges and defaults for standard vs low-voltage variants
- [ ] **Review timing parameters**: Check if timing constants differ between chip variants
- [ ] **Validate operating mode support**: Confirm all modes are available on all variants

##### Documentation

- [ ] **Update README**: Clarify chip-specific feature requirements
- [ ] **API documentation**: Add chip compatibility notes to all methods
- [ ] **Examples**: Create chip-specific usage examples
- [ ] **Migration guide**: Document differences for users switching between variants

##### Testing

- [ ] **Hardware validation**: Test on actual DRV2604, DRV2604L, DRV2605, DRV2605L hardware

## High-Priority Features 🚀

### 1. Advanced Auto-Calibration API

**Status**: Partially Complete ⚠️\
**Complexity**: Medium\
**Description**: Enhanced auto-calibration support with result validation and configuration helpers.

**Current State**: Basic auto-calibration implemented (`start_auto_calibration()`)

**Remaining Work**:

- Pre-calibration parameter validation
- Automatic rated voltage calculation helpers
- Calibration result interpretation and validation
- Calibration failure diagnosis and retry strategies

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

### 2. Real-Time Playback (RTP) Utilities

**Status**: Partially Complete ⚠️\
**Complexity**: Medium\
**Description**: High-level utilities for real-time haptic control with waveform generation.

**Current State**: Basic RTP input method implemented (`set_rtp_input()`)

**Remaining Work**:

- Waveform generation utilities (sine, triangle, sawtooth, noise)
- Amplitude envelope support (ADSR, fade-in/out)
- Sample rate control and timing utilities
- Streaming interface for continuous RTP data

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

### 3. Effect Metadata and Categorization

**Status**: Not Started\
**Complexity**: Low\
**Description**: Add metadata to effects for better discoverability and usage.

```rust
impl Effect {
    pub fn duration_ms(&self) -> Option<u16> { /* ... */ }
    pub fn category(&self) -> EffectCategory { /* ... */ }
    pub fn description(&self) -> &'static str { /* ... */ }
    pub fn intensity_level(&self) -> u8 { /* ... */ }
}

pub enum EffectCategory {
    Click,
    Buzz,
    Transition,
    Alert,
    Ramp,
    Smooth,
}
```

## Medium-Priority Features 📋

### 4. Advanced Diagnostics and Health Monitoring

**Status**: Partially Complete ⚠️\
**Complexity**: Medium\
**Description**: Enhanced actuator health monitoring beyond basic pass/fail.

**Current State**: Basic diagnostics implemented (`start_diagnostics()`, status monitoring)

**Remaining Work**:

- Extended diagnostic routines with detailed results
- Actuator impedance measurement and trending
- Temperature monitoring and thermal management
- Diagnostic result interpretation with recommended actions

### 5. Complex Waveform Composition

**Status**: Foundation Complete ✅\
**Complexity**: Medium\
**Description**: Tools for creating complex haptic patterns by combining effects.

**Current Foundation**: Complete waveform sequencing with up to 8 effects and timing

**Remaining Work**:

- Waveform sequencer with conditional logic
- Effect layering and blending capabilities
- Pattern templates and macro support
- Pattern validation and optimization

### 6. Power Management Optimization

**Status**: Basic Complete ✅\
**Complexity**: Medium\
**Description**: Advanced power management features for battery-powered applications.

**Current State**: Basic power management (standby modes, reset) implemented

**Remaining Work**:

- Intelligent standby mode management
- Power consumption profiling tools
- Battery voltage monitoring integration
- Low-power effect alternatives

## Low-Priority Features 🔮

### 7. Haptic Pattern Editor/Builder

**Status**: Not Started\
**Complexity**: High\
**Description**: Build-time tools for creating and validating complex haptic patterns.

### 8. Platform Integration Helpers

**Status**: Not Started\
**Complexity**: Low\
**Description**: Platform-specific integration helpers for common embedded systems.

### 9. Haptic Accessibility Features

**Status**: Not Started\
**Complexity**: Medium\
**Description**: Accessibility-focused haptic patterns for UI/UX applications.

## Testing Strategy 🧪

- [x] **Unit tests**: Core API functionality with mock I2C
- [x] **Compilation tests**: All feature combinations compile correctly
- [x] **Example applications**: Demonstration programs for effects and functionality
- [ ] **Integration tests**: Real hardware validation on development boards
- [ ] **Benchmarking**: Performance and power consumption characterization
- [x] **Documentation**: Comprehensive API documentation with usage examples

## Contributing 🤝

This roadmap is a living document. The recent major milestone of implementing the effects library, timing controls, and audio-to-vibe configuration represents significant progress toward a complete haptic driver solution.

**Priority areas for contribution**:

1. **🔥 IC Compatibility Audit (remaining)**: device.yaml register audit, voltage/timing parameter validation
1. **Advanced Calibration**: Enhanced calibration workflows and validation
1. **RTP Utilities**: Waveform generation and streaming capabilities
1. **Effect Metadata**: Adding categorization and descriptions to effects
1. **Hardware Testing**: Validation on real DRV260X hardware (especially DRV2604/DRV2604L)
1. **Examples**: Chip-specific usage examples and tutorials

Each feature should include:

- Comprehensive documentation
- Usage examples
- Unit tests where applicable
- Integration with existing async/sync APIs
- Backward compatibility considerations

______________________________________________________________________

**Recent Major Milestones** 🎯:

- ✅ Complete modular architecture refactor (Dec 2024)
- ✅ Full effects library implementation (123 effects)
- ✅ Waveform timing control system
- ✅ Audio-to-vibe configuration API
- ✅ Comprehensive async/sync API parity (70+ methods)
- ✅ Compile-time feature gating for all four device variants (Feb 2025)
- ✅ Device ID validation per feature-selected variant (Feb 2025)

The driver now provides a complete, production-ready foundation for haptic applications with both basic and advanced use cases covered.
