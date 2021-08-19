use crate::datetime::naive_date_to_string;
use crate::services::event::{Event, EventResponse};
use crate::url::EVENTS_URL;
use actix::{Actor, Context, Handler, Message, ResponseFuture};
use async_recursion::async_recursion;
use chrono::NaiveDate;
use log::debug;
use reqwest::Client;
use std::time::Duration as StandardDuration;
use strum::EnumProperty;
use strum_macros::{Display, EnumProperty, EnumString};
use tokio::time::sleep;

#[derive(Display, Debug, PartialEq, Clone, Copy, EnumString, EnumProperty)]
pub enum Direction {
    #[strum(props(Abbreviation = "HR"), to_string = "Heltermaa - Rohuküla")]
    HR,
    #[strum(props(Abbreviation = "RH"), to_string = "Rohuküla - Heltermaa")]
    RH,
    #[strum(props(Abbreviation = "KV"), to_string = "Kuivastu - Virtsu")]
    KV,
    #[strum(props(Abbreviation = "VK"), to_string = "Virtsu - Kuivastu")]
    VK,
}

pub struct EventManager {
    client: reqwest::Client,
    departure_date: NaiveDate,
    direction: Direction,
}

impl EventManager {
    pub fn new(departure_date: NaiveDate, direction: Direction) -> Self {
        Self {
            client: reqwest::Client::new(),
            departure_date,
            direction,
        }
    }

    pub async fn fetch_events(
        client: &Client,
        departure_date: &NaiveDate,
        direction: Direction,
    ) -> Result<Option<Vec<Event>>, reqwest::Error> {
        let body = client
            .get(EVENTS_URL)
            .query(&[
                (
                    "direction",
                    direction.get_str("Abbreviation").unwrap().to_string(),
                ),
                ("departure-date", naive_date_to_string(departure_date)),
            ])
            .send()
            .await?
            .text()
            .await?;

        let events = match serde_json::from_str::<EventResponse>(&body) {
            Ok(event_response) => Some(event_response.items),
            Err(_) => None,
        };

        Ok(events)
    }
}

impl Actor for EventManager {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<Option<Vec<Event>>, reqwest::Error>")]
pub struct FetchEvents;

impl Handler<FetchEvents> for EventManager {
    type Result = ResponseFuture<Result<Option<Vec<Event>>, reqwest::Error>>;

    fn handle(&mut self, _: FetchEvents, _ctx: &mut Context<Self>) -> Self::Result {
        let client = self.client.clone();
        let departure_date = self.departure_date.clone();
        let direction = self.direction;
        Box::pin(
            async move { EventManager::fetch_events(&client, &departure_date, direction).await },
        )
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), reqwest::Error>")]
pub struct WaitForSpot(pub String);

impl WaitForSpot {
    pub fn find_matching_event(&self, events: Vec<Event>) -> Option<Event> {
        let event_uuid = self.0.to_string();
        for event in events {
            if event.uuid == event_uuid {
                return Some(event.into());
            }
        }

        None
    }

    #[async_recursion]
    pub async fn wait_for_spot(
        &self,
        client: &Client,
        departure_date: &NaiveDate,
        direction: Direction,
    ) -> Result<(), reqwest::Error> {
        let events_option = EventManager::fetch_events(client, departure_date, direction).await?;

        if let Some(events) = events_option {
            if let Some(event) = self.find_matching_event(events) {
                if event.capacities.small_vehicles < 1 {
                    sleep(StandardDuration::from_secs(10)).await;
                    return self.wait_for_spot(client, departure_date, direction).await;
                } else {
                    debug!("Answer found");
                }
            } else {
                debug!("No matching event");
            }
        } else {
            debug!("No events found")
        }

        Ok(())
    }
}

impl Handler<WaitForSpot> for EventManager {
    type Result = ResponseFuture<Result<(), reqwest::Error>>;

    fn handle(&mut self, message: WaitForSpot, _ctx: &mut Context<Self>) -> Self::Result {
        let client = self.client.clone();
        let departure_date = self.departure_date.clone();
        let direction = self.direction;
        Box::pin(async move {
            message
                .wait_for_spot(&client, &departure_date, direction)
                .await?;

            Ok(())
        })
    }
}
