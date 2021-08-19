use chrono::{
    Date, DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, ParseError, ParseResult,
    TimeZone, Utc,
};

const DATE_FORMAT: &str = "%Y-%m-%d";
const DATETIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f%z";
const TIME_FORMAT: &str = "%H:%M";

pub fn get_naive_date(input: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(input, DATE_FORMAT)
}

pub fn get_local_date(input: &str) -> Result<Option<Date<Local>>, ParseError> {
    let naive_date = get_naive_date(input)?;
    Ok(Local.from_local_date(&naive_date).single())
}

pub fn get_utc_date(input: &str) -> Result<Date<Utc>, ParseError> {
    let naive_date = get_naive_date(input)?;
    Ok(Utc.from_utc_date(&naive_date))
}

pub fn get_naive_datetime(input: &str) -> Result<NaiveDateTime, ParseError> {
    NaiveDateTime::parse_from_str(input, DATETIME_FORMAT)
}

pub fn get_utc_datetime(input: &str) -> Result<DateTime<Utc>, ParseError> {
    let naive_datetime = get_naive_datetime(input)?;
    Ok(Utc.from_utc_datetime(&naive_datetime))
}

pub fn get_local_datetime(input: &str) -> ParseResult<DateTime<Local>> {
    let fixed_offset_datetime = DateTime::parse_from_str(input, DATETIME_FORMAT)?;
    Ok(fixed_offset_datetime_to_local(fixed_offset_datetime))
}

pub fn fixed_offset_datetime_to_utc(fixed_offset_datetime: DateTime<FixedOffset>) -> DateTime<Utc> {
    DateTime::from(fixed_offset_datetime)
}

pub fn fixed_offset_datetime_to_local(
    fixed_offset_datetime: DateTime<FixedOffset>,
) -> DateTime<Local> {
    DateTime::from(fixed_offset_datetime)
}

pub fn rfc3339_to_utc(input: &str) -> ParseResult<DateTime<Utc>> {
    let fixed_offset_datetime = DateTime::parse_from_rfc3339(input)?;
    Ok(fixed_offset_datetime_to_utc(fixed_offset_datetime))
}

pub fn rfc3339_to_local(input: &str) -> ParseResult<DateTime<Local>> {
    let fixed_offset_datetime = DateTime::parse_from_rfc3339(input)?;
    Ok(fixed_offset_datetime_to_local(fixed_offset_datetime))
}

pub fn local_datetime_to_time_string(local_datetime: DateTime<Local>) -> String {
    local_datetime.format(TIME_FORMAT).to_string()
}

pub fn rfc3339_to_local_time_string(input: &str) -> ParseResult<String> {
    let local_datetime = rfc3339_to_local(input)?;
    Ok(local_datetime_to_time_string(local_datetime))
}

pub fn service_datetime_to_local_time_string(input: &str) -> ParseResult<String> {
    let local_datetime = get_local_datetime(input)?;
    Ok(local_datetime_to_time_string(local_datetime))
}

pub fn naive_date_to_string(naive_date: &NaiveDate) -> String {
    naive_date.format(DATE_FORMAT).to_string()
}
