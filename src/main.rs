mod probe;

use std::time::Duration;

use actix::Actor;
use probe::Probe;

use crate::probe::FindSpot;

#[actix::main]
async fn main() {
    println!("Hello, world!");

    let address = Probe::new().start();

    address.send(FindSpot(Duration::MAX)).await;
    
    println!("I am all finished");
}
