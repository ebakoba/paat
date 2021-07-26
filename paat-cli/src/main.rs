use actix::Actor;
use chrono::{NaiveDate};
use paat_core::{Probe, FindSpot, datetime::{get_date}};
use dialoguer::Input;

#[actix::main]
async fn main() {
  let date_input : String = Input::new()
    .with_prompt("Please enter the date to watch")
    .default("2021-07-30".into())
    .interact_text().unwrap();
  let date_to_search = get_date(&date_input).unwrap();
  println!("Date to search: {:?}", date_to_search);

  let address = Probe::new().start();

  let datetime = NaiveDate::from_ymd(2021, 7, 24).and_hms(17, 30, 00);
  address.send(FindSpot(datetime)).await;

  println!("I am all finished");
}
