mod types;
mod url;
pub mod datetime;

use std::time::Duration as StandardDuration;
use tokio::time::sleep;
use async_recursion::async_recursion;
use reqwest::Client;
use actix::{Actor, Context, Handler, Message, ResponseFuture};
use chrono::{Date, NaiveDateTime, Utc};
use crate::types::event::{Event, EventResponse};
use url::{EVENTS_URL};

#[derive(Message)]
#[rtype(result = "Result<Vec<Event>, reqwest::Error>")]
pub struct FetchEvents(pub Date<Utc>);

impl FetchEvents {
  pub async fn fetch_events(&self, client: &Client) -> Result<Vec<Event>, reqwest::Error> {
    let requested_datetime = self.0; 
    let body = client.get(EVENTS_URL)
      .query(&[("direction", "HR"), ("departure-date", &requested_datetime.format("%Y-%m-%d").to_string())])
      .send()
      .await?
      .text()
      .await?;

    match serde_json::from_str::<EventResponse>(&body) {
      Ok(event_response) => Ok(event_response.items),
      Err(_) => Ok(vec![])
    }
  }
}

#[derive(Message)]
#[rtype(result = "Result<(), reqwest::Error>")]
pub struct FindSpot(pub NaiveDateTime);

impl FindSpot {
  pub fn find_matching_event(&self, events: Vec<Event>) -> Option<Event> {
    let target_datetime = self.0;
    for event in events {
      match NaiveDateTime::parse_from_str(&event.start, "%Y-%m-%dT%H:%M:%S%.f%z") {
        Ok (start_datetime) => {
          println!("Start datetime: {:?}", start_datetime);
          if target_datetime == start_datetime {
            return Some(event.into())
          }
        }
        Err(_) => {}
      }
    }

    None
  }

  #[async_recursion]
  pub async fn wait_for_opening(&self, client: &Client) -> Result<(), reqwest::Error> {
    let requested_datetime = self.0; 
    let body = client.get(EVENTS_URL)
      .query(&[("direction", "HR"), ("departure-date", &requested_datetime.format("%Y-%m-%d").to_string())])
      .send()
      .await?
      .text()
      .await?;

    match serde_json::from_str::<EventResponse>(&body) {
      Ok(response) => {
        if let Some(event) = self.find_matching_event(response.items.clone()) {
          println!("Event: {:?}", event);
          if event.capacities.small_vehicles < 1 {
            println!("Polling some more");
            sleep(StandardDuration::from_secs(10)).await;
            return self.wait_for_opening(client).await;
          } else {
            println!("Answer found");
          }
        } else {
          println!("No matching event");
        }
      }
      Err(error) => {
        println!("Error: {:?}", error);
      }
    }

    Ok(())
  }
}

pub struct Probe {
  client: reqwest::Client,
}

impl Probe {
  pub fn new() -> Self {
    Self {
      client: reqwest::Client::new()
    }
  }
}

impl Actor for Probe {
  type Context = Context<Self>;
}

impl Handler<FindSpot> for Probe {
  type Result = ResponseFuture<Result<(), reqwest::Error>>;

  fn handle(&mut self, message: FindSpot, _ctx: &mut Context<Self>) -> Self::Result {
    let client = self.client.clone();
    Box::pin(async move {
      message.wait_for_opening(&client).await?;

      Ok(())
    })
  }
}

impl Handler<FetchEvents> for Probe {
  type Result = ResponseFuture<Result<Vec<Event>, reqwest::Error>>;

  fn handle(&mut self, message: FetchEvents, _ctx: &mut Context<Self>) -> Self::Result {
    let client = self.client.clone();
    Box::pin(async move {
      let events = message.fetch_events(&client).await?;

      Ok(events)
    })
  }
}