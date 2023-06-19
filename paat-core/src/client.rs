use crate::{
    datetime::naive_date_to_output_string,
    types::{
        event::{EventMap, EventResponse, WaitForSpot},
        Direction,
    },
    url::EVENTS_URL,
};
use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use futures::{stream, Stream, StreamExt};
use reqwest::Client as ReqwestClient;
use std::time::Duration;
use strum::EnumProperty;
use tokio::time::sleep;

#[derive(Clone)]
pub struct Client {
    client: ReqwestClient,
    pause_between_stream_items: Duration,
}

impl Client {
    pub fn new(pause_between_stream_items: Duration) -> Self {
        Self {
            client: reqwest::Client::new(),
            pause_between_stream_items,
        }
    }

    pub async fn fetch_events(
        &self,
        departure_date: &NaiveDate,
        direction: &Direction,
    ) -> Result<EventMap> {
        let body = self
            .client
            .get(EVENTS_URL)
            .query(&[
                (
                    "direction",
                    direction.get_str("Abbreviation").unwrap().to_string(),
                ),
                (
                    "departure-date",
                    naive_date_to_output_string(departure_date),
                ),
            ])
            .send()
            .await?
            .text()
            .await?;
        if let Ok(event_response) = serde_json::from_str::<EventResponse>(&body) {
            return Ok(event_response
                .items
                .into_iter()
                .map(|event| (event.uuid.clone(), event))
                .collect());
        }

        Err(anyhow!("Failed to fetch events"))
    }

    fn create_event_stream<'a>(
        &'a self,
        departure_date: &'a NaiveDate,
        direction: &'a Direction,
    ) -> impl Stream<Item = Result<EventMap>> + 'a {
        stream::iter(0..).then(move |i| async move {
            if i > 0 {
                sleep(self.pause_between_stream_items).await;
            }
            self.fetch_events(departure_date, direction).await
        })
    }

    pub fn create_wait_stream<'a>(
        &'a self,
        departure_date: &'a NaiveDate,
        direction: &'a Direction,
        event_uuid: &'a str,
    ) -> impl Stream<Item = Result<WaitForSpot>> + 'a {
        self.create_event_stream(departure_date, direction)
            .map(move |event_map_result| {
                let event_map = event_map_result?;
                if let Some(event) = event_map.get(event_uuid) {
                    if event.capacities.small_vehicles > 0 {
                        return Ok(WaitForSpot::Done(event.capacities.small_vehicles as usize));
                    }
                    return Ok(WaitForSpot::Waiting);
                }
                Err(anyhow!(
                    "Failed to find corresponding event with following uuid: {}",
                    event_uuid
                ))
            })
    }
}
