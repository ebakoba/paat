use std::sync::Arc;

use anyhow::Result;
use chrono::NaiveDate;
use futures::{lock::Mutex, StreamExt};
use paat_core::{
    client::Client,
    types::{
        event::{EventMap, WaitForSpot},
        Direction,
    },
};
use tokio::runtime::Runtime;
use tuirealm::{
    listener::{ListenerResult, Poll},
    Event,
};

#[derive(PartialEq, Clone, PartialOrd, Eq, Debug)]
pub enum ApiEvent {
    FetchedEvents(EventMap),
    WaitResult((String, WaitForSpot)),
}

#[derive(Clone)]
pub struct ApiClient {
    departure_date: NaiveDate,
    direction: Direction,
    event_map: EventMap,
    event_list_sent: bool,
    wait_list: Arc<Mutex<Vec<(String, WaitForSpot)>>>,
}

impl ApiClient {
    pub fn try_new(
        event_client: &Client,
        runtime: &Runtime,
        departure_date: NaiveDate,
        direction: Direction,
    ) -> Result<Self> {
        let event_map = runtime.block_on(event_client.fetch_events(&departure_date, &direction))?;

        Ok(Self {
            departure_date,
            direction,
            event_map,
            event_list_sent: false,
            wait_list: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub fn start_monitoring(&self, event_client: &Client, runtime: &Runtime, event_uuid: String) {
        let departure = self.departure_date.clone();
        let direction = self.direction.clone();
        let event_uuid = event_uuid.to_owned();
        let event_client = event_client.to_owned();
        let wait_list = self.wait_list.clone();
        runtime.spawn(async move {
            let mut stream =
                Box::pin(event_client.create_wait_stream(&departure, &direction, &event_uuid));
            while let Some(Ok(wait_result)) = stream.next().await {
                if let Some(mut wait_list) = wait_list.try_lock() {
                    wait_list.push((event_uuid.clone(), wait_result))
                }
            }
        });
    }
}

impl Poll<ApiEvent> for ApiClient {
    fn poll(&mut self) -> ListenerResult<Option<Event<ApiEvent>>> {
        if !self.event_list_sent {
            self.event_list_sent = true;
            return Ok(Some(Event::User(ApiEvent::FetchedEvents(
                self.event_map.clone(),
            ))));
        }
        if let Some(mut wait_list) = self.wait_list.clone().try_lock() {
            if let Some(wait_result) = wait_list.pop() {
                return Ok(Some(Event::User(ApiEvent::WaitResult(wait_result))));
            }
        }

        Ok(None)
    }
}
