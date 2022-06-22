use anyhow::{Ok, Result};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::{include_bytes, io::Cursor};

pub async fn play_infinate_sound() -> Result<()> {
    let sound_bytes = include_bytes!("../assets/sample.wav");

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    let cursor = Cursor::new(sound_bytes.as_ref());
    let decoder = Decoder::new(cursor)?;

    sink.append(decoder.repeat_infinite());
    tokio::spawn(async move {
        sink.sleep_until_end();
    })
    .await?;
    Ok(())
}
