mod inputs;

use actix::Actor;
use dialoguer::{theme::ColorfulTheme, Select};
use paat_core::actors::event::{EventManager, FetchEvents, WaitForSpot};

use crate::inputs::input_departure_date;

#[actix::main]
async fn main() {
    let departure_date = input_departure_date().unwrap();
    let address = EventManager::new(departure_date).start();

    match address.send(FetchEvents).await {
        Ok(Ok(Some(mut events))) => {
            events.sort_by_key(|event| event.start.clone());
            let selection: usize = Select::with_theme(&ColorfulTheme::default())
                .items(&events)
                .interact()
                .unwrap();
            let selected_event = &events[selection];
            address
                .send(WaitForSpot(selected_event.uuid.clone()))
                .await
                .unwrap()
                .unwrap();
        }
        _ => {
            println!("Failed to fetch events");
        }
    }

    println!("I am all finished");
}
