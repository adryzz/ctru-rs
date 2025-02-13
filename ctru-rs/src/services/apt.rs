//! Applet service.
//!
//! The APT service handles integration with other applications,
//! including high-level OS features such as Sleep mode, the Home Menu and application switching.
//!
//! It also handles running applets, small programs made available by the OS to streamline specific functionality.
//! Those are implemented in the [`applets`](crate::applets) module.

use crate::error::ResultCode;

/// Handle to the Applet service.
pub struct Apt(());

impl Apt {
    /// Initialize a new service handle.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use ctru::services::apt::Apt;
    ///
    /// let apt = Apt::new()?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "aptInit")]
    pub fn new() -> crate::Result<Apt> {
        unsafe {
            ResultCode(ctru_sys::aptInit())?;
            Ok(Apt(()))
        }
    }

    /// Returns `true` if the application is running in the foreground as normal.
    ///
    /// # Notes
    ///
    /// This function is called as such since it automatically handles all checks for Home Menu switching, Sleep mode and other events that could take away control from the application.
    /// For this reason, its main use is as the condition of a while loop that controls the main logic for your program.
    ///
    /// # Example
    ///
    /// ```
    /// # let _runner = test_runner::GdbRunner::default();
    /// use std::error::Error;
    /// use ctru::services::apt::Apt;
    ///
    /// // In a simple `main` function, the structure should be the following.
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///
    /// let apt = Apt::new()?;
    ///
    /// while apt.main_loop() {
    ///     // Main program logic should be written here.
    /// }
    ///
    /// // Optional clean-ups after running the application should be written after the main loop.
    /// #
    /// # Ok(())
    /// # }
    /// ```
    #[doc(alias = "aptMainLoop")]
    pub fn main_loop(&self) -> bool {
        unsafe { ctru_sys::aptMainLoop() }
    }

    /// Set (in percentage) the amount of time to lend to the application thread spawned on the syscore (core #1).
    ///
    /// # Notes
    ///
    /// It is necessary to set a time limit before spawning threads on the syscore.
    /// The percentage value must be withing 5% and 89%, though it is suggested to use lower values (around 30-45%) to avoid slowing down the OS processes.
    #[doc(alias = "APT_SetAppCpuTimeLimit")]
    pub fn set_app_cpu_time_limit(&mut self, percent: u32) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::APT_SetAppCpuTimeLimit(percent))?;
            Ok(())
        }
    }
}

impl Drop for Apt {
    #[doc(alias = "aptExit")]
    fn drop(&mut self) {
        unsafe { ctru_sys::aptExit() };
    }
}
