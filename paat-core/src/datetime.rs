use chrono::{DateTime, Local, NaiveDate, ParseError, ParseResult};

const INPUT_DATE_FORMAT: &str = "%d.%m.%Y";
const OUTPUT_DATE_FORMAT: &str = "%Y-%m-%d";
const DATETIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f%z";
const TIME_FORMAT: &str = "%H:%M";

pub fn get_naive_date(input: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(input, INPUT_DATE_FORMAT)
}

pub fn get_local_datetime(input: &str) -> ParseResult<DateTime<Local>> {
    let fixed_offset_datetime = DateTime::parse_from_str(input, DATETIME_FORMAT)?;
    Ok(DateTime::from(fixed_offset_datetime))
}

pub fn local_datetime_to_time_string(local_datetime: DateTime<Local>) -> String {
    local_datetime.format(TIME_FORMAT).to_string()
}

pub fn service_datetime_to_local_time_string(input: &str) -> ParseResult<String> {
    let local_datetime = get_local_datetime(input)?;
    Ok(local_datetime_to_time_string(local_datetime))
}

pub fn naive_date_to_input_string(naive_date: &NaiveDate) -> String {
    naive_date.format(INPUT_DATE_FORMAT).to_string()
}

pub fn naive_date_to_output_string(naive_date: &NaiveDate) -> String {
    naive_date.format(OUTPUT_DATE_FORMAT).to_string()
}
