use anyhow::{anyhow, Result};
use rodio::{Decoder, OutputStream, Source};
use std::{include_bytes, io::Cursor, time::Duration};
use tokio::time::sleep;

pub async fn play_success_sound(timeout: u64) -> Result<()> {
    let bytes = if cfg!(unix) {
        include_bytes!("../assets/sample.wav")
    } else if cfg!(windows) {
        include_bytes!(r"..\assets\sample.wav")
    } else {
        return Err(anyhow!("Compiled on an unsupported platform"));
    };

    let (_, stream_handle) = OutputStream::try_default()?;
    let cursor = Cursor::new(bytes.as_ref());
    let decoder = Decoder::new(cursor)?;
    stream_handle.play_raw(decoder.convert_samples())?;

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    sleep(Duration::from_secs(timeout)).await;
    Ok(())
}
