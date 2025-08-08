# DRV260X Haptic Driver Family

[![Crates.io](https://img.shields.io/crates/v/drv260x.svg)](https://crates.io/crates/drv260x)
[![Documentation](https://docs.rs/drv260x/badge.svg)](https://docs.rs/drv260x)
[![License: Apache 2.0 OR MIT](https://img.shields.io/badge/license-Apache%202.0%20OR%20MIT-blue.svg)](LICENSE-APACHE)

A platform-agnostic Rust driver for the Texas Instruments DRV260X haptic driver family, built using the [`embedded-hal`] traits for I2C communication and powered by the [`device-driver`] framework for robust register access.

## Supported Devices

| Device | Description | Feature Flag | ROM Library | Status |
|-----------|--------------------------------------------------|--------------|-------------|--------|
| DRV2605 | Haptic driver with licensed ROM library | `drv2605` | ✅ Yes | ✅ |
| DRV2605L | Low-voltage version of DRV2605 | `drv2605l` | ✅ Yes | ✅ |
| DRV2604 | Haptic driver with RAM (no ROM library) | `drv2604` | ❌ No | ✅ |
| DRV2604L | Low-voltage version of DRV2604 | `drv2604l` | ❌ No | ✅ |

**Important**: You must specify the correct feature flag for your chip variant, as this determines:

- Available registers (some registers only exist on specific variants)
- ROM library availability (Effect enum only available on DRV2605/DRV2605L)
- Voltage-specific default configurations

## Features

- **🚀 High-level API** - Easy-to-use driver with comprehensive error handling
- **⚡ Async/sync support** - Full async/await support with feature gating
- **🎯 Multi-chip support** - Auto-detection for all DRV260X variants
- **📡 Multiple operating modes** - Internal/external trigger, PWM, audio-to-vibe, real-time playback
- **🔧 Auto-calibration** - Support for both ERM and LRA actuator calibration
- **🩺 Diagnostics** - Built-in actuator health monitoring
- **🎼 Waveform sequencing** - Complex haptic pattern creation with up to 8 effects
- **🎵 Predefined effects library** - 123 built-in haptic effects (DRV2605/DRV2605L only)
- **⏰ Waveform timing control** - Fine-tuning of library effect timing
- **🎧 Audio-to-vibe configuration** - Advanced audio-to-haptic conversion
- **📊 Real-time control** - Direct amplitude control for custom haptic effects
- **🔋 Power management** - Standby modes and power optimization
- **🦀 `#![no_std]` compatible** - Suitable for embedded environments

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
# For DRV2605 with ROM library
drv260x = { version = "0.1", features = ["drv2605"] }

# For DRV2605L with ROM library 
drv260x = { version = "0.1", features = ["drv2605l"] }

# For DRV2604 (RAM-only, no ROM library)
drv260x = { version = "0.1", features = ["drv2604"] }

# For DRV2604L (RAM-only, no ROM library)
drv260x = { version = "0.1", features = ["drv2604l"] }

# Combine with async support
# drv260x = { version = "0.1", features = ["drv2605", "async"] }
```

### Basic Usage

```rust
use drv260x::{Drv260x, OperatingMode, Effect};
use embedded_hal::i2c::I2c;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize your I2C peripheral
    let i2c = /* your I2C implementation */;
    
    // Create driver instance
    let mut haptic = Drv260x::new(i2c);
    
    // Initialize the device
    haptic.init()?;
    
    // Set operating mode
    haptic.set_mode(OperatingMode::Internal)?;
    
    // Play a predefined haptic effect
    haptic.set_single_effect_enum(Effect::StrongClick100)?;
    haptic.go()?;
    
    // Wait for completion
    while haptic.is_active()? {
        // Wait or do other work
    }
    
    Ok(())
}
```

### Using Predefined Effects (DRV2605/DRV2605L only)

```rust
// Only available with "drv2605" or "drv2605l" feature flags
#[cfg(any(feature = "drv2605", feature = "drv2605l"))]
{
    use drv260x::{Effect, WaveformEntry};
    
    // Play individual predefined effects
    haptic.set_single_effect_enum(Effect::StrongClick100)?;
    haptic.go()?;
    
    // Create complex sequences with predefined effects
    let sequence = [
        WaveformEntry::from(Effect::SharpClick100),
        WaveformEntry::wait(5),             // Wait 50ms (5 * 10ms)
        WaveformEntry::from(Effect::Buzz1_100),
        WaveformEntry::wait(2),             // Wait 20ms
        WaveformEntry::from(Effect::PulsingStrong1_100),
        WaveformEntry::stop(),              // End sequence
    ];
    
    haptic.set_waveform_sequence(&sequence)?;
    haptic.go()?;
}
```

### Custom Waveforms (All Devices)

```rust
// Available on all devices - use custom amplitude values for DRV2604/DRV2604L
use drv260x::WaveformEntry;

// For DRV2604/DRV2604L: use custom amplitude values (1-127)
// For DRV2605/DRV2605L: can use both ROM effects (1-123) and custom amplitudes
let sequence = [
    WaveformEntry::effect(100),         // Custom amplitude or ROM effect
    WaveformEntry::wait(5),             // Wait 50ms
    WaveformEntry::effect(80),          // Different amplitude
    WaveformEntry::stop(),              // End sequence
];

haptic.set_waveform_sequence(&sequence)?;
haptic.go()?;
```

### Convenient ERM Initialization

```rust
// One-line setup for ERM actuators in open-loop mode
haptic.init_open_loop_erm()?;

// Now ready to play effects
haptic.set_single_effect_enum(Effect::StrongClick100)?;
haptic.go()?;
```

### Async Usage

Enable the `async` feature and use the `_async` methods:

```rust
use drv260x::{Drv260x, OperatingMode};

#[cfg(any(feature = "drv2605", feature = "drv2605l"))]
use drv260x::Effect;

async fn haptic_demo(i2c: impl embedded_hal_async::i2c::I2c) -> Result<(), drv260x::Error<_>> {
    let mut haptic = Drv260x::new(i2c);
    
    // Convenient async ERM initialization
    haptic.init_open_loop_erm_async().await?;
    
    // Play predefined effects (ROM devices only)
    #[cfg(any(feature = "drv2605", feature = "drv2605l"))]
    {
        haptic.set_single_effect_enum_async(Effect::StrongClick100).await?;
    }
    
    // Custom amplitude (all devices)
    #[cfg(any(feature = "drv2604", feature = "drv2604l"))]
    {
        haptic.set_single_effect_async(100).await?; // Custom amplitude
    }
    
    haptic.go_async().await?;
    
    Ok(())
}
```

### Auto-Calibration

```rust
use drv260x::{OperatingMode, LoopGain, FbBrakeFactor};

// Configure for LRA actuator
haptic.set_actuator_type(true)?; // true = LRA, false = ERM

// Set calibration parameters
haptic.set_rated_voltage(0x3E)?;
haptic.set_overdrive_clamp_voltage(0x8C)?;
haptic.set_feedback_control(
    LoopGain::Medium,
    FbBrakeFactor::X2,
    1 // BEMF gain
)?;

// Start auto-calibration
haptic.start_auto_calibration()?;

// Wait for calibration to complete
while haptic.is_active()? {
    // Wait for GO bit to clear
}

// Check calibration result
let status = haptic.get_status()?;
if status.diagnostic_result {
    println!("Calibration failed!");
} else {
    println!("Calibration successful!");
}
```

## Architecture & Implementation Details

This crate is built using the [`device-driver`] framework, which provides robust, type-safe register access code generation. Understanding the architecture helps when extending functionality or debugging issues.

### Project Structure

```
drv260x/
├── device.yaml          # Register definitions and device specification
├── build.rs             # Build script for code generation
├── src/
│   ├── lib.rs           # Main driver structure and exports
│   ├── sync_impl.rs     # Synchronous method implementations
│   ├── async_impl.rs    # Asynchronous method implementations
│   ├── effects.rs       # Effect enum and waveform utilities
│   ├── ll.rs            # Low-level device interface
├── examples/
│   └── effects_demo.rs  # Effect library demonstration
└── ROADMAP.md           # Future feature development roadmap
```

### High-Level vs Low-Level APIs

#### High-Level Driver API

The high-level API is split across multiple modules for maintainability:

**`lib.rs`** - Core driver structure, error types, and module exports
**`sync_impl.rs`** - All synchronous method implementations
**`async_impl.rs`** - All asynchronous method implementations (when `async` feature is enabled)
**`effects.rs`** - Predefined effects enum and waveform utilities (DRV2605/DRV2605L only)

The API provides:

- **User-friendly methods** with descriptive names and comprehensive documentation
- **Predefined effects library** with 123 built-in haptic effects
- **Timing control methods** for fine-tuning waveform characteristics
- **Audio-to-vibe configuration** for audio-to-haptic conversion
- **Error handling** with meaningful error types and context
- **Device state tracking** to cache configuration and optimize I2C transactions
- **Input validation** to prevent invalid configurations
- **Async/sync variants** of all operations using the `_async` suffix pattern
- **Convenience methods** for common operations and workflows

Example high-level operations:

```rust
haptic.init_open_loop_erm()?;                // Convenient ERM initialization

// Predefined effects (DRV2605/DRV2605L only)
#[cfg(any(feature = "drv2605", feature = "drv2605l"))]
haptic.set_single_effect_enum(Effect::StrongClick100)?;

// Custom amplitudes (all devices)
haptic.set_single_effect(100)?;             // Custom amplitude value
haptic.set_overdrive_time_offset(5)?;       // Fine-tune timing
haptic.start_auto_calibration()?;           // Automated calibration workflow
let status = haptic.get_status()?;          // Parsed status information
```

#### `ll.rs` - Low-Level Device Interface

The low-level API provides:

- **Direct register access** via generated code from `device.yaml`
- **I2C interface abstraction** supporting both sync and async operations
- **Raw register manipulation** for advanced use cases
- **Type-safe field access** with automatic bit manipulation
- **Generated enums** for register field values

Example low-level operations:

```rust
// Direct register access
let status_reg = haptic.device().status().read()?;
let device_id = status_reg.device_id();

// Raw register manipulation
haptic.device().mode().modify(|reg| {
    reg.set_mode(OperatingMode::Internal);
    reg.set_standby(false);
})?;
```

**When to use each:**

- **High-level API**: For most applications, provides safety and convenience
- **Low-level API**: For advanced features, debugging, or when you need maximum control

### `device.yaml` - Device Specification

The `device.yaml` file is the single source of truth for the DRV260X register map. It defines:

#### Configuration

```yaml
config:
  register_address_type: u8    # I2C register addresses are 8-bit
  default_byte_order: LE       # Little-endian byte order
```

#### Register Definitions

Each register is defined with:

```yaml
RegisterName:
  type: register
  address: 0x00                # I2C register address
  size_bits: 8                 # Register size in bits
  reset_value: 0x00            # Power-on reset value
  access: RW                   # Access type (RW/RO/WO)
  description: "Register description"
  fields:
    field_name:
      start: 0                 # Starting bit position
      end: 3                   # Ending bit position (for multi-bit fields)
      base: uint               # Field type (uint/bool)
      description: "Field description"
      conversion:              # Optional enum conversion
        name: EnumName
        Value1:
          value: 0
          description: "Enum value description"
```

#### Code Generation Process

1. **Build time**: The [`device-driver-macros`] crate reads `device.yaml`
1. **Validation**: Register layouts are validated for overlaps and consistency
1. **Code generation**: Type-safe register access code is generated
1. **Compilation**: Generated code is compiled with the rest of the crate

The generated code provides:

- **Register structs** with type-safe field accessors
- **Enum types** for register field values with validation
- **Read/write/modify methods** for each register
- **Async variants** when the `async` feature is enabled

### I2C Interface Implementation

The `DeviceInterface` struct in `ll.rs` implements the required traits:

```rust
impl<I2cTrait: I2c> device_driver::RegisterInterface for DeviceInterface<I2cTrait> {
    type AddressType = u8;
    type Error = DeviceInterfaceError<I2cTrait::Error>;

    fn read_register(&mut self, address: u8, _size_bits: u32, data: &mut [u8]) -> Result<(), Self::Error> {
        // I2C write-read transaction: write register address, read data
        self.i2c.write_read(I2C_ADDRESS, &[address], data)
            .map_err(DeviceInterfaceError::I2c)
    }

    fn write_register(&mut self, address: u8, _size_bits: u32, data: &[u8]) -> Result<(), Self::Error> {
        // I2C write transaction: write register address followed by data
        let mut buf = [0u8; 9]; // Max for multi-byte writes
        buf[0] = address;
        buf[1..1 + data.len()].copy_from_slice(data);
        self.i2c.write(I2C_ADDRESS, &buf[..1 + data.len()])
            .map_err(DeviceInterfaceError::I2c)
    }
}
```

**Key implementation details:**

- **I2C address**: Fixed at `0x5A` for all DRV260X variants
- **Error propagation**: I2C errors are wrapped in `DeviceInterfaceError`
- **Multi-byte support**: Supports up to 8-byte register writes (for waveform sequences)
- **Async support**: Parallel async implementation when `async` feature is enabled

### Feature Flags and Conditional Compilation

#### Async Support

```toml
drv260x = { version = "0.1", features = ["async"] }
```

When enabled:

- Adds `embedded-hal-async` dependency
- Generates async variants of all register operations
- Provides `_async` suffix methods in the high-level API

#### Chip Variant Features

```toml
drv260x = { version = "0.1", features = ["drv2605"] }
```

**Required Feature Flags:**

- `drv2605` - Standard voltage DRV2605 with ROM library
- `drv2605l` - Low voltage DRV2605L with ROM library
- `drv2604` - Standard voltage DRV2604 with RAM only
- `drv2604l` - Low voltage DRV2604L with RAM only

**Feature Flag Effects:**

- **ROM Library Access**: Effect enum and `set_single_effect_enum()` methods are only available with `drv2605`/`drv2605l` features
- **Register Availability**: Some registers and configuration options are chip-specific
- **Default Configurations**: Voltage-specific defaults for calibration and operation
- **Compile-time Safety**: Prevents using ROM library methods on RAM-only devices

**Device ID Validation**: The driver validates the detected device ID matches your selected feature flag during initialization.

#### Debug and Logging

```toml
drv260x = { version = "0.1", features = ["defmt-03"] }
```

Enables [`defmt`] support for:

- Error types with `#[derive(defmt::Format)]`
- Status structures for debugging
- Enhanced debugging in `no_std` environments

### Error Handling Strategy

The crate provides comprehensive error types:

```rust
pub enum Error<E> {
    I2c(E),                              // I2C communication errors
    InvalidDeviceId { expected: u8, found: u8 },  // Device ID validation
    NotReady,                            // Device not ready for operation
    InvalidConfig(&'static str),         // Configuration validation errors
    Timeout,                             // Operation timeouts
    InvalidWaveform,                     // Waveform sequence errors
}
```

**Error handling patterns:**

- **I2C errors** are preserved and propagated from the HAL layer
- **Validation errors** provide context about what went wrong
- **Device errors** indicate hardware or configuration issues
- **All errors** implement standard Rust error traits when `std` is available

### Performance Considerations

#### I2C Transaction Optimization

- **Cached state**: Driver caches device configuration to minimize I2C reads
- **Multi-byte writes**: Waveform sequences use single I2C transactions
- **Lazy initialization**: Expensive operations are deferred until needed

#### Memory Usage

- **Zero-allocation**: All operations work without heap allocation
- **Stack-based**: Register data uses stack-allocated buffers
- **Minimal footprint**: Core driver struct is lightweight

#### Async Performance

- **Non-blocking**: Async methods don't block the async runtime
- **Cancellation safe**: Operations can be cancelled without corrupting device state
- **Concurrent safe**: Multiple async operations can be pending (though not recommended)

## Examples

See the `examples/` directory for complete examples:

- `effects_demo.rs` - Comprehensive demonstration of the Effect enum, waveform sequencing, timing control, and audio-to-vibe configuration

For more usage examples, see the documentation and the individual method examples in the API documentation.

## Development

### Building

```bash
# Check code
cargo check

# Run tests
cargo test

# Build documentation
cargo doc --open

# Check all feature combinations
cargo hack check --feature-powerset
```

### Contributing

This crate follows the roadmap outlined in [`ROADMAP.md`](ROADMAP.md). Recently completed features include:

1. **✅ Haptic Effects Library** - Complete 123-effect enum from TI ROM library (DRV2605/DRV2605L)
1. **✅ Waveform Timing Control** - Fine-tuning methods for library effects
1. **✅ Audio-to-Vibe Configuration** - Audio-to-haptic conversion methods
1. **✅ Modular Architecture** - Clean separation of sync/async implementations

See ROADMAP.md for upcoming priority features.

### Testing

The crate includes comprehensive tests:

- **Unit tests** - Core functionality with mocked I2C
- **Integration tests** - Real hardware validation
- **Documentation tests** - Ensure examples compile and work
- **Feature tests** - Validate all feature flag combinations

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

______________________________________________________________________

[`defmt`]: https://crates.io/crates/defmt
[`device-driver-macros`]: https://crates.io/crates/device-driver-macros
[`device-driver`]: https://crates.io/crates/device-driver
[`embedded-hal`]: https://crates.io/crates/embedded-hal
