use anyhow::Result;
use chrono::NaiveDate;
use paat_core::{
    client::Client,
    constants::TIMEOUT_BETWEEN_REQUESTS,
    types::{event::EventMap, Direction},
};
use std::time::Duration;
use tokio::runtime::Runtime;
use tuirealm::{
    listener::{ListenerResult, Poll},
    Event,
};

#[derive(PartialEq, Clone, PartialOrd, Eq, Debug)]
pub enum ApiEvent {
    FetchedEvents(EventMap),
}

pub struct ApiClient {
    departure_date: NaiveDate,
    direction: Direction,
    event_client: Client,
    runtime: Runtime,
    event_map: EventMap,
    event_list_sent: bool,
}

impl ApiClient {
    pub fn try_new(departure_date: NaiveDate, direction: Direction) -> Result<Self> {
        let event_client = Client::new(Duration::from_secs(TIMEOUT_BETWEEN_REQUESTS));
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        let event_map = runtime.block_on(event_client.fetch_events(&departure_date, &direction))?;

        Ok(Self {
            departure_date,
            direction,
            event_client,
            runtime,
            event_map,
            event_list_sent: false,
        })
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
        Ok(None)
    }
}
