//! Demonstration of the new Effect enum and enhanced functionality
//!
//! This example shows how to use the predefined Effect enum, timing offset methods,
//! and the convenient initialization method for ERM actuators.

#![no_std]
#![no_main]

use drv260x::{AthFilter, AthPeakTime, Drv260x, Effect, OperatingMode, WaveformEntry};
use embedded_hal_mock::eh1::i2c::{Mock, Transaction};
use panic_halt as _;

// Mock I2C implementation for demonstration
type MockI2c = Mock;

fn main() {
    // Set up mock I2C transactions (in a real application, use your HAL's I2C)
    let expectations = [
        Transaction::write_read(0x5A, vec![0x00], vec![0xE7]), // Read status (device ID = 7)
        Transaction::write(0x5A, vec![0x01, 0x00]),            // Clear standby
        Transaction::write(0x5A, vec![0x01, 0x00]),            // Set internal mode
        Transaction::write(0x5A, vec![0x1A, 0x80]),            // Set ERM mode
        Transaction::write(0x5A, vec![0x1D, 0x20]),            // Set ERM open loop
        Transaction::write(0x5A, vec![0x04, 0x01, 0x00]),      // Set waveform sequence
    ];

    let i2c = MockI2c::new(&expectations);
    let mut haptic = Drv260x::new(i2c);

    // Initialize for ERM in open-loop mode (convenience method)
    haptic.init_open_loop_erm().unwrap();

    // Play predefined effects using the Effect enum
    haptic
        .set_single_effect_enum(Effect::StrongClick100)
        .unwrap();
    haptic.go().unwrap();

    // Create a complex sequence with different effects
    let sequence = [
        WaveformEntry::from(Effect::SharpClick100),
        WaveformEntry::wait(5), // 50ms wait
        WaveformEntry::from(Effect::SoftBump60),
        WaveformEntry::wait(3), // 30ms wait
        WaveformEntry::from(Effect::Buzz1_100),
        WaveformEntry::stop(),
    ];
    haptic.set_waveform_sequence(&sequence).unwrap();
    haptic.go().unwrap();

    // Configure waveform timing offsets for fine-tuning
    haptic.set_overdrive_time_offset(5).unwrap(); // Extend overdrive
    haptic.set_sustain_time_offset_positive(10).unwrap(); // Extend positive sustain
    haptic.set_brake_time_offset(-2).unwrap(); // Reduce brake time

    // Configure audio-to-vibe settings
    haptic
        .set_audio_to_vibe_control(AthFilter::Hz150, AthPeakTime::Ms20)
        .unwrap();
    haptic.set_audio_to_vibe_min_input_level(0x19).unwrap();
    haptic.set_audio_to_vibe_max_input_level(0xFF).unwrap();

    // Demonstrate different effect categories
    demo_click_effects(&mut haptic);
    demo_buzz_effects(&mut haptic);
    demo_transition_effects(&mut haptic);

    loop {
        // Main application loop
    }
}

fn demo_click_effects(haptic: &mut Drv260x<MockI2c>) {
    // Different click effects with varying intensities
    let click_effects = [
        Effect::StrongClick100,
        Effect::StrongClick60,
        Effect::SharpClick100,
        Effect::MediumClick1_100,
        Effect::DoubleClick100,
        Effect::TripleClick100,
    ];

    for effect in &click_effects {
        haptic.set_single_effect_enum(*effect).unwrap();
        haptic.go().unwrap();

        // Wait for completion (in real code, you'd check is_active())
        // busy_wait_ms(100);
    }
}

fn demo_buzz_effects(haptic: &mut Drv260x<MockI2c>) {
    // Various buzz and vibration effects
    let buzz_effects = [
        Effect::StrongBuzz100,
        Effect::Buzz1_100,
        Effect::Buzz3_60,
        Effect::PulsingStrong1_100,
        Effect::PulsingMedium2_60,
        Effect::LongBuzzForProgrammaticStopping100,
    ];

    for effect in &buzz_effects {
        haptic.set_single_effect_enum(*effect).unwrap();
        haptic.go().unwrap();
        // busy_wait_ms(200);
    }
}

fn demo_transition_effects(haptic: &mut Drv260x<MockI2c>) {
    // Smooth transition effects for UI feedback
    let transition_effects = [
        Effect::TransitionClick1_100,
        Effect::TransitionRampUpLongSmooth1_0to100,
        Effect::TransitionRampDownMediumSmooth1_100to0,
        Effect::SmoothHum1_50,
        Effect::SmoothHum3_30,
    ];

    for effect in &transition_effects {
        haptic.set_single_effect_enum(*effect).unwrap();
        haptic.go().unwrap();
        // busy_wait_ms(150);
    }
}

#[cfg(feature = "async")]
async fn async_example() {
    use embedded_hal_async_mock::i2c::Mock as AsyncMock;

    // Async version using the new async methods
    let expectations = []; // Add your async I2C expectations
    let i2c = AsyncMock::new(&expectations);
    let mut haptic = Drv260x::new(i2c);

    // Initialize using async method
    haptic.init_open_loop_erm_async().await.unwrap();

    // Play effects asynchronously
    haptic
        .set_single_effect_enum_async(Effect::StrongClick100)
        .await
        .unwrap();
    haptic.go_async().await.unwrap();

    // Configure timing offsets asynchronously
    haptic.set_overdrive_time_offset_async(5).await.unwrap();
    haptic
        .set_sustain_time_offset_positive_async(10)
        .await
        .unwrap();

    // Configure audio-to-vibe asynchronously
    haptic
        .set_audio_to_vibe_control_async(AthFilter::Hz150, AthPeakTime::Ms20)
        .await
        .unwrap();
}
