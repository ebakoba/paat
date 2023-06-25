use anyhow::{Ok, Result};
use rodio::{Decoder, OutputStream, Sink, Source};
use send_wrapper::SendWrapper;
use std::{include_bytes, io::Cursor, rc::Rc};
use tokio::sync::oneshot::Receiver;

pub async fn play_infinite_sound(receiver: Receiver<()>) -> Result<()> {
    let sound_bytes = include_bytes!("../assets/sample.wav");

    let _wrapped_stream: SendWrapper<Rc<OutputStream>>;
    let sink: Sink;
    {
        let (stream, stream_handle) = OutputStream::try_default()?;
        sink = Sink::try_new(&stream_handle)?;
        _wrapped_stream = SendWrapper::new(Rc::new(stream));
    }

    let cursor = Cursor::new(sound_bytes.as_ref());
    let decoder = Decoder::new(cursor)?;
    sink.append(decoder.repeat_infinite());
    receiver.await?;

    Ok(())
}
