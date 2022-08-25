pub mod event;
use strum_macros::{Display, EnumProperty, EnumString};

#[derive(Display, Debug, Clone, Copy, EnumString, EnumProperty)]
pub enum Direction {
    #[strum(props(Abbreviation = "HR"), to_string = "Heltermaa - Rohuküla")]
    HR,
    #[strum(props(Abbreviation = "RH"), to_string = "Rohuküla - Heltermaa")]
    RH,
    #[strum(props(Abbreviation = "KV"), to_string = "Kuivastu - Virtsu")]
    KV,
    #[strum(props(Abbreviation = "VK"), to_string = "Virtsu - Kuivastu")]
    VK,
}
