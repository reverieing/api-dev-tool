use std::time::Duration;

pub struct Config {
    pub timeout_secs: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            timeout_secs: 30,
        }
    }
}

impl Config {
    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout_secs)
    }
}