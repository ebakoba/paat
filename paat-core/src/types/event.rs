use crate::datetime::service_datetime_to_local_time_string;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
};
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Capacity {
    #[serde(rename(deserialize = "pcs"))]
    pub passangers: i32,
    #[serde(rename(deserialize = "bc"))]
    pub bc: i32,
    #[serde(rename(deserialize = "sv"))]
    pub small_vehicles: i32,
    #[serde(rename(deserialize = "bv"))]
    pub large_vehicles: i32,
    #[serde(rename(deserialize = "dc"))]
    pub dc: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeWrapper {
    code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    #[serde(rename(deserialize = "uid"))]
    pub uuid: String,
    pub capacities: Capacity,
    #[serde(rename(deserialize = "pricelist"))]
    pub price_list: CodeWrapper,
    #[serde(rename(deserialize = "transportationType"))]
    pub transportation_type: CodeWrapper,
    pub ship: CodeWrapper,
    pub status: String,
    #[serde(rename(deserialize = "dtstart"))]
    pub start: String,
    #[serde(rename(deserialize = "dtend"))]
    pub end: String,
}

pub type EventMap = HashMap<String, Event>;

pub enum WaitForSpot {
    Done(usize),
    Waiting,
}

impl Display for Event {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let parse_results = (
            service_datetime_to_local_time_string(&self.start),
            service_datetime_to_local_time_string(&self.end),
        );
        match parse_results {
            (Ok(start_time_string), Ok(end_time_string)) => {
                let text = format!("{} - {}", start_time_string, end_time_string);
                fmt.write_str(&text)?;
            }
            _ => return Err(core::fmt::Error),
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventResponse {
    #[serde(rename(deserialize = "totalCount"))]
    pub total_count: i32,
    pub items: Vec<Event>,
}
