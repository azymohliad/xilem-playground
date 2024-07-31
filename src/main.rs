mod audio_input;
mod plot_view;
mod plot_widget;

use audio_input::start_audio_input;
use masonry::{vello::peniko::Gradient, Point};
use plot_view::bar_plot;
use std::sync::Arc;
use xilem::{
    core::fork, tokio::sync::mpsc, view::async_repeat, Color, EventLoop, WidgetView, Xilem,
};

struct AppData {
    signal: Arc<Vec<f64>>,
}

impl AppData {
    fn new() -> Self {
        Self {
            signal: Arc::new(Vec::new()),
        }
    }
}

fn app_logic(data: &mut AppData) -> impl WidgetView<AppData> {
    fork(
        bar_plot(data.signal.clone())
            .background(
                Gradient::new_linear(Point::new(0., 0.), Point::new(0., 1.))
                    .with_stops([Color::rgb(0.1, 0.3, 0.2), Color::rgb(0.1, 0.2, 0.3)]),
            )
            .foreground(
                Gradient::new_linear(Point::new(0., 0.), Point::new(0., 1.))
                    .with_stops([Color::rgb(0.5, 0.6, 0.7), Color::rgb(0.5, 0.7, 0.6)]),
            )
            .range(0.2)
            .gap(3.),
        async_repeat(
            |proxy| async move {
                let (tx, mut rx) = mpsc::channel(8);
                if let Ok(_stream) = start_audio_input(tx).map(|s| s.into_inner()) {
                    while let Some(samples) = rx.recv().await {
                        if proxy.message(samples).is_err() {
                            break;
                        }
                    }
                }
            },
            |data: &mut AppData, samples: Vec<f64>| {
                data.signal = Arc::new(samples);
            },
        ),
    )
}

fn main() {
    let app_data = AppData::new();
    Xilem::new(app_data, app_logic)
        .background_color(Color::rgb8(0x20, 0x20, 0x20))
        .run_windowed(EventLoop::with_user_event(), "Audio Input Signal".into())
        .unwrap();
}
