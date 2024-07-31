use anyhow::{anyhow, bail, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use xilem::tokio::sync::mpsc;

fn stream_error(error: cpal::StreamError) {
    eprintln!("An error occured on stream: {error}");
}

fn send_buffer<S>(data: &[S], sender: mpsc::Sender<Vec<f64>>)
where
    S: cpal::Sample,
    f64: cpal::FromSample<S>,
{
    sender
        .blocking_send(data.iter().map(|s| s.to_sample()).collect())
        .unwrap()
}

pub fn start_audio_input(sender: mpsc::Sender<Vec<f64>>) -> Result<cpal::Stream> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or(anyhow!("Failed to get the default input device"))?;

    let config = device.default_input_config()?;
    let stream = match config.sample_format() {
        cpal::SampleFormat::I8 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| send_buffer::<i8>(data, sender.clone()),
            stream_error,
            None,
        )?,
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| send_buffer::<i16>(data, sender.clone()),
            stream_error,
            None,
        )?,
        cpal::SampleFormat::I32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| send_buffer::<i32>(data, sender.clone()),
            stream_error,
            None,
        )?,
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data, _: &_| send_buffer::<f32>(data, sender.clone()),
            stream_error,
            None,
        )?,
        sample_format => {
            bail!("Unsupported sample format '{sample_format}'");
        }
    };

    stream.play()?;
    Ok(stream)
}
