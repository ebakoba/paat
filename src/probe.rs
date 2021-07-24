use std::str::FromStr;
use std::time::Duration as StandardDuration;
use tokio::time::sleep;
use async_recursion::async_recursion;
use reqwest::Client;
use actix::{Actor, ActorFutureExt, AsyncContext, Context, Handler, Message, ResponseActFuture, ResponseFuture, WrapFuture};
use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Capacity {
  #[serde(rename(deserialize = "pcs"))]
  passangers: i32,
  #[serde(rename(deserialize = "bc"))]
  bc: i32,
  #[serde(rename(deserialize = "sv"))]
  small_vehicles: i32,
  #[serde(rename(deserialize = "bv"))]
  large_vehicles: i32,
  #[serde(rename(deserialize = "dc"))]
  dc: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeWrapper {
  code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
  #[serde(rename(deserialize = "uid"))]
  uuid: String,
  capacities: Capacity,
  #[serde(rename(deserialize = "pricelist"))]
  price_list: CodeWrapper,
  #[serde(rename(deserialize = "transportationType"))]
  transportation_type: CodeWrapper,
  ship: CodeWrapper,
  status: String,
  #[serde(rename(deserialize = "dtstart"))]
  start: String,
  #[serde(rename(deserialize = "dtend"))]
  end: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventResponse {
  #[serde(rename(deserialize = "totalCount"))]
  total_count: i32,
  items: Vec<Event>
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
    let body = client.get("https://www.praamid.ee/online/events")
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
            sleep(StandardDuration::from_secs(5)).await;
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

  fn started(&mut self, _ctx: &mut Self::Context) {
    println!("I am alive!");
  }
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