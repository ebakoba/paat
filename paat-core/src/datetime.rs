use chrono::{NaiveDate, ParseError};

const BASE_DATE_FORMAT: &str = "%Y-%m-%d";

pub fn get_date(input: &str) -> Result<NaiveDate, ParseError> {
  NaiveDate::parse_from_str(input, BASE_DATE_FORMAT)
}
