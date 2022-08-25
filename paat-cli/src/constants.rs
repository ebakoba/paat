use lazy_static::lazy_static;
use std::time::Duration;

pub const TIMEOUT_BETWEEN_REQUESTS: u64 = 30;

lazy_static! {
    pub static ref TICK_TIMEOUT_DURATION: Duration = Duration::from_secs(1);
}
