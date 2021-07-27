use std::time::Duration as StandardDuration;
use tokio::time::sleep;
use async_recursion::async_recursion;
use reqwest::Client;
use actix::{Actor, Context, Handler, Message, ResponseFuture};
use chrono::{NaiveDate};
use crate::services::event::{Event, EventResponse};
use crate::url::{EVENTS_URL};

pub struct EventManager {
  client: reqwest::Client,
  departure_date: NaiveDate,
}

impl EventManager {
  pub fn new(departure_date: NaiveDate) -> Self {
    Self {
      client: reqwest::Client::new(),
      departure_date,
    }
  }

  pub async fn fetch_events(client: &Client, departure_date: &NaiveDate) -> Result<Option<Vec<Event>>, reqwest::Error> {
    let body = client.get(EVENTS_URL)
      .query(&[("direction", "HR"), ("departure-date", &departure_date.format("%Y-%m-%d").to_string())])
      .send()
      .await?
      .text()
      .await?;

    let events = match serde_json::from_str::<EventResponse>(&body) {
      Ok(event_response) => {
        Some(event_response.items)
      },
      Err(_) => {
        None
      }
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
    Box::pin(async move {
      EventManager::fetch_events(&client, &departure_date).await
    })
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
  pub async fn wait_for_spot(&self, client: &Client, departure_date: &NaiveDate) -> Result<(), reqwest::Error> { 
    let events_option = EventManager::fetch_events(client, departure_date).await?;
    
    if let Some(events) = events_option {
      if let Some(event) = self.find_matching_event(events) {
        println!("Event: {:?}", event);
        if event.capacities.small_vehicles < 1 {
          println!("Polling some more");
          sleep(StandardDuration::from_secs(10)).await;
          return self.wait_for_spot(client, departure_date).await;
        } else {
          println!("Answer found");
        }
      } else {
        println!("No matching event");
      }
    } else {
      println!("No events found")
    }
     
    Ok(())
  }
}

impl Handler<WaitForSpot> for EventManager {
  type Result = ResponseFuture<Result<(), reqwest::Error>>;

  fn handle(&mut self, message: WaitForSpot, _ctx: &mut Context<Self>) -> Self::Result {
    let client = self.client.clone();
    let departure_date = self.departure_date.clone();
    Box::pin(async move {
      message.wait_for_spot(&client, &departure_date).await?;

      Ok(())
    })
  }
}
