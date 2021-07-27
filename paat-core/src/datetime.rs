use chrono::{Date, Local, NaiveDate, ParseError, Utc, TimeZone};

const BASE_DATE_FORMAT: &str = "%Y-%m-%d";

pub fn get_naive_date(input: &str) -> Result<NaiveDate, ParseError> {
  NaiveDate::parse_from_str(input, BASE_DATE_FORMAT)
}

pub fn get_local_date(input: &str) -> Result<Option<Date<Local>>, ParseError> {
  let naive_date = get_naive_date(input)?;
  Ok(Local.from_local_date(&naive_date).single())
}

pub fn get_utc_date(input: &str) -> Result<Date<Utc>, ParseError> {
  let naive_date = get_naive_date(input)?;
  Ok(Utc.from_utc_date(&naive_date))
}