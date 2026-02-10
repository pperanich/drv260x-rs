//! Synchronous implementation of DRV260X driver methods
//!
//! This module contains all the synchronous methods for the DRV260X haptic driver.
//! Methods are organized by functionality for better maintainability.

use crate::ll::{AthFilter, AthPeakTime, FbBrakeFactor, LibrarySelection, LoopGain, OperatingMode};
use crate::{Drv260x, Effect, Error, StatusInfo, WaveformEntry};
use embedded_hal::i2c::I2c;

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

        self.device
            .waveform_sequencer(index as usize)
            .write(|reg| {
                reg.set_wav_frm_seq(entry.value);
                reg.set_wait(entry.is_wait);
            })?;

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
