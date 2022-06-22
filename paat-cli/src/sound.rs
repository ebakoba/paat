use anyhow::Result;
use rodio::{Decoder, OutputStream, Source};
use std::{include_bytes, io::Cursor, time::Duration};
use tokio::time::sleep;

pub async fn play_success_sound(timeout: u64) -> Result<()> {
    #[cfg(target_os = "linux")]
    let sound_bytes = include_bytes!("../assets/sample.wav");
    #[cfg(target_os = "windows")]
    let sound_bytes = include_bytes!(r"..\assets\sample.wav");

    let (_out, stream_handle) = OutputStream::try_default()?;
    let cursor = Cursor::new(sound_bytes.as_ref());
    let decoder = Decoder::new(cursor)?;
    stream_handle.play_raw(decoder.convert_samples())?;

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    sleep(Duration::from_secs(timeout)).await;
    Ok(())
}
