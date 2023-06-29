use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use paat_core::{
    constants::LINES,
    datetime::{get_current_date, get_naive_date, naive_date_to_input_string},
    types::{
        event::{Event, EventMap},
        Direction,
    },
};
use std::{io, str::FromStr};

pub fn input_departure_date() -> io::Result<NaiveDate> {
    let current_date = get_current_date();
    let date_input: String = Input::new()
        .with_prompt("Departure date")
        .default(naive_date_to_input_string(&current_date))
        .interact_text()?;
    let departure_date = get_naive_date(&date_input)
        .map_err(|_| io::Error::new(io::ErrorKind::Unsupported, "Unsupported date"))?;

    Ok(departure_date)
}

pub fn input_booking_id() -> io::Result<Option<String>> {
    let booking_id: String = Input::new()
        .allow_empty(true)
        .with_prompt("Booking ID (optional)")
        .interact_text()?;
    let booking_id = booking_id.trim().to_string();
    let booking_id = match booking_id.len() {
        0 => None,
        _ => Some(booking_id),
    };
    Ok(booking_id)
}

pub fn input_direction() -> io::Result<Direction> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&LINES)
        .interact()?;
    let direction = Direction::from_str(LINES[selection])
        .map_err(|_| io::Error::new(io::ErrorKind::Unsupported, "Unknown direction"))?;

    println!("Direction: {}", direction);
    Ok(direction)
}

pub fn input_event(event_map: EventMap) -> Result<Event> {
    let mut events = event_map.values().collect::<Vec<&Event>>();
    if events.is_empty() {
        println!("No ferry times found for that date");
        return Err(anyhow!("No ferry times found for that date"));
    }
    events.sort_by_key(|event| event.start.clone());

    let selection: usize = Select::with_theme(&ColorfulTheme::default())
        .items(&events)
        .interact()?;
    println!("Selected time: {}", events[selection]);
    Ok(events[selection].to_owned())
}
