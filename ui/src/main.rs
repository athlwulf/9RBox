use iced::{Application, Settings, window};
use iced::Size;

mod app; // we'll create this next

fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: Size::new(800.0, 600.0),
            decorations: true,
            resizable: true,
            ..window::Settings::default()
        },
        ..Settings::default()
    };
    app::NineBoxApp::run(settings)
}