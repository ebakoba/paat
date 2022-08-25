mod constants;
mod inputs;
mod output;
mod sound;

use crate::{
    constants::{TICK_TIMEOUT_DURATION, TIMEOUT_BETWEEN_REQUESTS},
    inputs::{input_departure_date, input_direction},
};
use anyhow::{anyhow, Result};
use env_logger::init;
use futures::StreamExt;
use indicatif::ProgressBar;
use inputs::input_event;
use output::create_final_output;
use paat_core::{client::Client, types::event::WaitForSpot};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    init();
    let direction = input_direction()?;
    let departure_date = input_departure_date()?;

    let client = Client::new(Duration::from_secs(TIMEOUT_BETWEEN_REQUESTS));
    let event_map = client.fetch_events(&departure_date, &direction).await?;

    let selected_event = input_event(event_map)?;
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.enable_steady_tick(*TICK_TIMEOUT_DURATION);

    let mut wait_stream =
        Box::pin(client.create_wait_stream(&departure_date, &direction, &selected_event.uuid));

    let mut wait_counter: usize = 0;
    while let Some(wait_result) = wait_stream.next().await {
        let wait_response = wait_result?;
        match wait_response {
            WaitForSpot::Done(number_of_spots) => {
                progress_bar.finish_and_clear();
                create_final_output(number_of_spots).await;
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
