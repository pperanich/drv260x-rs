//! Predefined haptic effects and waveform utilities
//!
//! This module contains the comprehensive Effect enum with all predefined
//! haptic effects from the DRV260X ROM library, as well as utilities for
//! working with waveform sequences.

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
