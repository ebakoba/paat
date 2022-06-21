mod constants;
mod inputs;
mod sound;

use crate::{
    constants::{TICK_TIMEOUT, TIMEOUT_BETWEEN_REQUESTS},
    inputs::{input_departure_date, input_direction},
};
use anyhow::{anyhow, Result};
use dialoguer::{theme::ColorfulTheme, Select};
use env_logger::init;
use futures::StreamExt;
use indicatif::ProgressBar;
use log::debug;
use paat_core::{
    client::Client,
    types::event::{Event, WaitForSpot},
};
use sound::play_success_sound;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    init();
    let direction = input_direction()?;
    let departure_date = input_departure_date()?;

    let client = Client::new(Duration::from_secs(TIMEOUT_BETWEEN_REQUESTS));
    let event_map = client.fetch_events(&departure_date, &direction).await?;
    let mut events = event_map.values().collect::<Vec<&Event>>();
    events.sort_by_key(|event| event.start.clone());

    let selection: usize = Select::with_theme(&ColorfulTheme::default())
        .items(&events)
        .interact()?;
    let selected_event = &events[selection];
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.enable_steady_tick(TICK_TIMEOUT);

    let mut wait_stream =
        Box::pin(client.create_wait_stream(&departure_date, &direction, &selected_event.uuid));

    let mut wait_counter: usize = 0;
    while let Some(wait_result) = wait_stream.next().await {
        let wait_response = wait_result?;
        match wait_response {
            WaitForSpot::Done(number_of_spots) => {
                progress_bar.finish_with_message(format!("Found {} spot(s)", number_of_spots));
                if let Err(music_error) = play_success_sound(5).await {
                    debug!("Failed to play sound: {}", music_error);
                }
                return Ok(());
            }
            WaitForSpot::Waiting => {
                wait_counter += 1;
                progress_bar.set_message(format!(
                    "\tNumber of tries: {}, time between requests is {} seconds",
                    wait_counter, TIMEOUT_BETWEEN_REQUESTS
                ));
            }
        }
    }

    Err(anyhow!("Failed to get an event from the event stream"))
}
