use iced::{Application, Settings, window};

mod app; // we'll create this next

fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: (800, 600),
            decorations: true,
            resizable: true,
            ..window::Settings::default()
        },
        ..Settings::default()
    };
    app::NineBoxApp::run(settings)
}