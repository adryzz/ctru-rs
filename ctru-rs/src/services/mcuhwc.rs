use crate::error::ResultCode;

/// Handle to the MCU.
pub struct McuHwc(());

impl McuHwc {
    /// Initialize a new service handle.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::mcuhwc::McuHwc;
    ///
    /// let mcu = McuHwc::new()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "mcuHwcInit")]
    pub fn new() -> crate::Result<McuHwc> {
        unsafe {
            ResultCode(ctru_sys::mcuHwcInit())?;
            Ok(McuHwc(()))
        }
    }

    pub fn write_led_pattern(&self, pattern: &[u8; 100]) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::MCUHWC_WriteRegister(0x2D, std::mem::transmute(pattern), pattern.len() as u32))?;
            Ok(())
        }
    }

    #[doc(alias = "MCUHWC_GetBatteryVoltage")]
    pub fn get_battery_voltage(&self) -> crate::Result<u8> {
        unsafe {
            let mut voltage = 0u8;
            ResultCode(ctru_sys::MCUHWC_GetBatteryVoltage(&mut voltage))?;
            Ok(voltage)
        }
    }

    #[doc(alias = "MCUHWC_GetBatteryLevel")]
    pub fn get_battery_level(&self) -> crate::Result<u8> {
        unsafe {
            let mut level = 0u8;
            ResultCode(ctru_sys::MCUHWC_GetBatteryLevel(&mut level))?;
            Ok(level)
        }
    }

    #[doc(alias = "MCUHWC_GetSoundSliderLevel")]
    pub fn get_sound_slider_level(&self) -> crate::Result<u8> {
        unsafe {
            let mut level = 0u8;
            ResultCode(ctru_sys::MCUHWC_GetSoundSliderLevel(&mut level))?;
            Ok(level)
        }
    }

    #[doc(alias = "MCUHWC_Get3dSliderLevel")]
    pub fn get_3d_slider_level(&self) -> crate::Result<u8> {
        unsafe {
            let mut level = 0u8;
            ResultCode(ctru_sys::MCUHWC_Get3dSliderLevel(&mut level))?;
            Ok(level)
        }
    }
}


impl Drop for McuHwc {
    #[doc(alias = "mcuHwcExit")]
    fn drop(&mut self) {
        unsafe { ctru_sys::mcuHwcExit() };
    }
}