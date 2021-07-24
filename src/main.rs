mod probe;
mod types;

use actix::Actor;
use probe::Probe;
use chrono::{NaiveDate};
use crate::probe::FindSpot;

#[actix::main]
async fn main() {
  println!("Hello, world!");

  let address = Probe::new().start();

  let datetime = NaiveDate::from_ymd(2021, 7, 24).and_hms(17, 30, 00);
  address.send(FindSpot(datetime)).await;

  println!("I am all finished");
}
