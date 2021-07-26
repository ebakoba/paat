use actix::Actor;
use chrono::{NaiveDate};
use paat_core::{Probe, FindSpot};
use dialoguer::Input;

#[actix::main]
async fn main() {
  println!("Hello, world!");

  let input : String = Input::new()
    .with_prompt("Tea or coffee?")
    .with_initial_text("Yes")
    .default("No".into())
    .interact_text().unwrap();
  println!("Input was: {:?}", input);

  let address = Probe::new().start();

  let datetime = NaiveDate::from_ymd(2021, 7, 24).and_hms(17, 30, 00);
  address.send(FindSpot(datetime)).await;

  println!("I am all finished");
}
