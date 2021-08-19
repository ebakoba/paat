use chrono::NaiveDate;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use paat_core::{actors::event::Direction, datetime::get_naive_date};
use std::{io, str::FromStr};

pub fn input_departure_date() -> io::Result<NaiveDate> {
    let date_input: String = Input::new()
        .with_prompt("Please enter the date to watch")
        .default("2021-07-30".into())
        .interact_text()?;
    let departure_date = get_naive_date(&date_input)
        .map_err(|_| io::Error::new(io::ErrorKind::Unsupported, "Unsupported date"))?;

    println!("Departure date: {:?}", departure_date);
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
