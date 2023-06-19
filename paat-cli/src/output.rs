use crate::sound::play_infinite_sound;
use futures::{
    future::FutureExt, // for `.fuse()`
    pin_mut,
    select,
};
use log::debug;
use tokio::signal::ctrl_c;

async fn print_end_text(number_of_spots: usize) {
    println!("Found {} spot(s)", number_of_spots);
    println!();
    println!("Press CTRL+c to exit");
}

pub async fn create_final_output(number_of_spots: usize) {
    let ctrl_c_future = ctrl_c().fuse();
    let music_future = play_infinite_sound().fuse();
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
              debug!("Failed to play music: {}", music_error);
            }
            continue
          },
          _ = text_future => continue
        }
    }
}
