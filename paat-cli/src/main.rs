mod inputs;

use actix::Actor;
use dialoguer::{theme::ColorfulTheme, Select};
use paat_core::actors::event::{EventManager, FetchEvents, WaitForSpot};
use std::io;

use crate::inputs::{input_departure_date, input_direction};

#[actix::main]
async fn main() -> io::Result<()> {
    let departure_date = input_departure_date()?;
    let direction = input_direction()?;
    let address = EventManager::new(departure_date, direction).start();

    match address.send(FetchEvents).await {
        Ok(Ok(Some(mut events))) => {
            events.sort_by_key(|event| event.start.clone());
            let selection: usize = Select::with_theme(&ColorfulTheme::default())
                .items(&events)
                .interact()?;
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
    Ok(())
}
