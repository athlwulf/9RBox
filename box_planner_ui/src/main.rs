mod app;
mod messages;
mod views;
mod widgets;
pub mod styles;

use app::App;
use iced::{Application, Size}; // Required for App::run

pub fn main() -> iced::Result {
    println!("Box Planner UI starting...");
    App::run(iced::Settings {
        window: iced::window::Settings {
            size: Size::new(2200.0, 1200.0),
            resizable: true,
            ..Default::default()
        },
        ..iced::Settings::default()
    })
}