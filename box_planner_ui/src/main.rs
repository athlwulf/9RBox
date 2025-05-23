mod app;
mod messages;
mod views;
mod widgets;

use app::App;
use iced::Application; // Required for App::run

pub fn main() -> iced::Result {
    println!("Box Planner UI starting...");
    App::run(iced::Settings {
        // Default settings are fine for now
        // You can customize window size, resizability, etc. here
        ..iced::Settings::default()
    })
}
