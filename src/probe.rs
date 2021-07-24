use actix::{Context, Actor, Message, Handler, ResponseFuture};
use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeWrapper {
  code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
  #[serde(rename(deserialize = "uid"))]
  uuid: String,
  capacities: Capacity,
  #[serde(rename(deserialize = "pricelist"))]
  price_list: CodeWrapper,
  #[serde(rename(deserialize = "transportationType"))]
  transportation_type: CodeWrapper,
  ship: CodeWrapper
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventResponse {
  #[serde(rename(deserialize = "totalCount"))]
  total_count: i32,
  items: Vec<Event>
}


#[derive(Message)]
#[rtype(result = "Result<(), reqwest::Error>")]
pub struct FindSpot(pub Duration);

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
      let time = message.0;
      let client = self.client.clone();
      Box::pin(async move {
        let body = client.get("https://www.praamid.ee/online/events?direction=HR&departure-date=2021-07-24")
          .send()
          .await?
          .text()
          .await?;
         println!("Body is {:?}", body);
        match serde_json::from_str::<EventResponse>(&body) {
          Ok(response) => {
          println!("Response is {:?}", response);
          for event in response.items {
            println!("Event: {:?}", event);
          }
          }
          Err(error) => {
            println!("Error: {:?}", error);
          }
        }
        
        Ok(())
    })
  }
}