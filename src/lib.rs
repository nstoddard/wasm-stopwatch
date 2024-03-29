//! A simple stopwatch for games and similar applications.

pub mod fps_logger;

#[cfg(target_arch = "wasm32")]
use web_sys::*;

#[derive(Clone)]
pub struct Stopwatch {
    start_time: f64,
    paused_at: Option<f64>,
    speed: f64,
}

/// A stopwatch which tracks time in seconds.
impl Stopwatch {
    /// Creates a new stopwatch with the current time set to 0.
    pub fn new() -> Self {
        Self::with_speed(1.0)
    }

    /// Creates a stopwatch which advances the given amount every second.
    ///
    /// For instance, `Stopwatch::with_speed(1.0/60.0)` creates a stopwatch which uses
    /// minutes as the time unit instead of seconds.
    pub fn with_speed(speed: f64) -> Self {
        let cur_time = Self::get_raw_time();
        Self { start_time: cur_time, paused_at: None, speed }
    }

    /// Returns whether the stopwatch is paused.
    pub fn paused(&self) -> bool {
        self.paused_at.is_some()
    }

    /// Pauses the stopwatch. If the stopwatch was already paused, this does nothing.
    pub fn pause(&mut self) {
        self.paused_at = Some(self.get_end_time());
    }

    /// Unpauses the stopwatch. If the stopwatch was already unpaused, this does nothing.
    pub fn unpause(&mut self) {
        if self.paused() {
            self.start_time = Self::get_raw_time() - self.get_time();
            self.paused_at = None;
        }
    }

    /// Toggles whether the stopwatch is paused.
    pub fn toggle_pause(&mut self) {
        if self.paused() {
            self.unpause();
        } else {
            self.pause();
        }
    }

    /// Gets the current time.
    pub fn get_time(&self) -> f64 {
        (self.get_end_time() - self.start_time) * self.speed
    }

    /// Sets the current time.
    pub fn set_time(&mut self, cur_time: f64) {
        self.start_time = self.get_end_time() - cur_time / self.speed;
        if self.paused() {
            self.paused_at = Some(self.start_time);
        }
    }

    /// Resets the stopwatch to zero.
    pub fn reset(&mut self) {
        self.set_time(0.0);
    }

    /// Advances the stopwatch by `time_diff`.
    pub fn add_time(&mut self, time_diff: f64) {
        self.start_time -= time_diff / self.speed;
        match self.paused_at {
            None => (),
            Some(paused_at) => self.paused_at = Some(paused_at + time_diff),
        }
    }

    /// Sleeps until this stopwatch reaches the given time. May sleep for slightly longer than
    /// requested (the same behavior as `std::thread::sleep`), so in practice most games should
    /// use vsync or similar mechanisms instead of using this to maintain a certain frame rate.
    ///
    /// Panics if the stopwatch is paused.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn sleep_until(&self, time: f64) {
        assert!(!self.paused());
        let time_diff = time / self.speed - self.get_time();
        Self::sleep(time_diff);
    }

    fn get_end_time(&self) -> f64 {
        match self.paused_at {
            None => Self::get_raw_time(),
            Some(paused_at) => paused_at,
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn get_raw_time() -> f64 {
        window().unwrap().performance().unwrap().now() / 1000.0
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn get_raw_time() -> f64 {
        (time::OffsetDateTime::now_utc() - time::OffsetDateTime::UNIX_EPOCH).as_seconds_f64()
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn sleep(time_diff: f64) {
        if time_diff > 0.0 {
            std::thread::sleep(std::time::Duration::from_secs_f64(time_diff));
        }
    }
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self::new()
    }
}
