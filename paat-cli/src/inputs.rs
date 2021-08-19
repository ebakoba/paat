use chrono::{NaiveDate, ParseResult};
use dialoguer::Input;
use paat_core::datetime::get_naive_date;

pub fn input_departure_date() -> ParseResult<NaiveDate> {
    let date_input: String = Input::new()
        .with_prompt("Please enter the date to watch")
        .default("2021-07-30".into())
        .interact_text()
        .unwrap();
    let departure_date = get_naive_date(&date_input)?;

    println!("Departure date: {:?}", departure_date);
    Ok(departure_date)
}
