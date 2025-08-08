//! Demonstration of the new Effect enum and enhanced functionality
//!
//! This example shows how to use the predefined Effect enum, timing offset methods,
//! and the convenient initialization method for ERM actuators.

use drv260x::{AthFilter, AthPeakTime, Drv260x, Effect, WaveformEntry};

fn main() {
    println!("DRV260X Effects Demo");
    println!("====================");

    // Note: In a real application, you would initialize your I2C peripheral here
    // let i2c = /* your I2C implementation */;
    // let mut haptic = Drv260x::new(i2c);

    // Example usage patterns:

    println!("\n1. Predefined Effects:");
    demo_predefined_effects();

    println!("\n2. Waveform Sequences:");
    demo_waveform_sequences();

    println!("\n3. Effect Categories:");
    demo_effect_categories();

    println!("\n4. API Methods Available:");
    demo_api_methods();
}

fn demo_predefined_effects() {
    println!("   - Effect::StrongClick100");
    println!("   - Effect::SharpClick60");
    println!("   - Effect::SoftBump30");
    println!("   - Effect::DoubleClick100");
    println!("   - Effect::Buzz1_100");
    println!("   - Effect::PulsingStrong1_100");

    println!("\n   Usage example:");
    println!("   haptic.set_single_effect_enum(Effect::StrongClick100).unwrap();");
    println!("   haptic.go().unwrap();");
}

fn demo_waveform_sequences() {
    println!("   Create complex sequences with multiple effects and timing:");
    println!(
        "   
   let sequence = [
       WaveformEntry::from(Effect::SharpClick100),
       WaveformEntry::wait(5),  // 50ms wait
       WaveformEntry::from(Effect::SoftBump60),
       WaveformEntry::wait(3),  // 30ms wait  
       WaveformEntry::from(Effect::Buzz1_100),
       WaveformEntry::stop(),
   ];
   haptic.set_waveform_sequence(&sequence).unwrap();"
    );
}

fn demo_effect_categories() {
    println!("   Click Effects: StrongClick100, SharpClick100, MediumClick1_100, DoubleClick100");
    println!("   Buzz Effects: StrongBuzz100, Buzz1_100, PulsingStrong1_100, LongBuzzForProgrammaticStopping100");
    println!("   Transition Effects: TransitionClick1_100, TransitionRampUpLongSmooth1_0to100, SmoothHum1_50");
    println!("   Alert Effects: Alert750ms, Alert1000ms");
}

fn demo_api_methods() {
    println!("   Sync Methods:");
    println!("   - init_open_loop_erm()            // Convenience ERM initialization");
    println!("   - set_single_effect_enum(Effect)  // Play predefined effects");
    println!("   - set_overdrive_time_offset(i8)   // Fine-tune waveform timing");
    println!("   - set_sustain_time_offset_*(i8)   // Adjust sustain timing");
    println!("   - set_brake_time_offset(i8)       // Control brake timing");
    println!("   - set_audio_to_vibe_control()     // Audio-to-haptic conversion");

    println!("\n   Async Methods (with async feature enabled):");
    println!("   - init_open_loop_erm_async()");
    println!("   - set_single_effect_enum_async()");
    println!("   - set_*_async() versions of all methods");

    println!("\n   Example initialization:");
    println!("   haptic.init_open_loop_erm().unwrap();  // One-line ERM setup");

    println!("\n   Example waveform timing control:");
    println!("   haptic.set_overdrive_time_offset(5).unwrap();");
    println!("   haptic.set_sustain_time_offset_positive(10).unwrap();");

    println!("\n   Example audio-to-vibe configuration:");
    println!("   haptic.set_audio_to_vibe_control(AthFilter::Hz150, AthPeakTime::Ms20).unwrap();");
}

#[cfg(feature = "async")]
async fn async_example() {
    println!("\nAsync Usage Example:");
    println!("====================");

    // In real code:
    // let i2c = /* your async I2C implementation */;
    // let mut haptic = Drv260x::new(i2c);

    println!("   // Initialize asynchronously");
    println!("   haptic.init_open_loop_erm_async().await.unwrap();");

    println!("   // Play effects asynchronously");
    println!("   haptic.set_single_effect_enum_async(Effect::StrongClick100).await.unwrap();");
    println!("   haptic.go_async().await.unwrap();");

    println!("   // Configure timing asynchronously");
    println!("   haptic.set_overdrive_time_offset_async(5).await.unwrap();");
}
