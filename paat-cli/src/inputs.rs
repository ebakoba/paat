use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use paat_core::{
    datetime::{get_naive_date, naive_date_to_input_string},
    types::event::{Direction, Event},
};
use std::{collections::HashMap, io, str::FromStr};

pub fn input_departure_date() -> io::Result<NaiveDate> {
    let current_date = chrono::Utc::now().naive_local().date();
    let date_input: String = Input::new()
        .with_prompt("Daparture date")
        .default(naive_date_to_input_string(&current_date))
        .interact_text()?;
    let departure_date = get_naive_date(&date_input)
        .map_err(|_| io::Error::new(io::ErrorKind::Unsupported, "Unsupported date"))?;

    Ok(departure_date)
}

pub fn input_direction() -> io::Result<Direction> {
    let directions = [
        "Heltermaa - Rohuküla",
        "Rohuküla - Heltermaa",
        "Kuivastu - Virtsu",
        "Virtsu - Kuivastu",
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&directions)
        .interact()?;
    let direction = Direction::from_str(directions[selection])
        .map_err(|_| io::Error::new(io::ErrorKind::Unsupported, "Unknown direction"))?;

    println!("Direction: {}", direction);
    Ok(direction)
}

pub fn input_event<'a>(event_map: HashMap<String, Event>) -> Result<Event> {
    let mut events = event_map.values().collect::<Vec<&Event>>();
    if events.len() == 0 {
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
