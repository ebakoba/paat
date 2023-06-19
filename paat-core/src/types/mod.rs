pub mod event;
use crate::constants::LINES;
use std::str::FromStr;
use strum_macros::{Display, EnumProperty, EnumString};

#[derive(Display, Debug, Clone, Copy, EnumString, EnumProperty, Default)]
pub enum Direction {
    #[default]
    #[strum(props(Abbreviation = "HR"), to_string = "Heltermaa - Rohuküla")]
    HR,
    #[strum(props(Abbreviation = "RH"), to_string = "Rohuküla - Heltermaa")]
    RH,
    #[strum(props(Abbreviation = "KV"), to_string = "Kuivastu - Virtsu")]
    KV,
    #[strum(props(Abbreviation = "VK"), to_string = "Virtsu - Kuivastu")]
    VK,
}

impl Direction {
    pub fn get_line_by_index(index: usize) -> Option<Self> {
        if index >= LINES.len() {
            return None;
        }
        if let Ok(direction) = Self::from_str(LINES[index]) {
            return Some(direction);
        }
        None
    }
}
