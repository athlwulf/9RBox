use iced::widget::{Column, Button, Text};
use iced::{Application, Command, Element, executor, Subscription, Theme};


/// The application state
pub struct NineBoxApp;

/// All the messages (events) your app can handle
#[derive(Debug, Clone)]
pub enum Message {
    ImportCsv,
}

impl Application for NineBoxApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    /// Called once when the app starts
    fn new(_flags: ()) -> (Self, Command<Message>) {
        println!("Application starting");
        (NineBoxApp, Command::none())
    }

    /// The window title
    fn title(&self) -> String {
        "9-Box Succession Planner".into()
    }

    /// Handle incoming messages (e.g. button presses)
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ImportCsv => {
                println!("Import CSV clicked");
            }
        }
        Command::none()
    }

    /// Produce the UI layout each frame
    fn view(&self) -> Element<Message> {
        println!("Rendering view");
        Column::new()
            .push(Button::new("Import CSV").on_press(Message::ImportCsv))
            .push(Text::new("Loading…"))
            .into()
    }

    /// No background subscriptions for now
    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}