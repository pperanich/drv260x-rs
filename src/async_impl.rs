//! Asynchronous implementation of DRV260X driver methods
//!
//! This module contains all the asynchronous methods for the DRV260X haptic driver.
//! All methods follow the same patterns as the synchronous versions but use async/await.

use crate::ll::{FbBrakeFactor, LoopGain, OperatingMode};
#[cfg(any(feature = "drv2605", feature = "drv2605l"))]
use crate::ll::{AthFilter, AthPeakTime, LibrarySelection};
use crate::{Drv260x, Error, StatusInfo, WaveformEntry};
#[cfg(any(feature = "drv2605", feature = "drv2605l"))]
use crate::Effect;
use embedded_hal_async::i2c::I2c as AsyncI2c;

cfg_if::cfg_if! {
    if #[cfg(feature = "drv2604")] {
        const EXPECTED_DEVICE_ID: u8 = 4;
    } else if #[cfg(feature = "drv2604l")] {
        const EXPECTED_DEVICE_ID: u8 = 6;
    } else if #[cfg(feature = "drv2605")] {
        const EXPECTED_DEVICE_ID: u8 = 3;
    } else if #[cfg(feature = "drv2605l")] {
        const EXPECTED_DEVICE_ID: u8 = 7;
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

        if device_id != EXPECTED_DEVICE_ID {
            return Err(Error::InvalidDeviceId {
                expected: EXPECTED_DEVICE_ID,
                found: device_id,
            });
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

    /// Get comprehensive device status information (async version)
    pub async fn get_status_async(&mut self) -> Result<StatusInfo, Error<E>> {
        let status = self.device.status().read_async().await?;
        Ok(StatusInfo {
            overcurrent_detected: status.oc_detect(),
            overtemperature_detected: status.over_temp(),
            feedback_status: status.fb_sts(),
            diagnostic_result: status.diag_result(),
            illegal_address: status.illegal_addr(),
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

        self.device
            .waveform_sequencer(index as usize)
            .write_async(|reg| {
                reg.set_wav_frm_seq(entry.value);
                reg.set_wait(entry.is_wait);
            })
            .await?;

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
}

/// Async methods only available on DRV2605 and DRV2605L variants (ROM library and audio-to-vibe).
#[cfg(all(feature = "async", any(feature = "drv2605", feature = "drv2605l")))]
impl<I2C, E> Drv260x<I2C>
where
    I2C: AsyncI2c<Error = E>,
{
    /// Set library selection (async version)
    pub async fn set_library_async(&mut self, library: LibrarySelection) -> Result<(), Error<E>> {
        self.device
            .library_selection()
            .modify_async(|reg| reg.set_library_sel(library))
            .await?;
        Ok(())
    }

    /// Set a single predefined effect in the first sequencer slot (async version)
    pub async fn set_single_effect_enum_async(&mut self, effect: Effect) -> Result<(), Error<E>> {
        let sequence = [WaveformEntry::from(effect), WaveformEntry::stop()];
        self.set_waveform_sequence_async(&sequence).await
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
