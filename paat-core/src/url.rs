use const_format::formatcp;

const BASE_URL: &str = "https://www.praamid.ee/online";
pub const EVENTS_URL: &str = formatcp!("{}/events", BASE_URL);
