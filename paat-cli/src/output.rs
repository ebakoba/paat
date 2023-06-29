use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use futures::{future::FutureExt, pin_mut, select};
use log::debug;
use paat_core::booking::change_booking;
use paat_core::sound::play_infinite_sound;
use paat_core::types::event::Event;
use paat_core::types::Direction;
use tokio::signal::ctrl_c;
use tokio::sync::oneshot;

async fn print_end_text(event: &Event) {
    println!("Found {} spot(s)", event.capacities.small_vehicles);
    println!();
    println!("Press CTRL+c to exit");
}

async fn create_booking_future(
    booking_id: &Option<String>,
    event: &Event,
    direction: &Direction,
    date: &NaiveDate,
) -> Result<()> {
    let booking_id = booking_id.clone();
    if let Some(booking_id) = booking_id {
        change_booking(&booking_id, event, direction, date).await?;
    }
    Ok(())
}

pub async fn create_final_output(
    event: &Event,
    direction: &Direction,
    date: &NaiveDate,
    booking_id: &Option<String>,
) -> Result<()> {
    let (sender, receiver) = oneshot::channel::<()>();
    let ctrl_c_future = ctrl_c().fuse();
    let music_future = play_infinite_sound(receiver).fuse();
    let text_future = print_end_text(event).fuse();
    let booking_future = create_booking_future(booking_id, event, direction, date).fuse();

    pin_mut!(ctrl_c_future, music_future, text_future, booking_future);

    loop {
        select! {
          _ = ctrl_c_future => {
            println!();
            println!("Goodbye!");
            break
          },
          music_result = music_future => {
            if let Err(music_error) = music_result {
              debug!("Failed to cancel music: {}", music_error)
            }

            continue
          },
          _ = text_future => continue,
          booking_result = booking_future => {
            if let Err(_err) = booking_result {
              println!("{}", _err);
              println!("Failed to change booking");
            }

            continue
          }
        }
    }
    sender
        .send(())
        .map_err(|_| anyhow!("Failed to send message to the receiver"))?;
    Ok(())
}
