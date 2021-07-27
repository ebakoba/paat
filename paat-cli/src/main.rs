use actix::Actor;
use paat_core::{actors::event::{EventManager, WaitForSpot, FetchEvents}, datetime::{get_naive_date}};
use dialoguer::Input;

#[actix::main]
async fn main() {
  let date_input : String = Input::new()
    .with_prompt("Please enter the date to watch")
    .default("2021-07-30".into())
    .interact_text().unwrap();
  let departure_date = get_naive_date(&date_input).unwrap();
  println!("Departure date: {:?}", departure_date);

  let address = EventManager::new(departure_date).start();

  match address.send(FetchEvents).await {
    Ok(Ok(Some(events))) => {
      if let Some(first_event) = events.get(0) {
        address.send(WaitForSpot(first_event.uuid.clone())).await.unwrap().unwrap();
      }
    }
    _ => {
      println!("Failed to fetch events");
    }
  }

  println!("I am all finished");
}
