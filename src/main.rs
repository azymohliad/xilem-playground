use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use masonry::{vello::peniko::Gradient, Point};
use plot_view::bar_plot;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use xilem::{core::fork, view::async_repeat_raw, Color, EventLoop, WidgetView, Xilem};

mod plot_view;
mod plot_widget;

struct AppData {
    receiver: Arc<Mutex<mpsc::Receiver<Vec<f64>>>>,
    plot: Arc<Vec<f64>>,
}

impl AppData {
    fn new(receiver: mpsc::Receiver<Vec<f64>>) -> Self {
        Self {
            receiver: Arc::new(Mutex::new(receiver)),
            plot: Arc::new(Vec::new()),
        }
    }
}

fn app_logic(data: &mut AppData) -> impl WidgetView<AppData> {
    let rx = data.receiver.clone();
    fork(
        bar_plot(data.plot.clone())
            .background(
                Gradient::new_linear(Point::new(0., 0.), Point::new(0., 1.))
                    .with_stops([Color::rgb(0.3, 0.3, 0.3), Color::rgb(0.1, 0.3, 0.2)]),
            )
            .foreground(
                Gradient::new_linear(Point::new(0., 0.), Point::new(0., 1.))
                    .with_stops([Color::rgb(0.7, 0.7, 0.7), Color::rgb(0.5, 0.7, 0.6)]),
            )
            .gap(3.),
        async_repeat_raw(
            move |proxy| {
                let rx = rx.clone();
                async move {
                    let mut rx = rx.lock().await;
                    while let Some(samples) = rx.recv().await {
                        if proxy.message(samples).is_err() {
                            break;
                        }
                    }
                }
            },
            |data: &mut AppData, samples: Vec<f64>| {
                data.plot = Arc::new(samples);
            },
        ),
    )
}

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

fn main() {
    let (sender, receiver) = mpsc::channel(8);
    let host = cpal::default_host();
    let device = host.default_input_device().unwrap();

    let config = device.default_input_config().unwrap();
    let stream = match config.sample_format() {
        cpal::SampleFormat::I8 => device
            .build_input_stream(
                &config.into(),
                move |data, _: &_| send_buffer::<i8>(data, sender.clone()),
                stream_error,
                None,
            )
            .unwrap(),
        cpal::SampleFormat::I16 => device
            .build_input_stream(
                &config.into(),
                move |data, _: &_| send_buffer::<i16>(data, sender.clone()),
                stream_error,
                None,
            )
            .unwrap(),
        cpal::SampleFormat::I32 => device
            .build_input_stream(
                &config.into(),
                move |data, _: &_| send_buffer::<i32>(data, sender.clone()),
                stream_error,
                None,
            )
            .unwrap(),
        cpal::SampleFormat::F32 => device
            .build_input_stream(
                &config.into(),
                move |data, _: &_| send_buffer::<f32>(data, sender.clone()),
                stream_error,
                None,
            )
            .unwrap(),
        sample_format => {
            eprintln!("Unsupported sample format '{sample_format}'");
            return;
        }
    };

    stream.play().unwrap();

    let data = AppData::new(receiver);

    Xilem::new(data, app_logic)
        .background_color(Color::rgb8(0x20, 0x20, 0x20))
        .run_windowed(EventLoop::with_user_event(), "Bar Plot".into())
        .unwrap();
}
