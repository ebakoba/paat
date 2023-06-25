use anyhow::{anyhow, Result};
use futures::{future::FutureExt, pin_mut, select};
use log::debug;
use paat_core::sound::play_infinite_sound;
use tokio::signal::ctrl_c;
use tokio::sync::oneshot;

async fn print_end_text(number_of_spots: usize) {
    println!("Found {} spot(s)", number_of_spots);
    println!();
    println!("Press CTRL+c to exit");
}

pub async fn create_final_output(number_of_spots: usize) -> Result<()> {
    let (sender, receiver) = oneshot::channel::<()>();
    let ctrl_c_future = ctrl_c().fuse();
    let music_future = play_infinite_sound(receiver).fuse();
    let text_future = print_end_text(number_of_spots).fuse();

    pin_mut!(ctrl_c_future, music_future, text_future);

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
          _ = text_future => continue
        }
    }
    sender
        .send(())
        .map_err(|_| anyhow!("Failed to send message to the receiver"))?;
    Ok(())
}
